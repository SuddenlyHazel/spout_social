use std::{str::FromStr, time::Duration};

use bytes::Bytes;
use futures::StreamExt;
use iroh::{PublicKey, SecretKey};
use iroh_base::Signature;
use iroh_docs::NamespaceId;
use iroh_gossip::{
    net::{Gossip, GossipEvent},
    proto::TopicId,
};
use serde::{Deserialize, Serialize};
use sled::Db;
use tracing::{info, warn};

use crate::{DocsClient, SpoutDoc};

const OCEAN_GOSSIP_TOPIC: &'static str = "social.spout.app.v0.1.0.ocean.gossip";
const OCEAN_DOC_KEY: &'static str = "social.spout.app.v0.1.0.ocean.doc";

#[derive(Serialize, Deserialize, Debug)]
pub enum OceanMessage {
    Ping { nonce: u64 },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OceanMessageEnvelope {
    from: PublicKey,
    data: Bytes,
    sig: Signature,
}

impl OceanMessageEnvelope {
    fn sign_and_seal(secret_key: &SecretKey, message: &OceanMessage) -> anyhow::Result<Bytes> {
        let data = serde_json::to_vec(message)?;
        let sig = secret_key.sign(&data);
        let from = secret_key.public();

        Ok(serde_json::to_vec(&OceanMessageEnvelope {
            from,
            data: data.into(),
            sig,
        })?
        .into())
    }

    fn verify_and_open(data: &Bytes) -> anyhow::Result<(PublicKey, OceanMessage)> {
        let OceanMessageEnvelope { from, data, sig } = serde_json::from_slice(&data)?;
        from.verify(&data, &sig)?;
        let message = serde_json::from_slice(&data)?;
        Ok((from, message))
    }
}

async fn create_oceans_doc(docs: &DocsClient, app_db: &Db) -> anyhow::Result<SpoutDoc> {
    info!("creating new ocreas doc");
    let doc = docs.create().await?;
    app_db.insert(OCEAN_DOC_KEY.as_bytes(), serde_json::to_vec(&doc.id())?)?;
    app_db.flush_async().await?;
    Ok(doc)
}

async fn get_or_create_oceans_doc(docs: &DocsClient, app_db: &Db) -> anyhow::Result<SpoutDoc> {
    if let Ok(Some(namespace_id)) = app_db.get(OCEAN_DOC_KEY.as_bytes()) {
        let namespace_id = serde_json::from_slice::<NamespaceId>(&namespace_id)?;
        match docs.open(namespace_id).await? {
            Some(doc) => return Ok(doc),
            None => return create_oceans_doc(docs, app_db).await,
        }
    } else {
        warn!("app db did not contain oceans doc key. this should only happen on a fresh install");
        create_oceans_doc(docs, app_db).await
    }
}

pub async fn init(
    gossip: Gossip,
    docs: DocsClient,
    app_db: Db,
    node_key: SecretKey,
) -> anyhow::Result<()> {
    tracing::info!("attempting to join ocean gossip topic");

    let _oceans_doc = get_or_create_oceans_doc(&docs, &app_db).await?;
    tracing::info!("successfully opened oceans doc");

    let topic_bytes = blake3::hash(OCEAN_GOSSIP_TOPIC.as_bytes());
    tracing::info!("topic_id {}", topic_bytes.to_hex());

    let bootstrap_peers = get_peers_for_bootstrap(&app_db)?;

    let (gossip_topic_tx, mut gossip_topic_rx) = gossip
        .subscribe_and_join(TopicId::from_bytes(topic_bytes.into()), bootstrap_peers)
        .await?
        .split();

    tracing::info!("ocean gossip topic joined");

    let _node_key = node_key.clone();

    tokio::task::spawn(async move {
        let node_key = _node_key;
        let tx = gossip_topic_tx;
        let mut timer = tokio::time::interval(Duration::from_secs(30));

        let mut counter = 0;
        loop {
            let envelope_bytes = OceanMessageEnvelope::sign_and_seal(
                &node_key,
                &OceanMessage::Ping { nonce: counter },
            )
            .expect("failed to construct ping message");
            timer.tick().await;
            let r = tx.broadcast_neighbors(envelope_bytes).await;
            tracing::info!("gossip send {r:?}");
            counter += 1;
        }
    });

    while let Some(Ok(event)) = gossip_topic_rx.next().await {
        tracing::info!("inside receiver loop?");
        match event {
            iroh_gossip::net::Event::Gossip(GossipEvent::Received(message)) => {
                if let Ok((signer, msg)) = OceanMessageEnvelope::verify_and_open(&message.content) {
                    tracing::info!("received ocean_message from {signer} content {msg:#?}");
                }
            }
            iroh_gossip::net::Event::Lagged => {
                tracing::info!("ocean gossip receiver is lagging");
            }
            iroh_gossip::net::Event::Gossip(GossipEvent::NeighborUp(key)) => {
                if let Err(e) = store_discovered_peer(&app_db, &key).await {
                    warn!(?e, warning = "failed to store ocean peer");
                }
            }
            other => {
                tracing::info!("other gossip event {other:#?}");
            }
        }
    }

    tracing::info!("ocean gossip topic down");
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct OceanPeer {
    discovered_at: i64,
    last_seen_at: i64,
}

fn get_peers_for_bootstrap(app_db: &Db) -> anyhow::Result<Vec<PublicKey>> {
    let key_str = format!("ocean_peer.discovered");

    let mut scan = app_db.scan_prefix(key_str);

    let mut results = vec![];
    while let Some(Ok((key, _))) = scan.next() {
        let key = String::from_utf8(key.to_vec())?;
        let key = key.split(".").collect::<Vec<_>>();
        if let Some(pubkey) = key.get(2) {
            let Ok(pubkey) = PublicKey::from_str(*pubkey) else {
                continue;
            };

            results.push(pubkey);
        }
        // TODO we should really _not_ just use all the previously discovered nodes here
    }

    Ok(results)
}

fn lookup_discovered_peer(app_db: &Db, key: &PublicKey) -> anyhow::Result<Option<OceanPeer>> {
    let key_str = format!("ocean_peer.discovered.{}", key);
    if let Some(peer) = app_db.get(key_str)? {
        return Ok(serde_json::from_slice(&peer)?);
    }
    Ok(None)
}

async fn store_discovered_peer(app_db: &Db, key: &PublicKey) -> anyhow::Result<()> {
    let now = chrono::Utc::now().timestamp_millis();

    let key_str = format!("ocean_peer.discovered.{}", key);

    let ocean_peer = if let Ok(Some(mut ocean_peer)) = lookup_discovered_peer(app_db, key) {
        ocean_peer.last_seen_at = now;
        ocean_peer
    } else {
        OceanPeer {
            discovered_at: now,
            last_seen_at: now,
        }
    };

    app_db.insert(key_str.as_bytes(), serde_json::to_vec(&ocean_peer)?)?;
    app_db.flush_async().await?;
    Ok(())
}

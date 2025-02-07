use std::time::Duration;

use bytes::Bytes;
use futures::StreamExt;
use iroh::{PublicKey, SecretKey};
use iroh_base::Signature;
use iroh_docs::NamespaceId;
use iroh_gossip::{
    net::{Gossip, GossipEvent},
    proto::TopicId,
};
use rinf::debug_print;
use serde::{Deserialize, Serialize};
use sled::Db;
use tracing::{info, warn};

use crate::{DocsClient, SpoutDoc};

const OCEAN_GOSSIP_TOPIC: &'static str = "social.spout.app.v0.1.0.ocean.gossip";
const OCEAN_DOC_KEY: &'static str = "social.spout.app.v0.1.0.ocean.doc";

#[derive(Serialize, Deserialize)]
pub enum OceanMessage {
    Ping,
}

#[derive(Serialize, Deserialize)]
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

pub async fn init(gossip: Gossip, docs: DocsClient, app_db: Db) -> anyhow::Result<()> {
    info!("attempting to join ocean gossip topic");

    let oceans_doc = get_or_create_oceans_doc(&docs, &app_db).await?;
    info!("successfully opened oceans doc");

    let topic_bytes = blake3::hash(OCEAN_GOSSIP_TOPIC.as_bytes());
    debug_print!("topic_id {}", topic_bytes.to_hex());

    let (mut gossip_topic_tx, mut gossip_topic_rx) = gossip
        .subscribe_and_join(
            TopicId::from_bytes(topic_bytes.into()),
            vec![
                // This is hazels macos node_id
                // PublicKey::from_str(
                //     "dab078205e0e8153862f9b8d2d612c305ae01f7391605d69c1338e90ba7bc661",
                // )
                // .expect("failed"),
            ],
        )
        .await?
        .split();

    debug_print!("ocean gossip topic joined");
    tokio::task::spawn(async move {
        let mut tx = gossip_topic_tx;
        let mut timer = tokio::time::interval(Duration::from_secs(10));

        let mut counter = 0;
        loop {
            timer.tick().await;
            let r = tx
                .broadcast(format!("Hello, World! {counter}").into())
                .await;
            debug_print!("gossip send {r:?}");
            counter += 1;
        }
    });

    debug_print!("Here?");
    while let Some(Ok(event)) = gossip_topic_rx.next().await {
        debug_print!("inside receiver loop?");
        match event {
            iroh_gossip::net::Event::Gossip(GossipEvent::Received(message)) => {
                debug_print!("got some gossip {message:?}");
            }
            iroh_gossip::net::Event::Lagged => {
                debug_print!("ocean gossip receiver is lagging");
            }
            other => {
                debug_print!("other gossip event {other:#?}");
            }
        }
    }

    debug_print!("ocean gossip topic down");
    Ok(())
}

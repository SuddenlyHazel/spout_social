#![allow(unused)]
use std::{str::FromStr, sync::Arc, time::Duration};

use anyhow::{anyhow, Context};
use iroh::{
    endpoint,
    protocol::{Router, RouterBuilder},
    Endpoint, NodeAddr, NodeId, RelayMap, RelayUrl, SecretKey,
};

use iroh_blobs::{net_protocol::Blobs, store::fs::Store, ALPN as BLOBS_ALPN};
use iroh_docs::{
    protocol::Docs, rpc::client::docs::ShareMode, store::Query, AuthorId, NamespaceId,
    ALPN as DOCS_ALPN,
};
use iroh_gossip::{net::Gossip, ALPN as GOSSIP_ALPN};
use rinf::debug_print;
use sled::Db;
use tokio::sync::{mpsc::Sender, Mutex};
use tracing::{info, instrument};

use crate::{
    app::{ocean::enter_ocean, posts, profile::profile_actors},
    app_fs,
    messages::{ProfileSignal, RequestUserProfile, UpdateUserProfile},
    models::{
        self,
        profile::{Controller, Profile},
    },
    node::{
        protocol::{client::OceanProtocolClientBuilder, node::OceanProtocol},
        OCEAN_ALPN,
    },
    SpoutDoc,
};

const SECRET_KEY: &'static str = &"NODE_SECRET_KEY";

pub async fn iroh_base(
    app_db: Db,
) -> anyhow::Result<(
    AuthorId,
    Endpoint,
    RouterBuilder,
    Blobs<Store>,
    Docs<Store>,
    Gossip,
)> {
    let data_dir = app_fs::app_data_path().await?;

    println!("{:?}", data_dir.canonicalize());

    let secret_key = if let Ok(Some(key)) = app_db.get(SECRET_KEY) {
        serde_json::from_slice(&key)?
    } else {
        let mut rng = rand::rngs::OsRng;
        let secret_key = SecretKey::generate(&mut rng);
        app_db
            .insert(SECRET_KEY, serde_json::to_vec(&secret_key)?)
            .context("Failed to store secret. Cannot continue")?;
        tracing::info!("perist key flush {:?}", app_db.flush());
        secret_key
    };

    let endpoint = Endpoint::builder()
        .secret_key(secret_key.clone())
        .discovery_n0()
        .relay_mode(iroh::RelayMode::Custom(RelayMap::from_url(
            RelayUrl::from_str("https://aps1-1.relay.iroh.network").unwrap(),
        )))
        .bind()
        .await?;

    // We initialize the Blobs protocol in-memory
    let blobs = Blobs::persistent(data_dir.clone()).await?.build(&endpoint);

    tracing::info!("addr is.. {:?}", endpoint.node_addr().await);

    let builder = Router::builder(endpoint.clone());

    // build the gossip protocol
    let gossip = Gossip::builder().spawn(builder.endpoint().clone()).await?;

    tracing::info!("build the gossip protocol");

    // build the docs protocol
    let docs = Docs::persistent(data_dir.clone())
        .spawn(&blobs, &gossip)
        .await?;

    let author = match docs.client().authors().default().await {
        Ok(author) => author,
        Err(_) => docs.client().authors().create().await?,
    };
    
    let mut router = builder
        .accept(BLOBS_ALPN, blobs.clone())
        .accept(GOSSIP_ALPN, gossip.clone())
        .accept(DOCS_ALPN, docs.clone());

    Ok((author, endpoint, router, blobs, docs, gossip))
}

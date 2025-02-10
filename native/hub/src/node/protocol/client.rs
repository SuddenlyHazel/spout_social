use anyhow::anyhow;
use iroh::{Endpoint, NodeAddr, PublicKey};
use iroh_docs::DocTicket;
use sled::Db;
use std::str::FromStr;

use crate::{
    app::{ocean::enter_ocean, posts::PostsHandle, profile::ProfilesHandle},
    node::{BOOTSTRAP_NODE_PUBKEY, OCEAN_ALPN},
    BlobsClient, DocsClient,
};

use super::{OceanEnvelope, RegisterProfileResponse};

#[derive(Clone)]
pub struct OceanProtocolClient(Endpoint);

impl OceanProtocolClient {
    pub fn new(endpoint: Endpoint) -> Self {
        OceanProtocolClient(endpoint)
    }

    pub async fn enter_ocean(
        endpoint: Endpoint,
        profiles_handle: ProfilesHandle,
        posts_handle: PostsHandle,
        app_db: Db,
        docs_client: DocsClient,
        blobs_client: BlobsClient,
    ) -> anyhow::Result<Self> {
        let ocean_client = OceanProtocolClient::new(endpoint);
        tokio::task::spawn(enter_ocean(
            ocean_client.clone(),
            profiles_handle,
            posts_handle,
            app_db,
            docs_client,
            blobs_client,
        ));
        Ok(ocean_client)
    }

    pub async fn register_profile(
        &self,
        profile_ticket: DocTicket,
        posts_ticket: DocTicket,
    ) -> anyhow::Result<DocTicket> {
        let node_addr = PublicKey::from_str(&BOOTSTRAP_NODE_PUBKEY)?;
        let node_addr = NodeAddr::new(node_addr);
        let conn = self.0.connect(node_addr, OCEAN_ALPN.as_bytes()).await?;

        let sealed = OceanEnvelope {
            msg: super::OceanMessage::RegisterProfile {
                profile_ticket,
                posts_ticket,
            },
        };

        let (mut tx, mut rx) = conn.open_bi().await?;

        tx.write_all(&serde_json::to_vec(&sealed)?).await?;
        tx.finish()?;

        let res = rx.read_to_end(1024 * 10).await?;

        let res = serde_json::from_slice::<RegisterProfileResponse>(&res)?;

        match res.0 {
            Ok(t) => Ok(t),
            Err(e) => Err(anyhow!("{e}")),
        }
    }
}

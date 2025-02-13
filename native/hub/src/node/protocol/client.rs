use anyhow::anyhow;
use iroh::{protocol::ProtocolHandler, Endpoint, NodeAddr, PublicKey};
use iroh_docs::DocTicket;
use sled::Db;
use std::str::FromStr;

use crate::{
    app::{ocean::enter_ocean, posts::PostsHandle, profile::ProfilesHandle},
    messages::Post,
    node::{BOOTSTRAP_NODE_PUBKEY, OCEAN_ALPN},
    BlobsClient, DocsClient,
};

use super::{OceanEnvelope, RegisterProfileResponse};

pub struct OceanPostRequest(pub tokio::sync::oneshot::Sender<Vec<Post>>);

pub type CommandMessage = OceanPostRequest;
pub type ProtocolCommandSender = tokio::sync::mpsc::Sender<CommandMessage>;
pub type ProtocolCommandReceiver = tokio::sync::mpsc::Receiver<CommandMessage>;

#[derive(Clone, Debug)]
pub struct OceanProtocolClient {
    endpoint: Endpoint,
    tx: ProtocolCommandSender,
}

impl OceanProtocolClient {
    pub async fn request_ocean_posts(&self) -> anyhow::Result<Vec<Post>> {
        let (tx, rx) = tokio::sync::oneshot::channel();

        self.tx.send(OceanPostRequest(tx)).await?;
        Ok(rx.await?)
    }

    pub async fn register_profile(
        &self,
        profile_ticket: DocTicket,
        posts_ticket: DocTicket,
    ) -> anyhow::Result<DocTicket> {
        let node_addr = PublicKey::from_str(&BOOTSTRAP_NODE_PUBKEY)?;
        let node_addr = NodeAddr::new(node_addr);
        let conn = self
            .endpoint
            .connect(node_addr, OCEAN_ALPN.as_bytes())
            .await?;

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

impl ProtocolHandler for OceanProtocolClient {
    fn accept(
        &self,
        conn: iroh::endpoint::Connecting,
    ) -> n0_future::future::Boxed<anyhow::Result<()>> {
        Box::pin(async { Ok(()) })
    }
}

pub struct OceanProtocolClientBuilder {
    endpoint: Endpoint,
    tx: ProtocolCommandSender,
    rx: ProtocolCommandReceiver,
}

impl OceanProtocolClientBuilder {
    pub fn new(endpoint: Endpoint) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        OceanProtocolClientBuilder { endpoint, tx, rx }
    }

    pub fn client(&self) -> OceanProtocolClient {
        OceanProtocolClient {
            endpoint: self.endpoint.clone(),
            tx: self.tx.clone(),
        }
    }

    pub async fn enter_ocean(
        self,
        profiles_handle: ProfilesHandle,
        posts_handle: PostsHandle,
        app_db: Db,
        docs_client: DocsClient,
        blobs_client: BlobsClient,
    ) -> anyhow::Result<()> {
        let client = self.client();
        let rx = self.rx;
        tokio::task::spawn(enter_ocean(
            client,
            profiles_handle,
            posts_handle,
            app_db,
            docs_client,
            blobs_client,
            rx,
        ));
        Ok(())
    }
}

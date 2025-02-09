use std::fmt::{format, Display};

use bytes::Bytes;
use futures::StreamExt;
use iroh::protocol::ProtocolHandler;
use iroh_docs::{AuthorId, DocTicket, NamespaceId};
use n0_future::future::Boxed;
use serde::{Deserialize, Serialize};

use super::OCEAN_PROFILE_KEY_BASE;

#[derive(Serialize, Deserialize)]
pub enum OceanMessage {
    RegisterProfile {
        profile_ticket: DocTicket,
        posts_ticket: DocTicket,
    },
}

#[derive(Serialize, Deserialize)]
pub struct RegisterProfileResponse(Result<DocTicket, String>);

#[derive(Serialize, Deserialize)]
pub struct OceanEnvelope {
    msg: OceanMessage,
}

#[derive(Clone)]
pub struct ProfileKey(String);
impl ProfileKey {
    pub fn new(namespace_id: &NamespaceId, handle: &String) -> Self {
        Self(format!(
            "{}.{}.{}",
            OCEAN_PROFILE_KEY_BASE, namespace_id, handle
        ))
    }
}

impl Display for ProfileKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub mod client {
    use anyhow::anyhow;
    use iroh::{Endpoint, NodeAddr, PublicKey};
    use iroh_docs::DocTicket;
    use std::str::FromStr;

    use crate::node::{BOOTSTRAP_NODE_PUBKEY, OCEAN_ALPN};

    use super::{OceanEnvelope, RegisterProfileResponse};

    #[derive(Clone)]
    pub struct OceanProtocolClient(Endpoint);

    impl OceanProtocolClient {
        pub fn new(endpoint: Endpoint) -> Self {
            OceanProtocolClient(endpoint)
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
}

pub mod node {
    use crate::models::profile::Profile;

    use anyhow::anyhow;
    use iroh_docs::{
        engine::LiveEvent::{self, ContentReady, InsertRemote},
        rpc::{client::docs::ShareMode, AddrInfoOptions},
        AuthorId, NamespaceId,
    };
    use tracing::{event, info, instrument, warn, Instrument, Level};

    use super::super::*;
    use super::*;
    use crate::{BlobsClient, DocsClient, SpoutDoc};
    use sled::Db;

    use iroh::endpoint::Connecting;

    #[derive(Debug)]
    pub struct OceanProtocol {
        app_db: Db,
        ocean_doc: SpoutDoc,
        docs_client: DocsClient,
        blobs_client: BlobsClient,
        author_id: AuthorId,
    }
    impl OceanProtocol {
        #[instrument]
        pub(crate) async fn new(
            app_db: Db,
            docs_client: DocsClient,
            blobs_client: BlobsClient,
            author_id: AuthorId,
        ) -> anyhow::Result<Self> {
            let ocean_doc = get_or_create_ocean_doc(&app_db, &docs_client).await?;
            tokio::task::spawn(
                start_profile_watcher_tasks(
                    app_db.clone(),
                    docs_client.clone(),
                    ocean_doc.clone(),
                    blobs_client.clone(),
                    author_id.clone(),
                )
            );
            tokio::task::spawn(
                start_posts_watcher_tasks(
                    app_db.clone(),
                    docs_client.clone(),
                    ocean_doc.clone(),
                    blobs_client.clone(),
                    author_id.clone(),
                )
            );
            Ok(Self {
                app_db,
                ocean_doc,
                docs_client,
                blobs_client,
                author_id,
            })
        }
    }

    pub async fn get_or_create_ocean_doc(
        app_db: &Db,
        docs_client: &DocsClient,
    ) -> anyhow::Result<SpoutDoc> {
        let doc = if let Ok(Some(v)) = app_db.get(OCEAN_APP_DB_KEY) {
            info!("found entry for oceans doc in app_db");
            let namespace_id = serde_json::from_slice(&v)?;
            info!(">> deser namespace({namespace_id}) for ocean doc");
            let doc = docs_client.open(namespace_id).await?;
            let doc = doc.ok_or_else(|| {
                anyhow!("failed to open existing ocean doc namespace({namespace_id})")
            })?;
            doc
        } else {
            info!("oceans doc was not found for node. creating one");
            let new_doc = docs_client.create().await?;
            app_db.insert(OCEAN_APP_DB_KEY, serde_json::to_vec(&new_doc.id())?)?;
            info!(
                ">> created a new ocean doc for node namespace({})",
                new_doc.id()
            );
            new_doc
        };
        Ok(doc)
    }

    impl ProtocolHandler for OceanProtocol {
        fn accept(&self, conn: Connecting) -> Boxed<anyhow::Result<()>> {
            Box::pin(node::handle_connection(
                conn,
                self.app_db.clone(),
                self.ocean_doc.clone(),
                self.docs_client.clone(),
                self.blobs_client.clone(),
                self.author_id.clone(),
            ))
        }
    }

    #[instrument(target="start_posts_watcher_tasks", fields(ocean_doc= %ocean_doc.id()))]
    async fn start_posts_watcher_tasks(
        app_db: Db,
        docs_client: DocsClient,
        ocean_doc: SpoutDoc,
        blobs_client: BlobsClient,
        author_id: AuthorId,
    ) -> anyhow::Result<()> {
        let tree = app_db.open_tree(OCEAN_PROFILE_KEY_BASE)?;

        if tree.len() == 0 {
            warn!("no ocean user post docs have been synced yet");
        }
        
        for next in tree.iter().by_ref() {
            if let Ok((namespace_id, doc_ticket)) = next {
                let namespace_id = TryInto::<[u8; 32]>::try_into(namespace_id.as_ref())
                    .map_err(|_| anyhow!("failed to convert namespace_id to [u8; 32]"));

                let Ok(namespace_id) = namespace_id else {
                    continue;
                };

                let namespace_id = NamespaceId::from(namespace_id);

                let Ok(doc_ticket) = serde_json::from_slice::<DocTicket>(&doc_ticket) else {
                    continue;
                };

                let Ok(Some(posts_doc)) = docs_client.open(namespace_id.clone()).await else {
                    continue;
                };

                let nodes = doc_ticket.nodes;

                info!("starting post_change_watcher(namespace({namespace_id}))");
                tokio::task::spawn(posts_change_watcher(
                    namespace_id,
                    posts_doc.clone(),
                    blobs_client.clone(),
                ));
                info!("post_change_watcher(namespace({}) started", namespace_id);

                let res = posts_doc.start_sync(nodes).await;
            }
        }
        Ok(())
    }

    #[instrument(target="start_profile_watcher_tasks", fields(ocean_doc= %ocean_doc.id()))]
    async fn start_profile_watcher_tasks(
        app_db: Db,
        docs_client: DocsClient,
        ocean_doc: SpoutDoc,
        blobs_client: BlobsClient,
        author_id: AuthorId,
    ) -> anyhow::Result<()> {
        let tree = app_db.open_tree(OCEAN_PROFILE_KEY_BASE)?;

        for next in tree.iter().by_ref() {
            if let Ok((namespace_id, doc_ticket)) = next {
                let namespace_id = TryInto::<[u8; 32]>::try_into(namespace_id.as_ref())
                    .map_err(|_| anyhow!("failed to convert namespace_id to [u8; 32]"));

                let Ok(namespace_id) = namespace_id else {
                    continue;
                };

                let namespace_id = NamespaceId::from(namespace_id);

                let Ok(doc_ticket) = serde_json::from_slice::<DocTicket>(&doc_ticket) else {
                    continue;
                };

                let Ok(Some(user_doc)) = docs_client.open(namespace_id.clone()).await else {
                    continue;
                };

                let nodes = doc_ticket.nodes;

                info!("starting profile_change_watcher(namespace({namespace_id}))");
                tokio::task::spawn(profile_change_watcher(
                    namespace_id,
                    user_doc.clone(),
                    ocean_doc.clone(),
                    app_db.clone(),
                    blobs_client.clone(),
                    author_id,
                ));
                info!(
                    "starting profile_change_watcher(namespace({}) started",
                    namespace_id
                );

                let res = user_doc.start_sync(nodes).await;
            }
        }
        Ok(())
    }

    pub(crate) async fn handle_connection(
        conn: Connecting,
        app_db: Db,
        ocean_doc: SpoutDoc,
        docs_client: DocsClient,
        blobs_client: BlobsClient,
        author_id: AuthorId,
    ) -> anyhow::Result<()> {
        let connection = conn.await?;
        let peer_id = connection.remote_node_id()?;

        let (mut tx, mut rx) = connection.accept_bi().await?;
        info!("accepted connection remote({})", peer_id);

        let envelope: OceanEnvelope = {
            // TODO this is a lot of data..
            let data = rx.read_to_end(1024 * 1024).await?;
            serde_json::from_slice(&data)?
        };

        info!("received message from remote({})", peer_id);

        match envelope.msg {
            OceanMessage::RegisterProfile {
                profile_ticket,
                posts_ticket,
            } => {
                info!("OceanMessage::RegisterProfile from remote({})", peer_id);

                // Store User Profile

                let id = profile_ticket.capability.id();
                let tree = app_db.open_tree(OCEAN_PROFILE_KEY_BASE)?;
                tree.insert(id.clone(), serde_json::to_vec(&profile_ticket)?)?;
                let doc = docs_client.import(profile_ticket).await?;

                let profile_change_watcher_future = profile_change_watcher(
                    id,
                    doc,
                    ocean_doc.clone(),
                    app_db.clone(),
                    blobs_client.clone(),
                    author_id.clone(),
                );

                tokio::task::spawn(async move {
                    if let Err(e) = profile_change_watcher_future.await {
                        warn!(?e, "profile_change_watcher_future returned")
                    }
                });

                // Store User Posts
                let id = posts_ticket.capability.id();
                let tree = app_db.open_tree(OCEAN_POSTS_KEY_BASE)?;
                tree.insert(id.clone(), serde_json::to_vec(&posts_ticket)?)?;
                let doc = docs_client.import(posts_ticket).await?;

                let posts_change_watcher_future = profile_change_watcher(
                    id,
                    doc,
                    ocean_doc.clone(),
                    app_db.clone(),
                    blobs_client.clone(),
                    author_id.clone(),
                );

                tokio::task::spawn(async move {
                    if let Err(e) = posts_change_watcher_future.await {
                        warn!(?e, "posts_change_watcher_future returned")
                    }
                });

                // Share back with the user the ocean doc ticket
                let ocean_doc_ticket = ocean_doc
                    .share(ShareMode::Read, AddrInfoOptions::RelayAndAddresses)
                    .await?;
                let _ = tx
                    .write_all(&serde_json::to_vec(&RegisterProfileResponse(Ok(
                        ocean_doc_ticket,
                    )))?)
                    .await?;
                tx.finish()?;

                connection.closed().await;
            }
        }

        Ok(())
    }

    pub(crate) async fn posts_change_watcher(
        namespace_id: NamespaceId,
        user_posts_doc: SpoutDoc,
        blobs_client: BlobsClient,
    ) -> anyhow::Result<()> {
        info!("starting posts_change_watcher namespace({})", namespace_id);
        let mut sub = user_posts_doc.subscribe().await?;
        while let Some(Ok(event)) = sub.next().await {
            match event {
                LiveEvent::InsertRemote { from, entry, .. } => {
                    // TODO we should be queuing these download tasks
                    info!(
                        "user {from} inserted an entry(size : {}) {}",
                        entry.content_len(),
                        String::from_utf8_lossy(entry.id().key())
                    );
                }
                ContentReady { hash } => {
                    let Ok(mut reader) = blobs_client.read(hash.clone()).await else {
                        warn!("failed to read synced bytes for hash {hash}");
                        continue;
                    };

                    let Ok(bytes) = reader.read_to_bytes().await else {
                        warn!("reader failed to read bytes for hash {hash}");
                        continue;
                    };
                }
                LiveEvent::NeighborUp(peer) => {
                    info!("Peer({peer}) up for Namespace({namespace_id})");
                }
                LiveEvent::NeighborDown(peer) => {
                    info!("Peer({peer}) down for Namespace({namespace_id})");
                }
                _ => {}
            }
        }
        Ok(())
    }

    #[instrument(target= "profile_change_watcher", fields(namespace_id = %namespace_id))]
    pub(crate) async fn profile_change_watcher(
        namespace_id: NamespaceId,
        user_doc: SpoutDoc,
        ocean_doc: SpoutDoc,
        app_db: Db,
        blobs_client: BlobsClient,
        author_id: AuthorId,
    ) -> anyhow::Result<()> {
        info!(
            "starting profile_change_watcher namespace({})",
            namespace_id
        );
        let mut sub = user_doc.subscribe().await?;

        while let Some(Ok(event)) = sub.next().await {
            match event {
                LiveEvent::NeighborUp(peer) => {
                    info!("Peer({peer}) up for Namespace({namespace_id})");
                }
                LiveEvent::NeighborDown(peer) => {
                    info!("Peer({peer}) down for Namespace({namespace_id})");
                }
                LiveEvent::InsertRemote { from, entry, .. } => {
                    // TODO we should be queuing these download tasks
                    info!(
                        "user {from} inserted an entry(size : {}) {}",
                        entry.content_len(),
                        String::from_utf8_lossy(entry.id().key())
                    );
                }
                LiveEvent::ContentReady { hash } => {
                    let Ok(mut reader) = blobs_client.read(hash.clone()).await else {
                        warn!("failed to read synced bytes for hash {hash}");
                        continue;
                    };

                    let Ok(bytes) = reader.read_to_bytes().await else {
                        warn!("reader failed to read bytes for hash {hash}");
                        continue;
                    };

                    let Ok(profile) = serde_json::from_slice::<Profile>(&bytes) else {
                        warn!("failed to deserialize profile for synced bytes {hash}");
                        continue;
                    };

                    let Ok(tree) = app_db.open_tree(DOWNLOADED_PROFILES_TREE) else {
                        warn!("failed to open tree({DOWNLOADED_PROFILES_TREE})");
                        continue;
                    };

                    let profile_key = ProfileKey::new(&namespace_id, &profile.handle);

                    match tree.get(&profile_key.to_string()) {
                        Ok(maybe_existing) => {
                            if maybe_existing.is_some() {
                                info!("overwriting existing profile({profile_key})");
                            } else {
                                info!("creating first ocean profile({profile_key})");
                            }

                            if let Err(e) = ocean_doc
                                .set_bytes(author_id, profile_key.to_string(), bytes.to_vec())
                                .await
                            {
                                warn!("failed to insert profile into ocean_doc: {e}");
                            }
                            if let Err(e) = tree.insert(profile_key.to_string(), bytes.to_vec()) {
                                warn!("failed to insert profile into tree: {e}");
                            }
                        }

                        Err(e) => {
                            warn!(
                                ?e,
                                "failed to read key from tree({DOWNLOADED_PROFILES_TREE})"
                            );
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

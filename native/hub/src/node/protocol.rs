use futures::StreamExt;
use iroh::protocol::ProtocolHandler;
use iroh_docs::DocTicket;
use n0_future::future::Boxed;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum OceanMessage {
    RegisterProfile {
        profile_ticket: DocTicket,
        posts_ticket: DocTicket,
    },
}

#[derive(Serialize, Deserialize)]
pub struct RegisterProfileResponse(Result<(), String>);

#[derive(Serialize, Deserialize)]
pub struct OceanEnvelope {
    msg: OceanMessage,
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
        ) -> anyhow::Result<()> {
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

            let res = rx.read_to_end(1024 * 10).await?;

            let res = serde_json::from_slice::<RegisterProfileResponse>(&res)?;
            if let Err(e) = res.0 {
                return Err(anyhow!("{e}"));
            }
            Ok(())
        }
    }
}

pub mod node {
    use crate::models::profile::Profile;

    use anyhow::anyhow;
    use iroh_docs::{AuthorId, NamespaceId};
    use tracing::{info, warn};

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
        pub(crate) async fn new(
            app_db: Db,
            docs_client: DocsClient,
            blobs_client: BlobsClient,
            author_id: AuthorId,
        ) -> anyhow::Result<Self> {
            let ocean_doc = get_or_create_ocean_doc(&app_db, &docs_client).await?;
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

                let id = profile_ticket.capability.id();
                let tree = app_db.open_tree(OCEAN_PROFILE_KEY_BASE)?;

                tree.insert(id.clone(), serde_json::to_vec(&profile_ticket)?)?;
                // let DocTicket { capability, nodes } = profile_ticket;
                // TODO import and set a custom download policy
                // so we're not just blindly importing a bunch of stuff
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

                // TODO Similarly, we need to subscribe to the users post document..

                let _ = tx
                    .write_all(&serde_json::to_vec(&RegisterProfileResponse(Ok(())))?)
                    .await?;
            }
        }

        Ok(())
    }

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
                iroh_docs::engine::LiveEvent::InsertRemote { from, entry, .. } => {
                    // TODO we should be queuing these download tasks
                    info!(
                        "user {from} inserted an entry(size : {}) {}",
                        entry.content_len(),
                        String::from_utf8_lossy(entry.id().key())
                    );

                    if entry.content_len() > 1024 * 1024 * 10 {
                        warn!(
                            "tisk tisk.. peer tried to sync {} bytes. Yeeting..",
                            entry.content_len()
                        );
                        user_doc.leave().await?;
                    }
                }

                iroh_docs::engine::LiveEvent::ContentReady { hash } => {
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

                    let profile_key =
                        format!("resolved-profile.{}.{}", profile.handle, namespace_id);
                    match tree.get(&profile_key) {
                        Ok(maybe_existing) => {
                            if maybe_existing.is_some() {
                                info!("overwriting existing profile({profile_key})");
                            } else {
                                info!("creating first ocean profile({profile_key})");
                            }

                            if let Err(e) = ocean_doc
                                .set_bytes(author_id, profile_key.clone(), bytes.to_vec())
                                .await
                            {
                                warn!("failed to insert profile into ocean_doc: {e}");
                            }
                            if let Err(e) = tree.insert(profile_key, bytes.to_vec()) {
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

pub mod posts {
    use std::sync::Arc;

    use chrono::{DateTime, Utc};
    use futures::StreamExt;
    use iroh_docs::{engine::LiveEvent, AuthorId, NamespaceId};
    use tokio::sync::RwLock;
    use tracing::{info, warn};

    use crate::{messages::Post, node::OCEAN_POSTS_KEY_BASE, BlobsClient, SpoutDoc};

    #[derive(Clone, Debug)]
    pub enum PostWatcherEvent {
        WatcherStarted,
        PeerUp,
        PeerDown,
        RecordInserted,
    }

    #[derive(Debug)]
    pub struct PostsChangeWatcher {
        pub namespace_id: Arc<NamespaceId>,
        pub last_active: Arc<RwLock<Option<DateTime<Utc>>>>,
        pub rx: tokio::sync::broadcast::Receiver<PostWatcherEvent>,
    }

    impl PostsChangeWatcher {
        pub fn new(
            ocean_author_id: AuthorId,
            namespace_id: NamespaceId,
            user_posts_doc: SpoutDoc,
            ocean_doc: SpoutDoc,
            blobs_client: BlobsClient,
        ) -> anyhow::Result<Self> {
            let last_active: Arc<RwLock<Option<DateTime<Utc>>>> = Default::default();

            let (tx, rx) = tokio::sync::broadcast::channel(100);

            tokio::task::spawn(posts_change_watcher(
                ocean_author_id,
                namespace_id,
                user_posts_doc,
                ocean_doc,
                blobs_client,
                last_active.clone(),
                tx,
            ));
            let namespace_id = Arc::new(namespace_id);

            Ok(Self {
                namespace_id,
                last_active,
                rx,
            })
        }
    }

    async fn posts_change_watcher(
        ocean_author_id: AuthorId,
        namespace_id: NamespaceId,
        user_posts_doc: SpoutDoc,
        ocean_doc: SpoutDoc,
        blobs_client: BlobsClient,
        last_active: Arc<RwLock<Option<DateTime<Utc>>>>,
        mut tx: tokio::sync::broadcast::Sender<PostWatcherEvent>,
    ) -> anyhow::Result<()> {
        tx.send(PostWatcherEvent::WatcherStarted)?;
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
                    last_active.write().await.replace(Utc::now());
                    tx.send(PostWatcherEvent::RecordInserted)?;
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

                    let Ok(post) = serde_json::from_slice::<Post>(&bytes) else {
                        continue;
                    };
                    let key = format!("{}.{}", OCEAN_POSTS_KEY_BASE, Utc::now().timestamp_millis());
                    let _ = ocean_doc.set_bytes(ocean_author_id, key, bytes).await;
                }
                LiveEvent::NeighborUp(peer) => {
                    info!("Peer({peer}) up for Namespace({namespace_id})");
                    last_active.write().await.replace(Utc::now());
                    tx.send(PostWatcherEvent::PeerUp)?;
                }
                LiveEvent::NeighborDown(peer) => {
                    info!("Peer({peer}) down for Namespace({namespace_id})");
                    tx.send(PostWatcherEvent::PeerDown)?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub mod profiles {
    use std::sync::Arc;

    use crate::{models::profile::Profile, node::DOWNLOADED_PROFILES_TREE};

    use iroh_docs::{
        engine::LiveEvent::{self},
        AuthorId, NamespaceId,
    };
    use tracing::{info, instrument, warn};

    use super::super::*;
    use crate::{BlobsClient, SpoutDoc};
    use sled::Db;

    #[derive(Debug, Clone)]
    pub enum ProfileWatcherEvent {
        WatcherStarted,
        PeerUp,
        PeerDown,
        RecordInserted,
    }

    #[derive(Debug)]
    pub struct ProfileChangeWatcher {
        pub namespace_id: Arc<NamespaceId>,
        pub rx: tokio::sync::broadcast::Receiver<ProfileWatcherEvent>,
    }

    impl ProfileChangeWatcher {
        pub fn start(
            namespace_id: NamespaceId,
            user_doc: SpoutDoc,
            ocean_doc: SpoutDoc,
            app_db: Db,
            blobs_client: BlobsClient,
            author_id: AuthorId,
        ) -> anyhow::Result<Self> {
            let (tx, rx) = tokio::sync::broadcast::channel(100);

            tokio::task::spawn(profile_change_watcher(
                namespace_id,
                user_doc,
                ocean_doc,
                app_db,
                blobs_client,
                author_id,
                tx,
            ));
            let namespace_id = Arc::new(namespace_id);
            Ok(Self { namespace_id, rx })
        }
    }

    #[instrument(target= "profile_change_watcher", fields(namespace_id = %namespace_id))]
    async fn profile_change_watcher(
        namespace_id: NamespaceId,
        user_doc: SpoutDoc,
        ocean_doc: SpoutDoc,
        app_db: Db,
        blobs_client: BlobsClient,
        author_id: AuthorId,
        mut tx: tokio::sync::broadcast::Sender<ProfileWatcherEvent>,
    ) -> anyhow::Result<()> {
        tx.send(ProfileWatcherEvent::WatcherStarted)?;
        info!(
            "starting profile_change_watcher namespace({})",
            namespace_id
        );
        let mut sub = user_doc.subscribe().await?;

        while let Some(Ok(event)) = sub.next().await {
            match event {
                LiveEvent::NeighborUp(peer) => {
                    info!("Peer({peer}) up for Namespace({namespace_id})");
                    tx.send(ProfileWatcherEvent::PeerUp)?;
                }
                LiveEvent::NeighborDown(peer) => {
                    info!("Peer({peer}) down for Namespace({namespace_id})");
                    tx.send(ProfileWatcherEvent::PeerDown)?;
                }
                LiveEvent::InsertRemote { from, entry, .. } => {
                    // TODO we should be queuing these download tasks
                    info!(
                        "user {from} inserted an entry(size : {}) {}",
                        entry.content_len(),
                        String::from_utf8_lossy(entry.id().key())
                    );
                    tx.send(ProfileWatcherEvent::RecordInserted)?;
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

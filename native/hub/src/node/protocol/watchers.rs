pub mod posts {
    use futures::StreamExt;
    use iroh_docs::{engine::LiveEvent, NamespaceId};
    use tracing::{info, warn};

    use crate::{BlobsClient, SpoutDoc};

    pub struct PostsChangeWatcher {}

    impl PostsChangeWatcher {
        pub fn new(
            namespace_id: NamespaceId,
            user_posts_doc: SpoutDoc,
            blobs_client: BlobsClient,
        ) -> anyhow::Result<Self> {
            tokio::task::spawn(posts_change_watcher(
                namespace_id,
                user_posts_doc,
                blobs_client,
            ));
            Ok(Self {})
        }
    }
    async fn posts_change_watcher(
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
                LiveEvent::ContentReady { hash } => {
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
}

pub mod profiles {
    use crate::{models::profile::Profile, node::DOWNLOADED_PROFILES_TREE};

    use iroh_docs::{
        engine::LiveEvent::{self},
        AuthorId, NamespaceId,
    };
    use tracing::{info, instrument, warn};

    use super::super::*;
    use crate::{BlobsClient, SpoutDoc};
    use sled::Db;

    pub struct ProfileChangeWatcher {}

    impl ProfileChangeWatcher {
        pub fn start(
            namespace_id: NamespaceId,
            user_doc: SpoutDoc,
            ocean_doc: SpoutDoc,
            app_db: Db,
            blobs_client: BlobsClient,
            author_id: AuthorId,
        ) -> anyhow::Result<Self> {
            tokio::task::spawn(profile_change_watcher(
                namespace_id,
                user_doc,
                ocean_doc,
                app_db,
                blobs_client,
                author_id,
            ));
            Ok(Self {})
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

use crate::node::protocol::watchers::{posts::PostsChangeWatcher, profiles::ProfileChangeWatcher};

use anyhow::anyhow;
use iroh_docs::{
    rpc::{client::docs::ShareMode, AddrInfoOptions},
    AuthorId, NamespaceId,
};
use tracing::{info, instrument, warn};

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
    pub async fn new(
        app_db: Db,
        docs_client: DocsClient,
        blobs_client: BlobsClient,
        author_id: AuthorId,
    ) -> anyhow::Result<Self> {
        let ocean_doc = get_or_create_ocean_doc(&app_db, &docs_client).await?;
        tokio::task::spawn(start_profile_watcher_tasks(
            app_db.clone(),
            docs_client.clone(),
            ocean_doc.clone(),
            blobs_client.clone(),
            author_id.clone(),
        ));
        tokio::task::spawn(start_posts_watcher_tasks(
            app_db.clone(),
            docs_client.clone(),
            ocean_doc.clone(),
            blobs_client.clone(),
            author_id.clone(),
        ));
        Ok(Self {
            app_db,
            ocean_doc,
            docs_client,
            blobs_client,
            author_id,
        })
    }
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
            let post_change_watcher =
                PostsChangeWatcher::new(namespace_id, posts_doc.clone(), blobs_client.clone())?;
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

            let profile_change_watcher = ProfileChangeWatcher::start(
                namespace_id,
                user_doc.clone(),
                ocean_doc.clone(),
                app_db.clone(),
                blobs_client.clone(),
                author_id,
            )?;

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

            let profile_change_watcher = ProfileChangeWatcher::start(
                id,
                doc,
                ocean_doc.clone(),
                app_db.clone(),
                blobs_client.clone(),
                author_id.clone(),
            );

            // Store User Posts
            let id = posts_ticket.capability.id();
            let tree = app_db.open_tree(OCEAN_POSTS_KEY_BASE)?;
            tree.insert(id.clone(), serde_json::to_vec(&posts_ticket)?)?;
            let doc = docs_client.import(posts_ticket).await?;

            let posts_change_watcher = PostsChangeWatcher::new(id, doc, blobs_client.clone())?;

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

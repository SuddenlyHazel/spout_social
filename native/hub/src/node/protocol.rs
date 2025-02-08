use core::task;
use std::collections::{BTreeSet, HashSet};

use futures::StreamExt;
use iroh::{endpoint::Connecting, protocol::ProtocolHandler, NodeAddr, PublicKey};
use iroh_blobs::{
    net_protocol::DownloadMode, rpc::client::blobs::DownloadOptions, util::SetTagOption::Auto,
    BlobFormat,
};
use iroh_docs::{
    rpc::client::docs::Doc, store::DownloadPolicy::NothingExcept, DocTicket, Entry, NamespaceId,
};
use n0_future::future::Boxed;
use serde::{Deserialize, Serialize};
use sled::Db;
use tracing::{info, warn};

use crate::{models::profile::Profile, BlobsClient, DocsClient, SpoutDoc};

use super::{DOWNLOADED_PROFILES_TREE, OCEAN_PROFILE_KEY_BASE};

#[derive(Serialize, Deserialize)]
pub enum OceanMessage {
    RegisterProfile { profile_ticket: DocTicket },
}

#[derive(Serialize, Deserialize)]
pub struct RegisterProfileResponse(Result<(), String>);

#[derive(Serialize, Deserialize)]
pub struct OceanEnvelope {
    msg: OceanMessage,
}

#[derive(Debug)]
pub struct OceanProtocol {
    app_db: Db,
    ocean_doc: SpoutDoc,
    docs_client: DocsClient,
    blobs_client: BlobsClient,
}

impl ProtocolHandler for OceanProtocol {
    fn accept(&self, conn: Connecting) -> Boxed<anyhow::Result<()>> {
        Box::pin(handle_connection(
            conn,
            self.app_db.clone(),
            self.docs_client.clone(),
            self.blobs_client.clone(),
        ))
    }
}

async fn handle_connection(
    conn: Connecting,
    app_db: Db,
    docs_client: DocsClient,
    blobs_client: BlobsClient,
) -> anyhow::Result<()> {
    let connection = conn.await?;

    let (mut tx, mut rx) = connection.accept_bi().await?;

    let envelope: OceanEnvelope = {
        // TODO this is a lot of data..
        let data = rx.read_to_end(1024 * 1024).await?;
        serde_json::from_slice(&data)?
    };

    match envelope.msg {
        OceanMessage::RegisterProfile { profile_ticket } => {
            let id = profile_ticket.capability.id();
            let tree = app_db.open_tree(OCEAN_PROFILE_KEY_BASE)?;

            tree.insert(id.clone(), serde_json::to_vec(&profile_ticket)?)?;
            // let DocTicket { capability, nodes } = profile_ticket;
            // TODO import and set a custom download policy
            // so we're not just blindly importing a bunch of stuff
            let doc = docs_client.import(profile_ticket).await?;
            tokio::task::spawn(profile_change_watcher(
                id,
                doc,
                app_db.clone(),
                blobs_client.clone(),
            ));
            let _ = tx
                .write_all(&serde_json::to_vec(&RegisterProfileResponse(Ok(())))?)
                .await?;
        }
    }

    Ok(())
}

async fn profile_change_watcher(
    namespace_id: NamespaceId,
    user_doc: SpoutDoc,
    app_db: Db,
    blobs_client: BlobsClient,
) -> anyhow::Result<()> {
    let mut sub = user_doc.subscribe().await?;

    while let Some(Ok(event)) = sub.next().await {
        match event {
            iroh_docs::engine::LiveEvent::InsertRemote {
                from,
                entry,
                content_status,
            } => {
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
                let profile_key = format!("{}{}", profile.handle, namespace_id);
                match tree.get(&profile_key) {
                    Ok(maybe_existing) => {
                        if maybe_existing.is_some() {
                            info!("overwriting existing profile({profile_key})");
                        } else {
                            info!("creating first ocean profile({profile_key})");
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

// TODO future version..
// async fn profile_change_watcher(
//     user_doc: SpoutDoc,
//     app_db: Db,
//     blobs_client: BlobsClient,
// ) -> anyhow::Result<()> {
//     let mut sub = user_doc.subscribe().await?;

//     while let Some(Ok(event)) = sub.next().await {
//         match event {
//             iroh_docs::engine::LiveEvent::InsertRemote {
//                 from,
//                 entry,
//                 content_status,
//             } => {
//                 // TODO we should be queuing these download tasks
//                 info!(
//                     "user {from} inserted an entry(size : {}) {}",
//                     entry.content_len(),
//                     String::from_utf8_lossy(entry.id().key())
//                 );
//                 let mut resolved_peers: BTreeSet<NodeAddr> = BTreeSet::new();
//                 resolved_peers.insert(NodeAddr::new(from));

//                 user_doc.get_sync_peers().await.and_then(|peers| {
//                     peers.map(|peers| {
//                         peers.into_iter().for_each(|peer| {
//                             resolved_peers
//                                 .insert(NodeAddr::new(PublicKey::from_bytes(&peer).unwrap()));
//                         });
//                     });
//                     Ok(())
//                 })?;

//                 let resolved_peers: Vec<_> = resolved_peers.into_iter().collect();

//                 let download_request = blobs_client
//                     .download_with_opts(
//                         entry.content_hash(),
//                         DownloadOptions {
//                             format: BlobFormat::Raw,
//                             nodes: resolved_peers,
//                             tag: Auto,
//                             mode: DownloadMode::Queued,
//                         },
//                     )
//                     .await;

//                 if let Ok(download_request) = download_request {
//                     let download_result = download_request.await;
//                 }
//             }

//             iroh_docs::engine::LiveEvent::ContentReady { hash } => {}
//             _ => {}
//         }
//     }
//     Ok(())
// }

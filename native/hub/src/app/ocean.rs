use std::str::FromStr;

use anyhow::anyhow;
use futures::StreamExt;
use iroh::{NodeAddr, PublicKey};
use iroh_docs::{store::Query, NamespaceId};
use rinf::debug_print;
use sled::Db;

use crate::{
    messages::{ProfileListRequestQuery, ProfileListResponse, ProfileSignal},
    models::profile::Profile,
    node::{protocol::client::OceanProtocolClient, BOOTSTRAP_NODE_PUBKEY, OCEAN_PROFILE_KEY_BASE},
    BlobsClient, DocsClient, SpoutDoc,
};

use super::{posts::PostsHandle, profile::ProfilesHandle};

pub const APP_OCEAN_DOC_DB_KEY: &'static str = "social.spout.app.ocean.protocol.v0.1.0";

pub async fn enter_ocean(
    ocean_client: OceanProtocolClient,
    profiles_handle: ProfilesHandle,
    posts_handle: PostsHandle,
    app_db: Db,
    docs_client: DocsClient,
    blobs_client: BlobsClient,
) -> anyhow::Result<()> {
    debug_print!("Entered the ocean task :D");

    let doc = match app_db.get(APP_OCEAN_DOC_DB_KEY)? {
        Some(namespace) => {
            debug_print!("Found existing ocean doc");

            let namespace: NamespaceId = serde_json::from_slice(&namespace)?;
            let doc = docs_client.open(namespace).await?.ok_or(anyhow!(
                "failed to open ocean doc with stored namespace({namespace})"
            ))?;
            debug_print!("loaded ocean doc from disk");
            let bootstrap_addr = NodeAddr::new(PublicKey::from_str(BOOTSTRAP_NODE_PUBKEY)?);
            doc.start_sync(vec![bootstrap_addr]).await?;
            debug_print!("started ocean doc sync");

            doc
        }
        None => {
            debug_print!("no existing ocean document found. Attempting to register with node");
            debug_print!("waiting for profile");

            let (profile_ticket, posts_ticket) = (
                profiles_handle.user_posts_ticket().await?,
                posts_handle.user_posts_ticket().await?,
            );
            let register_result = ocean_client
                .register_profile(profile_ticket, posts_ticket)
                .await;
            debug_print!("register result {register_result:?}");

            match register_result {
                Ok(ocean_ticket) => {
                    let doc = docs_client.import(ocean_ticket).await?;
                    app_db.insert(APP_OCEAN_DOC_DB_KEY, serde_json::to_vec(&doc.id())?)?;
                    doc
                }
                Err(e) => return Err(e.into()),
            }
        }
    };

    tokio::task::spawn(profile_list_actor(doc.clone(), blobs_client));

    Ok(())
}

async fn profile_list_actor(ocean_doc: SpoutDoc, blobs_client: BlobsClient) -> anyhow::Result<()> {
    let listener = ProfileListRequestQuery::get_dart_signal_receiver();

    while let Some(req) = listener.recv().await {
        let ProfileListRequestQuery { offset, amount } = req.message;
        debug_print!("Received list request!");
        let mut res = ocean_doc
            .get_many(
                Query::key_prefix(OCEAN_PROFILE_KEY_BASE)
                    .offset(offset as u64)
                    .limit(amount as u64),
            )
            .await?;
        let mut results = vec![];
        while let Some(Ok(profile)) = res.next().await {
            let content_hash = profile.content_hash();
            let Ok(mut reader) = blobs_client.read(content_hash).await else {
                continue;
            };

            let Ok(bytes) = reader.read_to_bytes().await else {
                continue;
            };

            let Ok(profile) = serde_json::from_slice::<Profile>(&bytes) else {
                continue;
            };

            results.push(ProfileSignal {
                name: profile.name,
                handle: profile.handle,
                bio: profile.bio,
                location: profile.location,
                profile_image: profile.profile_image.to_vec(),
            });
        }

        ProfileListResponse {
            count: results.len() as i64,
            profiles: results,
        }
        .send_signal_to_dart();
    }
    Ok(())
}

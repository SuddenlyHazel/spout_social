use anyhow::anyhow;
use bytes::Bytes;
use iroh::{NodeAddr, NodeId};
use iroh_blobs::Hash;
use iroh_docs::{store::Query, AuthorId, DocTicket};

use rinf::debug_print;
use serde::{Deserialize, Serialize};

use crate::{BlobsClient, DocsClient, SpoutDoc};

const PERSON_PLACEHOLDER: &[u8] = include_bytes!("../../../../assets/person.png");

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub handle: String,
    pub bio: String,
    pub location: String,
    pub profile_image: bytes::Bytes,
}

const PROFILE_KEY: &'static str = "profile";

pub struct Controller {}

#[allow(unused)]
impl Controller {
    pub async fn load_profile_from_doc(
        doc: SpoutDoc,
        blobs_client: &BlobsClient,
    ) -> anyhow::Result<Profile> {
        if let Some(entry) = doc.get_one(Query::key_exact("profile")).await? {
            let blob = blobs_client.read_to_bytes(entry.content_hash()).await?;
            return Ok(serde_json::from_slice(&blob)?);
        }
        Err(anyhow!("Profile entry was not found in document"))
    }
    
    pub async fn load_profile_from_ticket(
        docs_client: &DocsClient,
        blobs_client: &BlobsClient,
        ticket: DocTicket,
    ) -> anyhow::Result<Profile> {
        let doc = docs_client.import(ticket).await?;
        if let Some(entry) = doc.get_one(Query::key_exact("profile")).await? {
            let blob = blobs_client.read_to_bytes(entry.content_hash()).await?;
            return Ok(serde_json::from_slice(&blob)?);
        }
        Err(anyhow!("Failed to load profile document"))
    }

    pub async fn write_profile(
        doc: &SpoutDoc,
        author_id: AuthorId,
        profile: &Profile,
    ) -> anyhow::Result<Hash> {
        let hash = doc
            .set_bytes(author_id, PROFILE_KEY, serde_json::to_vec(profile)?)
            .await?;

        match doc.get_sync_peers().await {
            Ok(Some(peers)) => {
                let nodes = peers
                    .iter()
                    .map(NodeId::from_bytes)
                    .flatten()
                    .map(NodeAddr::new)
                    .collect::<Vec<_>>();
                let r = doc.start_sync(nodes.clone()).await;
                debug_print!("Profile sync result={r:?} with peers {:?}", nodes);
            }
            Ok(None) => {
                debug_print!("No peers to sync document with?");
            }
            Err(e) => {
                debug_print!("Couldn't seek profile change with peers {e}");
            }
        }

        Ok(hash)
    }

    pub async fn create_profile(
        docs_client: &DocsClient,
        author: AuthorId,
        name: String,
        handle: String,
        bio: String,
        location: String,
    ) -> anyhow::Result<(Profile, SpoutDoc)> {
        let profile_image = Bytes::from_static(PERSON_PLACEHOLDER);

        let new_doc = docs_client.create().await?;

        let profile = Profile {
            name,
            handle,
            bio,
            location,
            profile_image,
        };
        new_doc
            .set_bytes(author, PROFILE_KEY, serde_json::to_vec(&profile)?)
            .await?;
        Ok((profile, new_doc))
    }
}

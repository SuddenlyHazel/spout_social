use anyhow::anyhow;
use base64::Engine;
use iroh_blobs::Hash;
use iroh_docs::rpc::client::docs::Doc;
use iroh_docs::{store::Query, AuthorId, DocTicket};

use quic_rpc::transport::flume::FlumeConnector;
use serde::{Deserialize, Serialize};

use crate::{BlobsClient, DocsClient, SpoutDoc};

const PERSON_PLACEHOLDER: &[u8] = include_bytes!("../../../../assets/person.png");

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub handle: String,
    pub bio: String,
    pub location: String,
    pub profile_image: String,
}

const PROFILE_KEY: &'static str = "profile";

pub struct Controller {}

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
        Ok(doc
            .set_bytes(author_id, PROFILE_KEY, serde_json::to_vec(profile)?)
            .await?)
    }

    pub async fn create_profile(
        docs_client: &DocsClient,
        author: AuthorId,
        name: String,
        handle: String,
        bio: String,
        location: String,
    ) -> anyhow::Result<(Profile, SpoutDoc)> {
        let profile_image = base64::engine::general_purpose::URL_SAFE.encode(PERSON_PLACEHOLDER);

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

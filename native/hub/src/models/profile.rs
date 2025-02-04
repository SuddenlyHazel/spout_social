use anyhow::anyhow;
use iroh_docs::rpc::client::docs::Doc;
use iroh_docs::{store::Query, AuthorId, DocTicket};

use quic_rpc::transport::flume::FlumeConnector;
use serde::{Deserialize, Serialize};

use crate::{BlobsClient, DocsClient};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub handle: String,
    pub bio: String,
    pub location: String,
    pub profile_image: String,
}

pub struct Controller {}

impl Controller {
    pub async fn load_profile(
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

    pub async fn create_profile(
        doc: Doc<FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>>,
        author: AuthorId,
        name: String,
        handle: String,
        bio: String,
        location: String,
    ) -> anyhow::Result<Profile> {
        let profile = Profile {
            name,
            handle,
            bio,
            location,
            profile_image: String::new(),
        };
        doc.set_bytes(author, "profile", serde_json::to_vec(&profile)?)
            .await?;
        Ok(profile)
    }
}

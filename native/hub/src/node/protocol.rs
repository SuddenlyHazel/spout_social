use std::fmt::Display;

use futures::StreamExt;
use iroh::protocol::ProtocolHandler;
use iroh_docs::{DocTicket, NamespaceId};
use n0_future::future::Boxed;
use serde::{Deserialize, Serialize};
use sled::Db;
use tracing::info;
use anyhow::anyhow;
use crate::{node::OCEAN_APP_DB_KEY, DocsClient, SpoutDoc};

use super::OCEAN_PROFILE_KEY_BASE;

pub mod node;
pub mod client;
mod watchers;

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
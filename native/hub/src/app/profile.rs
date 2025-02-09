use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use iroh::{NodeAddr, NodeId};

use iroh_blobs::{net_protocol::Blobs, store::fs::Store};
use iroh_docs::{
    protocol::Docs,
    rpc::{client::docs::ShareMode, AddrInfoOptions},
    AuthorId, DocTicket, NamespaceId,
};
use rinf::debug_print;
use sled::Db;
use tokio::{
    sync::{mpsc::Sender, watch, Mutex},
    task::JoinHandle,
};

use crate::{
    messages::{ProfileSignal, RequestUserProfile, UpdateUserProfile},
    models::{
        self,
        profile::{Controller, Profile},
    },
    BlobsClient, DocsClient, SpoutDoc,
};

const PROFILE_DOC_KEY: &'static str = &"PROFILE_DOC_KEY";

#[derive(Clone)]
pub struct ProfilesHandle {
    profile_doc: SpoutDoc,
}

impl ProfilesHandle {
    pub async fn user_posts_ticket(&self) -> anyhow::Result<DocTicket> {
        Ok(self
            .profile_doc
            .share(ShareMode::Read, AddrInfoOptions::Relay)
            .await?)
    }
}

pub async fn profile_signals(
    docs: Docs<Store>,
    blobs: Blobs<Store>,
    author: AuthorId,
    app_db: Db,
) -> anyhow::Result<ProfilesHandle> {
    let (profile, doc) = load_profile_with_doc(&app_db, &docs, &blobs, author).await?;

    tokio::task::spawn(_profile_signals(
        docs,
        blobs,
        author,
        app_db,
        profile,
        doc.clone(),
    ));

    Ok(ProfilesHandle { profile_doc: doc })
}

async fn load_profile_with_doc(
    app_db: &Db,
    docs: &Docs<Store>,
    blobs: &Blobs<Store>,
    author: AuthorId,
) -> anyhow::Result<(Profile, SpoutDoc)> {
    let (profile, doc) = match app_db.get(PROFILE_DOC_KEY) {
        Ok(Some(existing_doc)) => {
            let namespace_id: NamespaceId = serde_json::from_slice(&existing_doc)?;
            if let Some(doc) = docs.client().open(namespace_id).await? {
                let profile =
                    Controller::load_profile_from_doc(doc.clone(), &blobs.client()).await?;
                debug_print!("Found existing profile document.. neato");
                (profile, doc)
            } else {
                return Err(anyhow!(
                    "Failed to load profile from existing doc. Datastore might be corrupted"
                ));
            }
        }
        other => {
            debug_print!("Existing profile document wasnt found. Lets create one {other:?}");

            let (profile, doc) = models::profile::Controller::create_profile(
                docs.client(),
                author.clone(),
                "New User".into(),
                "Fake Handle".into(),
                "Strange new user".into(),
                "devnull".into(),
            )
            .await?;
            debug_print!("Storing newly created document for l8tr");
            app_db
                .insert(PROFILE_DOC_KEY, serde_json::to_vec(&doc.id())?)
                .context("Failed to store profile_doc id in db")?;
            debug_print!("store profile key flush {:?}", app_db.flush());

            (profile, doc)
        }
    };
    Ok((profile, doc))
}

async fn _profile_signals(
    docs: Docs<Store>,
    blobs: Blobs<Store>,
    author: AuthorId,
    app_db: Db,
    profile: Profile,
    doc: SpoutDoc,
) -> anyhow::Result<()> {
    let ticket = doc
        .share(ShareMode::Read, iroh_docs::rpc::AddrInfoOptions::Id)
        .await?
        .to_string();

    debug_print!("profile ticket created {ticket}");

    let profile =
        models::profile::Controller::load_profile_from_doc(doc.clone(), blobs.client()).await?;

    let profile = Arc::new(Mutex::new(profile));

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let _ = tokio::task::spawn(profile_update_actor(
        profile.clone(),
        tx,
        doc.clone(),
        author.clone(),
    ));

    let _profile = profile.clone();
    // Push updates to the UI on profile edit..
    tokio::spawn(async move {
        while let Some(_) = rx.recv().await {
            let locked = _profile.lock().await;
            ProfileSignal {
                name: locked.name.clone(),
                handle: locked.handle.clone(),
                bio: locked.bio.clone(),
                location: locked.location.clone(),
                profile_image: locked.profile_image.clone().to_vec(),
            }
            .send_signal_to_dart();
            drop(locked);
        }
    });

    let listener = RequestUserProfile::get_dart_signal_receiver();

    while let Some(_) = listener.recv().await {
        debug_print!("Received profile request");
        let locked = profile.lock().await;

        ProfileSignal {
            name: locked.name.clone(),
            handle: locked.handle.clone(),
            bio: locked.bio.clone(),
            location: locked.location.clone(),
            profile_image: locked.profile_image.clone().to_vec(),
        }
        .send_signal_to_dart();
        drop(locked);
        println!("Send profile info");
    }
    println!("Profile req listener returned");

    Ok(())
}

async fn profile_update_actor(
    profile: Arc<Mutex<Profile>>,
    updated: Sender<()>,
    doc: SpoutDoc,
    author_id: AuthorId,
) {
    let listener = UpdateUserProfile::get_dart_signal_receiver();

    if let Ok(Some(peers)) = doc.get_sync_peers().await {
        let nodes = peers
            .iter()
            .map(NodeId::from_bytes)
            .flatten()
            .map(NodeAddr::new)
            .collect::<Vec<_>>();
        doc.start_sync(nodes).await;
    }
    while let Some(update_request) = listener.recv().await {
        let req = update_request.message;
        let mut locked = profile.lock().await;
        locked.name = req.name;
        locked.bio = req.bio;
        locked.handle = req.handle;
        locked.location = req.location;
        locked.profile_image = req.profile_image.into();

        Controller::write_profile(&doc, author_id, &locked)
            .await
            .expect("failed to write document");
        drop(locked);
        println!("Wrote profile updates :3");
        updated.send(()).await;
    }
}

#![allow(unused)]
use std::{str::FromStr, sync::Arc, time::Duration};

use anyhow::{anyhow, Context};
use iroh::{protocol::Router, Endpoint, NodeAddr, NodeId, RelayMap, RelayUrl, SecretKey};
use iroh_blobs::{net_protocol::Blobs, store::fs::Store, ALPN as BLOBS_ALPN};
use iroh_docs::{
    protocol::Docs, rpc::client::docs::ShareMode, AuthorId, NamespaceId, ALPN as DOCS_ALPN,
};
use iroh_gossip::{net::Gossip, ALPN as GOSSIP_ALPN};
use rinf::debug_print;
use sled::Db;
use tokio::sync::{mpsc::Sender, Mutex};
use tracing::info;

use crate::{
    app_fs,
    messages::{ProfileSignal, RequestUserProfile, UpdateUserProfile},
    models::{
        self,
        profile::{Controller, Profile},
    },
    posts, SpoutDoc,
};

const SECRET_KEY: &'static str = &"NODE_SECRET_KEY";
const PROFILE_DOC_KEY: &'static str = &"PROFILE_DOC_KEY";

pub async fn launch_iroh(app_db: Db) -> anyhow::Result<()> {
    // Create an endpoint, it allows creating and accepting
    // connections in the iroh p2p world

    let data_dir = app_fs::app_data_path().await?;

    println!("{:?}", data_dir.canonicalize());

    let secret_key = if let Ok(Some(key)) = app_db.get(SECRET_KEY) {
        serde_json::from_slice(&key)?
    } else {
        let mut rng = rand::rngs::OsRng;
        let secret_key = SecretKey::generate(&mut rng);
        app_db
            .insert(SECRET_KEY, serde_json::to_vec(&secret_key)?)
            .context("Failed to store secret. Cannot continue")?;
        debug_print!("perist key flush {:?}", app_db.flush());
        secret_key
    };

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .discovery_n0()
        .relay_mode(iroh::RelayMode::Custom(RelayMap::from_url(
            RelayUrl::from_str("https://aps1-1.relay.iroh.network").unwrap(),
        )))
        .bind()
        .await?;

    // We initialize the Blobs protocol in-memory
    let blobs = Blobs::persistent(data_dir.clone()).await?.build(&endpoint);
    
    debug_print!("addr is.. {:?}", endpoint.node_addr().await);

    let builder = Router::builder(endpoint);

    // build the gossip protocol
    let gossip = Gossip::builder().spawn(builder.endpoint().clone()).await?;
    debug_print!("build the gossip protocol");

    // build the docs protocol
    let docs = Docs::persistent(data_dir.clone())
        .spawn(&blobs, &gossip)
        .await?;

    let author = match docs.client().authors().default().await {
        Ok(author) => author,
        Err(_) => docs.client().authors().create().await?,
    };

    debug_print!("AuthorId {:?}", author);

    debug_print!("build the docs protocol");

    tokio::spawn(posts::start_actors(
        app_db.clone(),
        docs.client().to_owned(),
        blobs.client().to_owned(),
        author.clone(),
    ));

    let router = builder
        .accept(BLOBS_ALPN, blobs.clone())
        .accept(GOSSIP_ALPN, gossip)
        .accept(DOCS_ALPN, docs.clone())
        .spawn()
        .await?;

    let _ = tokio::task::spawn(profile_signals(
        docs.clone(),
        blobs.clone(),
        author.clone(),
        app_db.clone(),
    ));

    let mut timer = tokio::time::interval(Duration::from_secs(30));
    loop {
        timer.tick().await;

        rinf::debug_print!(
            "is_shutdown {} endpoint.is_closed {} endpoint.remote_info {:?}",
            router.is_shutdown(),
            router.endpoint().is_closed(),
            router.endpoint().remote_info_iter().collect::<Vec<_>>()
        );
    }

    Ok(())
}

pub async fn profile_signals(
    docs: Docs<Store>,
    blobs: Blobs<Store>,
    author: AuthorId,
    app_db: Db,
) -> anyhow::Result<()> {
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
                .context("Failed to store new profile document id in app_db")?;
            debug_print!("store profile key flush {:?}", app_db.flush());

            (profile, doc)
        }
    };

    let ticket = doc
        .share(ShareMode::Read, iroh_docs::rpc::AddrInfoOptions::Relay)
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
                profile_image: locked.profile_image.clone(),
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
            profile_image: locked.profile_image.clone(),
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
        locked.profile_image = req.profile_image;

        Controller::write_profile(&doc, author_id, &locked)
            .await
            .expect("failed to write document");
        drop(locked);
        println!("Wrote profile updates :3");
        updated.send(()).await;
    }
}

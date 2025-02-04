use std::sync::Arc;

use directories::ProjectDirs;
use futures::{lock, StreamExt};
use iroh::{protocol::Router, Endpoint};
use iroh_blobs::{
    net_protocol::Blobs, store::fs::Store, util::local_pool::LocalPool, ALPN as BLOBS_ALPN,
};
use iroh_docs::{protocol::Docs, rpc::client::docs::ShareMode, AuthorId, ALPN as DOCS_ALPN};
use iroh_gossip::{net::Gossip, ALPN as GOSSIP_ALPN};
use tokio::sync::{mpsc::Sender, Mutex};

use crate::{
    app_fs,
    messages::{profile, ProfileSignal, RequestUserProfile, UpdateUserProfile},
    models::{
        self,
        profile::{Controller, Profile},
    },
    SpoutDoc,
};

pub async fn launch_iroh() -> anyhow::Result<()> {
    // Create an endpoint, it allows creating and accepting
    // connections in the iroh p2p world

    let data_dir = app_fs::app_data_path().await?;

    println!("{:?}", data_dir.canonicalize());

    let endpoint = Endpoint::builder().discovery_n0().bind().await?;

    // We initialize the Blobs protocol in-memory
    let local_pool = LocalPool::default();
    let blobs = Blobs::persistent(data_dir.clone())
        .await?
        .build(&local_pool, &endpoint);

    println!("addr is.. {:?}", endpoint.node_addr().await);

    let builder = Router::builder(endpoint);

    // build the gossip protocol
    let gossip = Gossip::builder().spawn(builder.endpoint().clone()).await?;
    println!("build the gossip protocol");
    // build the docs protocol
    let docs = Docs::persistent(data_dir.clone())
        .spawn(&blobs, &gossip)
        .await?;

    let author = match docs.client().authors().default().await {
        Ok(author) => author,
        Err(_) => docs.client().authors().create().await?,
    };

    println!("AuthorId {:?}", author);

    println!("build the docs protocol");

    // Now we build a router that accepts blobs connections & routes them
    // to the blobs protocol.

    let _ =
        tokio::task::spawn(profile_signals(docs.clone(), blobs.clone(), author.clone())).await?;

    let router = builder
        .accept(BLOBS_ALPN, blobs.clone())
        .accept(GOSSIP_ALPN, gossip)
        .accept(DOCS_ALPN, docs.clone())
        .spawn()
        .await?;

    Ok(())
}

pub async fn profile_signals(
    docs: Docs<Store>,
    blobs: Blobs<Store>,
    author: AuthorId,
) -> anyhow::Result<()> {
    println!("No documents found.. safe to assume this is a new account");

    let (profile, profile_doc) = models::profile::Controller::create_profile(
        docs.client(),
        author.clone(),
        "New User".into(),
        "Fake Handle".into(),
        "Strange new user".into(),
        "devnull".into(),
    )
    .await?;

    let doc_ticket = profile_doc
        .share(ShareMode::Write, Default::default())
        .await?;

    let profile = models::profile::Controller::load_profile(
        docs.client(),
        blobs.client(),
        doc_ticket.clone(),
    )
    .await?;

    let profile = Arc::new(Mutex::new(profile));

    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    let _ = tokio::task::spawn(profile_update_actor(
        profile.clone(),
        tx,
        profile_doc.clone(),
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
        println!("Received profile request");
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

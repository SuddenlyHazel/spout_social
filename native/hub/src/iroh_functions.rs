use futures::StreamExt;
use iroh::{protocol::Router, Endpoint};
use iroh_blobs::{
    net_protocol::Blobs, store::mem::Store, util::local_pool::LocalPool, ALPN as BLOBS_ALPN,
};
use iroh_docs::{protocol::Docs, rpc::client::docs::ShareMode, AuthorId, ALPN as DOCS_ALPN};
use iroh_gossip::{net::Gossip, ALPN as GOSSIP_ALPN};

use crate::models;

pub async fn launch_iroh() -> anyhow::Result<()> {
    // Create an endpoint, it allows creating and accepting
    // connections in the iroh p2p world
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;

    // We initialize the Blobs protocol in-memory
    let local_pool = LocalPool::default();
    let blobs = Blobs::memory().build(&local_pool, &endpoint);

    println!("addr is.. {:?}", endpoint.node_addr().await);

    let builder = Router::builder(endpoint);

    // build the gossip protocol
    let gossip = Gossip::builder().spawn(builder.endpoint().clone()).await?;
    println!("build the gossip protocol");
    // build the docs protocol
    let docs = Docs::memory().spawn(&blobs, &gossip).await?;

    let author = docs.client().authors().create().await?;
    println!("build the docs protocol");

    let res = tokio::spawn(profile_signals(docs.clone(), blobs.clone(), author.clone())).await?;
    println!("{res:?}");
    // Now we build a router that accepts blobs connections & routes them
    // to the blobs protocol.
    let router = builder
        .accept(BLOBS_ALPN, blobs.clone())
        .accept(GOSSIP_ALPN, gossip)
        .accept(DOCS_ALPN, docs)
        .spawn()
        .await?;

    // Gracefully shut down the router
    println!("Shutting down.");
    router.shutdown().await?;
    local_pool.shutdown().await;

    Ok(())
}

pub async fn profile_signals(
    docs: Docs<Store>,
    blobs: Blobs<Store>,
    author: AuthorId,
) -> anyhow::Result<()> {
    let doc_list = docs.client().list().await?;
    let mut doc_stream = doc_list;
    println!("Attempting to query docs");
    let mut all_docs = vec![];
    while let Some(doc) = doc_stream.next().await {
        match doc {
            Ok(doc) => {
                println!("Document: {:?}", doc);
                all_docs.push(doc);
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }

    if all_docs.is_empty() {
        println!("No documents found.. safe to assume this is a new account");
        let new_doc = docs.client().create().await?;
        let doc_ticket = new_doc.share(ShareMode::Write, Default::default()).await?;

        let profile = models::profile::Controller::create_profile(
            new_doc,
            author,
            "New User".into(),
            "Fake Handle".into(),
            "Strange new user".into(),
            "devnull".into(),
        )
        .await?;

        let profile = models::profile::Controller::load_profile(
            docs.client(),
            blobs.client(),
            doc_ticket.clone(),
        )
        .await?;
      
        println!("{profile:?}");
    }
    Ok(())
}

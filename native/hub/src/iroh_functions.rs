use iroh::{protocol::Router, Endpoint};
use iroh_blobs::{net_protocol::Blobs, util::local_pool::LocalPool};

pub async fn launch_iroh() -> anyhow::Result<()> {
    // Create an endpoint, it allows creating and accepting
    // connections in the iroh p2p world
    let endpoint = Endpoint::builder().discovery_n0().bind().await?;

    // We initialize the Blobs protocol in-memory
    let local_pool = LocalPool::default();
    let blobs = Blobs::memory().build(&local_pool, &endpoint);

    println!("addr is.. {:?}", endpoint.node_addr().await);

    // Now we build a router that accepts blobs connections & routes them
    // to the blobs protocol.
    let router = Router::builder(endpoint)
        .accept(iroh_blobs::ALPN, blobs.clone())
        .spawn()
        .await?;

    // Gracefully shut down the router
    println!("Shutting down.");
    router.shutdown().await?;
    local_pool.shutdown().await;

    Ok(())
}

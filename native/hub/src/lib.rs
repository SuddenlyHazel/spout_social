//! This `hub` crate is the
//! entry point of the Rust logic.

use app::debug;
use app::posts;
use app::profile::profile_actors;
use app_db::app_db;
use app_fs::app_data_path;
use iroh::protocol::Router;
use iroh_blobs::net_protocol::Blobs;
use iroh_blobs::rpc::client::blobs::Client as _BlobsClient;
use iroh_blobs::store::fs::Store;
use iroh_docs::protocol::Docs;
use iroh_docs::rpc::client::docs::Client as _DocsClient;

use iroh_docs::rpc::client::docs::Doc;
use node::protocol::client::OceanProtocolClient;
use node::protocol::client::OceanProtocolClientBuilder;
use node::OCEAN_ALPN;
use quic_rpc::transport::flume::FlumeConnector;
use std::fs::File;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::FmtSubscriber;

rinf::write_interface!();

mod app;
pub mod app_db;
mod app_fs;
pub mod iroh_functions;
mod messages;
mod models;
pub mod node;

pub type DocsClient =
    _DocsClient<FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>>;
pub type BlobsClient =
    _BlobsClient<FlumeConnector<iroh_blobs::rpc::proto::Response, iroh_blobs::rpc::proto::Request>>;
pub type SpoutDoc =
    Doc<FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>>;

pub type SpoutDocs = Docs<Store>;
pub type SpoutBlobs = Blobs<Store>;

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    app_fs::init().await.expect("failed to init app filesystem");
    let app_dir = app_data_path().await.expect("failed to get app path");

    let path = format!("{}-app.log", chrono::Utc::now().timestamp_millis());
    let path = app_dir.join(path);

    let file = File::create(&path).expect("Failed to create log file");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file);
    let file_writer = BoxMakeWriter::new(non_blocking).and(std::io::stdout);
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        .with_writer(file_writer)
        // completes the builder.
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);
    info!("path {:?}", path.canonicalize());

    // We must hold onto an instance of router
    let (_router, _ocean_client) = start().await.expect("failed to start backend");

    // Keep the main function running until Dart shutdown.
    #[cfg(not(feature = "headless"))]
    rinf::dart_shutdown().await;
}

async fn start() -> anyhow::Result<(Router, OceanProtocolClient)> {
    let spout_db = app_db().await.expect("Failed to get AppDB");

    let (author, endpoint, mut router_builder, blobs, docs, _gossip) =
        iroh_functions::iroh_base(spout_db.clone()).await?;

    let ocean_client_builder = OceanProtocolClientBuilder::new(endpoint);
    let ocean_client = ocean_client_builder.client();

    router_builder = router_builder.accept(OCEAN_ALPN, ocean_client.clone());

    let posts_handle = posts::start_actors(
        ocean_client.clone(),
        spout_db.clone(),
        docs.client().to_owned(),
        blobs.client().to_owned(),
        author.clone(),
    )
    .await?;

    let profiles_handle = profile_actors(
        docs.clone(),
        blobs.clone(),
        author.clone(),
        spout_db.clone(),
    )
    .await?;

    let _ = ocean_client_builder
        .enter_ocean(
            profiles_handle.clone(),
            posts_handle.clone(),
            spout_db,
            docs.client().to_owned(),
            blobs.client().to_owned(),
        )
        .await?;

    let router = router_builder.spawn().await?;
    tokio::spawn(debug::current_nodes(posts_handle, profiles_handle));
    Ok((router, ocean_client))
}

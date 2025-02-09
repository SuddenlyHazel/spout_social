//! This `hub` crate is the
//! entry point of the Rust logic.

use app_db::app_db;
use app_fs::app_data_path;
use iroh_blobs::rpc::client::blobs::Client as _BlobsClient;
use iroh_docs::rpc::client::docs::Client as _DocsClient;

use iroh_docs::rpc::client::docs::Doc;
use quic_rpc::transport::flume::FlumeConnector;
use std::fs::File;
use tracing::info;
use tracing::Level;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::FmtSubscriber;

rinf::write_interface!();

mod app_db;
mod app_fs;
mod iroh_functions;
mod messages;
mod models;
mod app;
pub mod node;

pub type DocsClient =
    _DocsClient<FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>>;
pub type BlobsClient =
    _BlobsClient<FlumeConnector<iroh_blobs::rpc::proto::Response, iroh_blobs::rpc::proto::Request>>;
pub type SpoutDoc =
    Doc<FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>>;

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

    start().await.expect("failed to start backend");
    // Keep the main function running until Dart shutdown.
    #[cfg(not(feature = "headless"))]
    rinf::dart_shutdown().await;
}

pub async fn start() -> anyhow::Result<()> {
    let spout_db = app_db().await.expect("Failed to get AppDB");

    tokio::spawn(iroh_functions::launch_iroh(spout_db.clone()));
    Ok(())
}

//! This `hub` crate is the
//! entry point of the Rust logic.

use app_db::app_db;
use app_fs::app_data_path;
use iroh_blobs::rpc::client::blobs::Client as _BlobsClient;
use iroh_docs::rpc::client::docs::Client as _DocsClient;

use iroh_docs::rpc::client::docs::Doc;
use quic_rpc::transport::flume::FlumeConnector;
use std::fs::File;
use tracing::Level;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::FmtSubscriber;

mod app_db;
mod app_fs;
mod iroh_functions;
mod messages;
mod models;
mod ocean;
mod posts;
mod sample_functions;
mod tutorial_function;
// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

rinf::write_interface!();

pub type DocsClient =
    _DocsClient<FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>>;
pub type BlobsClient =
    _BlobsClient<FlumeConnector<iroh_blobs::rpc::proto::Response, iroh_blobs::rpc::proto::Request>>;
pub type SpoutDoc =
    Doc<FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>>;

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    app_fs::init().await.expect("failed to init app filesystem");
    let app_dir = app_data_path().await.expect("failed to get app path");

    let path = format!("{}-app.log", chrono::Utc::now().timestamp_millis());
    let path = app_dir.join(path);

    rinf::debug_print!("path {:?}", path.canonicalize());
    let file = File::create(path).expect("Failed to create log file");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file);
    let file_writer = BoxMakeWriter::new(non_blocking);
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        .with_writer(file_writer)
        // completes the builder.
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);

    let spout_db = app_db().await.expect("Failed to get AppDB");

    // Spawn concurrent tasks.
    // Always use non-blocking async functions like `tokio::fs::File::open`.
    // If you must use blocking code, use `tokio::task::spawn_blocking`
    // or the equivalent provided by your async library.
    tokio::spawn(sample_functions::communicate());
    tokio::spawn(tutorial_function::calculate_precious_data());
    tokio::spawn(tutorial_function::stream_amazing_number());
    tokio::spawn(iroh_functions::launch_iroh(spout_db.clone()));
    // Keep the main function running until Dart shutdown.
    rinf::dart_shutdown().await;
}

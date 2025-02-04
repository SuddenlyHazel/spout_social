//! This `hub` crate is the
//! entry point of the Rust logic.

use iroh_docs::rpc::client::docs::Client as _DocsClient;
use iroh_blobs::rpc::client::blobs::Client as _BlobsClient;

use quic_rpc::transport::flume::FlumeConnector;

mod iroh_functions;
mod messages;
mod sample_functions;
mod tutorial_function;
mod models;
// Uncomment below to target the web.
// use tokio_with_wasm::alias as tokio;

rinf::write_interface!();

pub type DocsClient = _DocsClient<FlumeConnector<iroh_docs::rpc::proto::Response, iroh_docs::rpc::proto::Request>>;
pub type BlobsClient = _BlobsClient<FlumeConnector<iroh_blobs::rpc::proto::Response, iroh_blobs::rpc::proto::Request>>;

// You can go with any async library, not just `tokio`.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Spawn concurrent tasks.
    // Always use non-blocking async functions like `tokio::fs::File::open`.
    // If you must use blocking code, use `tokio::task::spawn_blocking`
    // or the equivalent provided by your async library.
    tokio::spawn(sample_functions::communicate());
    tokio::spawn(tutorial_function::calculate_precious_data());
    tokio::spawn(tutorial_function::stream_amazing_number());
    tokio::spawn(iroh_functions::launch_iroh());

    // Keep the main function running until Dart shutdown.
    rinf::dart_shutdown().await;
}

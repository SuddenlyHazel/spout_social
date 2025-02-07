use hub::start;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = start().await.expect("Failed to start backend");
    let _ = tokio::signal::ctrl_c().await;
}

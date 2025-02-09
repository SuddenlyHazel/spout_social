use hub::{node::tui::SpoutTui, start};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
    .with(tui_logger::tracing_subscriber_layer())
    .init();
    tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();

    let terminal = ratatui::init();
    let _ = start().await.expect("Failed to start backend");

    let tui = SpoutTui {
        state: Default::default(),
        selected_tab: 0,
        is_running: true,
    };

    tui.run(terminal).await;
    ratatui::restore();
}

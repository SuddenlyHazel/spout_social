use hub::{
    app_db, iroh_functions,
    node::{protocol::node::OceanProtocol, tui::SpoutTui},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(tui_logger::tracing_subscriber_layer())
        .init();
    tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();

    let terminal = ratatui::init();
    // let _ = start().await.expect("Failed to start backend");

    let spout_db = app_db::app_db().await.expect("Failed to get AppDB");

    let (author, endpoint, mut router, blobs, docs, gossip) =
        iroh_functions::iroh_base(spout_db.clone())
            .await
            .expect("failed to init iroh base services");

    let ocean = OceanProtocol::new(
        spout_db.clone(),
        docs.client().to_owned(),
        blobs.client().to_owned(),
        author.clone(),
    )
    .await
    .expect("failed to start ocean protocol");

    router = router.accept(hub::node::OCEAN_ALPN, ocean);
    let _router = router.spawn().await.expect("failed to start iroh router");

    let tui = SpoutTui {
        state: Default::default(),
        selected_tab: 0,
        is_running: true,
    };

    let _ = tui.run(terminal).await;
    ratatui::restore();
}

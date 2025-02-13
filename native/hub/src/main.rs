use hub::{
    app_db, iroh_functions,
    node::{protocol::node::OceanProtocol, tui::SpoutTui},
};
use iroh::protocol::Router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(tui_logger::tracing_subscriber_layer())
        .init();
    tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");

    let (_router, ocean_protocol) = runtime
        .block_on(start())
        .expect("failed to start protocol layer");

    let terminal = ratatui::init();
    
    let tui = SpoutTui {
        logger_state: Default::default(),
        profiles_list_state: Default::default(),
        posts_list_state: Default::default(),

        selected_tab: 0,
        is_running: true,
        ocean_protocol,
    };

    let _ = tui.run(terminal);
    ratatui::restore();
}

async fn start() -> anyhow::Result<(Router, OceanProtocol)> {
    let spout_db = app_db::app_db().await.expect("Failed to get AppDB");

    let (author, endpoint, mut router, blobs, docs, gossip) =
        iroh_functions::iroh_base(spout_db.clone())
            .await
            .expect("failed to init iroh base services");

    let ocean_protocol = OceanProtocol::new(
        spout_db.clone(),
        docs.client().to_owned(),
        blobs.client().to_owned(),
        author.clone(),
    )
    .await
    .expect("failed to start ocean protocol");

    router = router.accept(hub::node::OCEAN_ALPN, ocean_protocol.clone());
    let router = router.spawn().await.expect("failed to start iroh router");
    Ok((router, ocean_protocol))
}

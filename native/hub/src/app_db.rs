use anyhow::Context;

use crate::app_fs;

pub struct AppDb(sled::Db);

pub async fn app_db() -> anyhow::Result<sled::Db> {
    let data_dir = app_fs::app_data_path().await?;
    let db_path = data_dir.join("spout.db");
    tracing::info!("{db_path:?} {:?}", db_path.canonicalize());
    if !db_path.exists() {
        tracing::info!("App DB doesn't exist. Attempting to create..");
    }
    let cfg = sled::Config::default();

    let db = cfg
        .flush_every_ms(Some(1000))
        .path(db_path)
        .open()
        .context("Failed to create AppDB. Thats not great..")?;
    let r = db.flush();
    tracing::info!("{r:?}");
    Ok(db)
}

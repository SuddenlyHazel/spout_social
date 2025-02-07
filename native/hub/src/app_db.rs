use anyhow::Context;
use rinf::debug_print;

use crate::app_fs;

pub struct AppDb(sled::Db);

pub async fn app_db() -> anyhow::Result<sled::Db> {
    let data_dir = app_fs::app_data_path().await?;
    let db_path = data_dir.join("spout.db");
    debug_print!("{db_path:?} {:?}", db_path.canonicalize());
    if !db_path.exists() {
        debug_print!("App DB doesn't exist. Attempting to create..");
    }
    let cfg = sled::Config::default();

    let db = cfg
        .flush_every_ms(Some(1000))
        .path(db_path)
        .open()
        .context("Failed to create AppDB. Thats not great..")?;
    let r = db.flush();
    debug_print!("{r:?}");
    Ok(db)
}

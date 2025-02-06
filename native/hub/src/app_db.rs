use anyhow::Context;
use rinf::debug_print;

use crate::app_fs;

pub async fn app_db() -> anyhow::Result<sled::Db> {
    let data_dir = app_fs::app_data_path().await?;
    let db_path = data_dir.join("spout.db");
    if !db_path.exists() {
        debug_print!("App DB doesn't exist. Attempting to create..");
    }
    Ok(sled::open(db_path).context("Failed to create AppDB. Thats not great..")?)
}

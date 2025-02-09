use std::path::PathBuf;

use directories::ProjectDirs;

pub async fn init() -> anyhow::Result<()> {
    #[cfg(feature = "headless")]
    let app_directories =
        ProjectDirs::from("social", "spout", "node").expect("failed to load project_dirs");

    #[cfg(not(feature = "headless"))]
    let app_directories =
        ProjectDirs::from("social", "spout", "app").expect("failed to load project_dirs");

    let data_dir = app_directories.data_dir();
    tracing::info!("Data dir {:?}", data_dir.canonicalize());

    if !data_dir.exists() {
        tokio::fs::create_dir_all(data_dir).await?;
    }
    Ok(())
}

pub async fn app_data_path() -> anyhow::Result<PathBuf> {
    #[cfg(feature = "headless")]
    let app_directories =
        ProjectDirs::from("social", "spout", "node").expect("failed to load project_dirs");

    #[cfg(not(feature = "headless"))]
    let app_directories =
        ProjectDirs::from("social", "spout", "app").expect("failed to load project_dirs");

    let data_dir = app_directories.data_dir().to_owned();
    Ok(data_dir)
}

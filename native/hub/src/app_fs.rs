use std::path::PathBuf;

use directories::ProjectDirs;
use rinf::debug_print;

pub async fn init() -> anyhow::Result<()> {
    let app_directories =
        ProjectDirs::from("com", "example", "sproutOne").expect("failed to load project_dirs");

    let data_dir = app_directories.data_dir();
    debug_print!("Data dir {:?}", data_dir.canonicalize());
    
    if !data_dir.exists() {
        tokio::fs::create_dir_all(data_dir).await?;
    }
    Ok(())
}

pub async fn app_data_path() -> anyhow::Result<PathBuf> {
    let app_directories = ProjectDirs::from("com", "example", "sproutOne").unwrap();

    let data_dir = app_directories.data_dir().to_owned();
    Ok(data_dir)
}

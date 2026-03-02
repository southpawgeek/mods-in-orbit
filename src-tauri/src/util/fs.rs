use eyre::Result;
use std::fs as stdfs;
use std::fs::File as stdfile;
use std::path::Path;
use tokio::{fs::File, io::AsyncWriteExt};
use url::Url;

use anyhow::Error;
use tauri::{AppHandle, Manager};

use crate::state::ManagerExt;

pub fn create_directory<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    stdfs::create_dir_all(path)?;
    Ok(())
}

pub fn create_file(path: &Path) -> Result<(), Error> {
    log::info!("attempting to create {}", path.display());
    if let Some(parent_dir) = path.parent() {
        stdfs::create_dir_all(parent_dir)?;
    }

    stdfile::create(path)?;
    Ok(())
}

pub fn delete_file(path: &Path) -> Result<(), Error> {
    log::info!("attempting to delete {}", path.display());
    if path.exists() {
        stdfs::remove_file(path)?;
    }
    Ok(())
}

pub fn move_file(src_path: &Path, dest_path: &Path) -> Result<(), Error> {
    log::info!(
        "attempting to move {} to {}",
        src_path.display(),
        dest_path.display()
    );

    if src_path.exists() {
        if let Some(parent_dir) = dest_path.parent() {
            stdfs::create_dir_all(parent_dir)?;
        }

        stdfs::rename(src_path, dest_path)?;
    }

    Ok(())
}

pub async fn download_file_async(app: AppHandle, url: Url, filepath: &Path) -> Result<(), Error> {
    log::info!("attempting to download {}", filepath.display());
    let http = app.app_state().http.clone();

    let mut response = http.get(url.as_ref()).send().await?.error_for_status()?;

    let mut dest_file = File::create(filepath).await?;

    while let Some(chunk) = response.chunk().await? {
        dest_file.write_all(&chunk).await?;
    }

    dest_file.flush().await?;

    Ok(())
}

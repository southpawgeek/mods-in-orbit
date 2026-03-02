use anyhow::{anyhow, Error};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::{borrow::Cow, vec};

use eyre::Result;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter};
use url::Url;
use uuid::Uuid;

use crate::mods::commands::check_if_mod_installed;
use crate::util::fs::{create_directory, create_file, delete_file, move_file};
use crate::{
    mods::structs::{ModEntry, Versions},
    state::ManagerExt,
    util::fs::download_file_async,
};

const GITHUB_API_URL: &str =
    "https://api.github.com/repos/ShackledMars261/mio-modlinks/commits?path=mods.json&per_page=1";
const MODS_JSON_URL: &str =
    "https://raw.githubusercontent.com/ShackledMars261/mio-modlinks/refs/heads/main/mods.json";

// change this to a user defined variable!!
pub const MIO_GAME_DIR: &str = "C:/Program Files (x86)/Steam/steamapps/common/MIO";

pub async fn update_list_task(http: &reqwest::Client) -> Result<Vec<ModEntry>> {
    let str = http
        .get(MODS_JSON_URL)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let mods: Vec<ModEntry> = serde_json::from_str(&str)?;

    Ok(mods)
}

pub async fn update_downloaded_mods_task(modlist: Vec<ModEntry>) -> Result<Vec<ModEntry>, Error> {
    let mut results: Vec<ModEntry> = Vec::new();
    for game_mod in modlist.iter() {
        let newest_version = game_mod
            .versions
            .versions
            .keys()
            .filter_map(|key| Version::parse(key).ok())
            .max()
            .ok_or(anyhow!(
                "{}",
                "error getting newest semantic version for mod"
            ))?;
        let newest_download_link = game_mod
            .versions
            .versions
            .get(&newest_version.to_string())
            .ok_or(anyhow!(
                "{}",
                "error getting download link from newest semantic version for mod"
            ))?;
        let parsed_url = Url::parse(newest_download_link)?;
        if let Some(last_segment) = parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
        {
            let filepath_str: String;
            if game_mod.id == Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")? {
                filepath_str = format!("{}/{}", MIO_GAME_DIR, last_segment);
            } else {
                filepath_str = format!("{}/mods/{}", MIO_GAME_DIR, last_segment);
            }
            let filepath = Path::new(&filepath_str);
            if filepath.exists() {
                log::info!("path found: {}", filepath_str);
                results.push(game_mod.clone());
            } else {
                log::info!("path not found: {}", filepath_str);
                continue;
            }
        } else {
            return Err(anyhow!("error getting final segment of url"));
        }
    }

    Ok(results)
}

pub fn sync_downloaded_mods(app: AppHandle) -> Result<(), Error> {
    let mut results: Vec<Uuid> = Vec::new();
    let modlist = app.app_state().lock_modlist();

    for game_mod in modlist.iter() {
        let newest_version = game_mod
            .versions
            .versions
            .keys()
            .filter_map(|key| Version::parse(key).ok())
            .max()
            .ok_or(anyhow!(
                "{}",
                "error getting newest semantic version for mod"
            ))?;
        let newest_download_link = game_mod
            .versions
            .versions
            .get(&newest_version.to_string())
            .ok_or(anyhow!(
                "{}",
                "error getting download link from newest semantic version for mod"
            ))?;
        let parsed_url = Url::parse(newest_download_link)?;
        if let Some(last_segment) = parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
        {
            let filepath_str: String;
            if game_mod.id == Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")? {
                filepath_str = format!("{}/{}", MIO_GAME_DIR, last_segment);
            } else {
                filepath_str = format!("{}/mods/{}", MIO_GAME_DIR, last_segment);
            }
            let filepath = Path::new(&filepath_str);
            if filepath.exists() {
                log::info!("path found: {}", filepath_str);
                results.push(game_mod.id);
            } else {
                log::info!("path not found: {}", filepath_str);
                continue;
            }
        } else {
            return Err(anyhow!("error getting final segment of url"));
        }
    }

    // let mut downloaded_mods = app.app_state().lock_downloaded_mods();
    // downloaded_mods.retain(|item| results.contains(&item.id));

    app.emit("sync-downloaded-mods", results)?;

    Ok(())
}

pub fn get_mod(app: AppHandle, mod_id: Uuid) -> Result<ModEntry, Error> {
    let modlist = app.app_state().lock_modlist();

    for game_mod in modlist.iter() {
        if game_mod.id == mod_id {
            return Ok(game_mod.clone());
        }
    }

    Err(anyhow!("{}", "mod not found"))
}

pub fn get_mod_filepath(target_mod: ModEntry) -> Result<PathBuf, Error> {
    let newest_version = target_mod
        .versions
        .versions
        .keys()
        .filter_map(|key| Version::parse(key).ok())
        .max()
        .ok_or(anyhow!(
            "{}",
            "error getting newest semantic version for mod"
        ))?;
    let download_url_str = target_mod
        .versions
        .versions
        .get(&newest_version.to_string())
        .ok_or(anyhow!(
            "{}",
            "error getting download link from newest semantic version for mod"
        ))?;

    let download_url = Url::parse(download_url_str)?;
    if let Some(last_segment) = download_url
        .path_segments()
        .and_then(|segments| segments.last())
    {
        let filepath_str: String;
        if (target_mod.id == Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?) {
            filepath_str = format!("{}/{}", MIO_GAME_DIR, last_segment);
        } else {
            filepath_str = format!("{}/mods/{}", MIO_GAME_DIR, last_segment);
        }
        let mut filepath = PathBuf::from(filepath_str);
        if check_mod_disabled(target_mod)? {
            filepath.set_extension("dll.disable");
        }
        return Ok(filepath);
    } else {
        return Err(anyhow!("error getting final segment of url"));
    }
}

pub fn check_mod_disabled(target_mod: ModEntry) -> Result<bool, Error> {
    let newest_version = target_mod
        .versions
        .versions
        .keys()
        .filter_map(|key| Version::parse(key).ok())
        .max()
        .ok_or(anyhow!(
            "{}",
            "error getting newest semantic version for mod"
        ))?;
    let download_url_str = target_mod
        .versions
        .versions
        .get(&newest_version.to_string())
        .ok_or(anyhow!(
            "{}",
            "error getting download link from newest semantic version for mod"
        ))?;

    let download_url = Url::parse(download_url_str)?;
    if let Some(last_segment) = download_url
        .path_segments()
        .and_then(|segments| segments.last())
    {
        let filepath_str: String;
        if (target_mod.id == Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?) {
            filepath_str = format!("{}/{}", MIO_GAME_DIR, last_segment);
        } else {
            filepath_str = format!("{}/mods/{}", MIO_GAME_DIR, last_segment);
        }
        let mut filepath = PathBuf::from(filepath_str);
        filepath.set_extension("dll.disable");
        return Ok(filepath.exists());
    } else {
        return Err(anyhow!("error getting final segment of url"));
    }
}

pub fn check_mod_enabled(target_mod: ModEntry) -> Result<bool, Error> {
    Ok(!(check_mod_disabled(target_mod)?))
}

pub async fn install_mod(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
    // sync_downloaded_mods(app.clone())?;
    let target_mod = get_mod(app.clone(), mod_id)?;

    let is_installed = check_mod_installed(app.clone(), mod_id).await?;
    if is_installed {
        // sync_downloaded_mods(app.clone())?;
        return Ok(false);
    }

    if !(target_mod.id == Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?) {
        Box::pin(install_mod(
            app.clone(),
            Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?,
        ))
        .await?;

        if !check_mod_enabled_from_handle(
            app.clone(),
            Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?,
        )
        .await?
        {
            enable_mod(
                app.clone(),
                Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?,
            )
            .await?;
        }
    }

    ensure_dependencies_installed(app.clone(), mod_id).await?;
    ensure_dependencies_enabled(app.clone(), mod_id).await?;

    // app.emit("download-started", target_mod.clone())?;

    let newest_version = target_mod
        .versions
        .versions
        .keys()
        .filter_map(|key| Version::parse(key).ok())
        .max()
        .ok_or(anyhow!(
            "{}",
            "error getting newest semantic version for mod"
        ))?;
    let download_url_str = target_mod
        .versions
        .versions
        .get(&newest_version.to_string())
        .ok_or(anyhow!(
            "{}",
            "error getting download link from newest semantic version for mod"
        ))?;

    let download_url = Url::parse(download_url_str)?;
    if let Some(last_segment) = download_url
        .path_segments()
        .and_then(|segments| segments.last())
    {
        let filepath_str: String;
        if (target_mod.id == Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?) {
            filepath_str = format!("{}/{}", MIO_GAME_DIR, last_segment);
            create_directory(Path::new(&format!("{}/mods", MIO_GAME_DIR)))?;
            create_directory(Path::new(&format!("{}/modconfig", MIO_GAME_DIR)))?;
        } else {
            filepath_str = format!("{}/mods/{}", MIO_GAME_DIR, last_segment);
        }
        let filepath = Path::new(&filepath_str);
        download_file_async(app.clone(), download_url, filepath).await?;

        for file_path in target_mod.config_files.iter() {
            log::info!("{}", format!("{}/modconfig/{}", MIO_GAME_DIR, file_path));
            create_file(Path::new(&format!(
                "{}/modconfig/{}",
                MIO_GAME_DIR, file_path
            )))?;
        }

        // let mut downloaded_mods_guard = app.app_state().lock_downloaded_mods();
        // downloaded_mods_guard.push(target_mod.clone());

        // app.emit("download-finished", target_mod)?;

        // sync_downloaded_mods(app.clone())?;
        Ok(true)
    } else {
        // sync_downloaded_mods(app.clone())?;
        return Err(anyhow!("error getting final segment of url"));
    }
}

pub async fn uninstall_mod(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
    // sync_downloaded_mods(app.clone())?;
    let target_mod = get_mod(app.clone(), mod_id)?;

    let is_installed = check_mod_installed(app.clone(), mod_id).await?;
    if !is_installed {
        // sync_downloaded_mods(app.clone())?;
        return Ok(false);
    }

    ensure_dependants_disabled(app.clone(), mod_id).await?;

    let dependent_mod_ids: Vec<Uuid> = {
        let modlist = app.app_state().lock_modlist();

        modlist
            .iter()
            .filter(|game_mod| game_mod.dependencies.contains(&mod_id))
            .map(|game_mod| game_mod.id)
            .collect()
    };

    for dep_id in dependent_mod_ids {
        Box::pin(uninstall_mod(app.clone(), dep_id)).await?;
    }

    // app.emit("uninstall-started", target_mod.clone())?;

    let filepath = get_mod_filepath(target_mod)?;
    delete_file(&filepath)?;

    // let mut downloaded_mods_guard = app.app_state().lock_downloaded_mods();
    // downloaded_mods_guard.retain(|item| item.id != mod_id);

    // app.emit("uninstall-finished", target_mod)?;

    // sync_downloaded_mods(app.clone())?;
    Ok(true)
}

// pub async fn check_mod_installed(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
// let mut result = false;
// let mods = app.app_state().lock_downloaded_mods().clone();
// for game_mod in mods.iter() {
// if game_mod.id == mod_id {
// result = true;
// }
// }
// Ok(result)
// }

pub async fn check_mod_installed(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
    let target_mod = get_mod(app, mod_id)?;
    let mut filepath = get_mod_filepath(target_mod)?;

    if filepath.exists() {
        return Ok(true);
    } else {
        filepath.set_extension("dll.disable");
        if filepath.exists() {
            return Ok(true);
        }
        return Ok(false);
    }
}

pub fn check_mod_installed_sync(
    downloaded_mods: Vec<ModEntry>,
    mod_id: Uuid,
) -> Result<bool, Error> {
    for game_mod in downloaded_mods {
        if game_mod.id == mod_id {
            return Ok(true);
        }
    }

    return Ok(false);
}

pub async fn ensure_dependencies_installed(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
    if !(mod_id == Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?) {
        if !(check_mod_installed(
            app.clone(),
            Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?,
        )
        .await?)
        {
            Box::pin(install_mod(
                app.clone(),
                Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?,
            ))
            .await?;
        }
    }

    let target_mod = get_mod(app.clone(), mod_id)?;
    let mut result = false;

    for mod_id in target_mod.dependencies.iter() {
        if !(check_mod_installed(app.clone(), mod_id.clone()).await?) {
            Box::pin(install_mod(app.clone(), mod_id.clone())).await?;
            result = true;
        }
    }

    Ok(result)
}

pub async fn ensure_dependencies_enabled(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
    if !(mod_id == Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?) {
        if !(check_mod_enabled_from_handle(
            app.clone(),
            Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?,
        )
        .await?)
        {
            Box::pin(enable_mod(
                app.clone(),
                Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?,
            ))
            .await?;
        }
    }

    let target_mod = get_mod(app.clone(), mod_id)?;
    let mut result = false;

    for mod_id in target_mod.dependencies.iter() {
        if !(check_mod_enabled(target_mod.clone())?) {
            Box::pin(enable_mod(app.clone(), mod_id.clone())).await?;
            result = true;
        }
    }

    Ok(result)
}

pub async fn ensure_dependants_disabled(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
    let dependents = get_dependants(app.clone(), mod_id).await?;
    let mut result = false;

    for id in dependents.iter() {
        let curr_mod = get_mod(app.clone(), *id)?;
        if check_mod_enabled(curr_mod)? {
            Box::pin(disable_mod(app.clone(), *id)).await?;
            result = true;
        }
    }

    Ok(result)
}

pub async fn get_dependants(app: AppHandle, mod_id: Uuid) -> Result<Vec<Uuid>, Error> {
    let mods = app.app_state().lock_modlist();

    let mut results: Vec<Uuid> = Vec::new();

    for game_mod in mods.iter() {
        if (game_mod.id == Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?) {
            continue;
        }
        if (game_mod.dependencies.contains(&mod_id)
            || mod_id == Uuid::parse_str("4aa269de-c271-404a-8d5e-b107fe6e5898")?)
        {
            results.push(game_mod.id);
        }
    }

    Ok(results)
}

pub async fn disable_mod(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
    if check_mod_disabled_from_handle(app.clone(), mod_id).await? {
        return Ok(false);
    }

    ensure_dependants_disabled(app.clone(), mod_id).await?;

    let target_mod = get_mod(app, mod_id)?;

    let src_filepath = get_mod_filepath(target_mod)?;
    let mut dest_filepath = src_filepath.clone();

    dest_filepath.set_extension("dll.disable");

    move_file(&src_filepath, &dest_filepath)?;

    Ok(true)
}

pub async fn enable_mod(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
    let target_mod = get_mod(app.clone(), mod_id)?;

    if check_mod_enabled(target_mod.clone())? {
        return Ok(false);
    }

    Box::pin(ensure_dependencies_installed(app.clone(), mod_id)).await?;
    Box::pin(ensure_dependencies_enabled(app.clone(), mod_id)).await?;

    let src_filepath = get_mod_filepath(target_mod)?;
    let parent_dir = src_filepath
        .parent()
        .ok_or(anyhow!("error getting parent directory"))?;

    let stem = src_filepath
        .file_stem()
        .ok_or(anyhow!("error getting file stem"))?;
    let new_filename = OsString::from(stem);

    let mut dest_filepath = parent_dir.to_path_buf();
    dest_filepath.push(new_filename);

    move_file(&src_filepath, &dest_filepath)?;

    Ok(true)
}

pub async fn check_mod_enabled_from_handle(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
    let target_mod = get_mod(app, mod_id)?;
    Ok(check_mod_enabled(target_mod)?)
}

pub async fn check_mod_disabled_from_handle(app: AppHandle, mod_id: Uuid) -> Result<bool, Error> {
    let target_mod = get_mod(app, mod_id)?;
    Ok(check_mod_disabled(target_mod)?)
}

pub async fn toggle_mod(app: AppHandle, mod_id: Uuid) -> Result<(), Error> {
    if check_mod_enabled_from_handle(app.clone(), mod_id).await? {
        disable_mod(app, mod_id).await?;
    } else {
        enable_mod(app, mod_id).await?;
    }

    Ok(())
}

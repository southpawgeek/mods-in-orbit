use anyhow::{anyhow, Error};
use semver::Version;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

use crate::{
    mods::{
        modlist::{
            check_mod_disabled_from_handle, check_mod_enabled_from_handle, check_mod_installed,
            disable_mod as disable_mod_from_uuid, enable_mod as enable_mod_from_uuid,
            get_mod as get_mod_from_uuid, install_mod, toggle_mod as toggle_mod_enabled,
            uninstall_mod,
        },
        structs::ModEntry,
    },
    state::ManagerExt,
};

#[tauri::command]
pub fn get_mods(app: AppHandle) -> Vec<ModEntry> {
    app.app_state().lock_modlist().clone()
}

#[tauri::command]
pub async fn check_if_mod_installed(app: AppHandle, mod_id: Uuid) -> bool {
    check_mod_installed(app, mod_id).await.unwrap()
}

#[tauri::command]
pub async fn get_newest_mod_version(app: AppHandle, mod_id: Uuid) -> String {
    let mods = app.app_state().lock_modlist().clone();
    for game_mod in mods {
        if game_mod.id == mod_id {
            let newest_version = game_mod
                .versions
                .versions
                .keys()
                .filter_map(|key| Version::parse(key).ok())
                .max()
                .ok_or(anyhow!(
                    "{}",
                    "error getting newest semantic version for mod"
                ))
                .expect("error getting newest mod version");
            return newest_version.to_string();
        }
    }
    return "".to_string();
}

#[tauri::command]
pub async fn trigger_install_mod(app: AppHandle, mod_id: Uuid) -> bool {
    install_mod(app, mod_id).await.unwrap()
}

#[tauri::command]
pub async fn trigger_uninstall_mod(app: AppHandle, mod_id: Uuid) -> bool {
    uninstall_mod(app, mod_id).await.unwrap()
}

#[tauri::command]
pub async fn get_mod(app: AppHandle, mod_id: Uuid) -> ModEntry {
    get_mod_from_uuid(app, mod_id).unwrap()
}

#[tauri::command]
pub async fn enable_mod(app: AppHandle, mod_id: Uuid) -> bool {
    enable_mod_from_uuid(app, mod_id).await.unwrap()
}

#[tauri::command]
pub async fn disable_mod(app: AppHandle, mod_id: Uuid) -> bool {
    disable_mod_from_uuid(app, mod_id).await.unwrap()
}

#[tauri::command]
pub async fn check_if_mod_enabled(app: AppHandle, mod_id: Uuid) -> bool {
    check_mod_enabled_from_handle(app, mod_id).await.unwrap()
}

#[tauri::command]
pub async fn check_if_mod_disabled(app: AppHandle, mod_id: Uuid) -> bool {
    check_mod_disabled_from_handle(app, mod_id).await.unwrap()
}

#[tauri::command]
pub async fn toggle_mod(app: AppHandle, mod_id: Uuid) -> () {
    toggle_mod_enabled(app, mod_id).await.unwrap()
}

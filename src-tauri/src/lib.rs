use crate::{
    mods::{modlist::check_mod_installed_sync, structs::ModEntry},
    state::ManagerExt,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tauri::{AppHandle, Emitter, Listener};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_store::StoreExt;
use tokio::runtime::Handle;
use uuid::Uuid;

mod commands;
mod mods;
mod state;
mod util;

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    log::info!(
        "ModsInOrbit v{} running on {}",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
    );

    if let Err(err) = state::setup(app.handle()) {
        log::error!("setup error: {:?}", err);

        app.dialog()
            .message(format!("Failed to launch ModsInOrbit: {err:?}"))
            .blocking_show();

        return Err(err.into());
    }

    // let downloaded_mods_list = app.app_state().lock_downloaded_mods().clone();
    //
    // #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    // #[serde(rename_all = "camelCase")]
    // struct CheckModInstalledResponse {
    // pub id: String,
    // pub is_installed: bool,
    // }
    //
    // let app_handle = app.handle().clone();
    //
    // app.listen("check-mod-installed", move |event| {
    // let app_handle = app_handle.clone();
    //
    // if let Ok(payload) = serde_json::from_str::<ModEntry>(&event.payload()) {
    // let is_installed = check_mod_installed_sync(downloaded_mods_list.clone(), payload.id)
    // .expect("error checking if mod is installed");
    //
    // let resp = CheckModInstalledResponse {
    // id: payload.id.to_string(),
    // is_installed,
    // };
    //
    // app_handle
    // .emit("check-mod-installed-response", resp)
    // .unwrap();
    // }
    // });

    log::info!("setup done");

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Trace)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        // .setup(|app| {
        // let store = app.store("store.json")?;
        //
        // store.close_resource();
        //
        // Ok(())
        // })
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::update_local_modlist,
            commands::download_mod,
            mods::commands::get_mods,
            mods::commands::trigger_install_mod,
            mods::commands::trigger_uninstall_mod,
            mods::commands::check_if_mod_installed,
            mods::commands::get_newest_mod_version,
            mods::commands::get_mod,
            mods::commands::enable_mod,
            mods::commands::disable_mod,
            mods::commands::check_if_mod_enabled,
            mods::commands::check_if_mod_disabled,
            mods::commands::toggle_mod,
            util::launch::launch_game,
        ])
        .setup(setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

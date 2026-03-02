use crate::mods::modlist;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub async fn update_local_modlist() {
    log::info!("updating the local modlist from local file");
    // let result = modlist::get_cached_games();
    // match result {
    //     Ok(mods_cache) => {
    //         for mod_entry in &mods_cache {
    //             log::info!("Mod found! \"{}\"", mod_entry.display_title);
    //         }
    //     }
    //     Err(error) => {}
    // }
}

#[tauri::command]
pub async fn download_mod(id: String) {}

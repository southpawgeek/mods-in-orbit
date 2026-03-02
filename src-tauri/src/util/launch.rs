use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub async fn launch_game(app: AppHandle) -> () {
    app.opener()
        .open_url("steam://run/1672810", None::<&str>)
        .unwrap();
}

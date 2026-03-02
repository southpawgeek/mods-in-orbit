use serde_json::Value;
use tauri::{utils::config::AppConfig, AppHandle};
use tauri_plugin_store::StoreExt;

type Error = Box<dyn std::error::Error>;

pub enum ConfigStore {
    AppConfig,
}

pub struct Store {}

fn get_store_str(store: ConfigStore) -> &'static str {
    match store {
        ConfigStore::AppConfig => "app.conf.json",
        _ => "app.conf.json",
    }
}

pub fn get_store_value(
    app_handle: &AppHandle,
    store: ConfigStore,
    key: &str,
) -> Result<Value, Error> {
    let store = app_handle.store(get_store_str(store))?;
    let value = store.get(key).expect("Failed to get value from store");

    Ok(value)
}

pub fn set_store_value(
    app_handle: &AppHandle,
    store: ConfigStore,
    key: &str,
    value: Value,
) -> Result<(), Error> {
    let store = app_handle.store(get_store_str(store))?;
    store.set(key, value);

    Ok(())
}

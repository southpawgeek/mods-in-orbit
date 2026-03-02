use std::sync::{Mutex, MutexGuard};

use eyre::{Context, Result};
use tauri::{path::BaseDirectory, AppHandle, Manager};
use tokio::runtime::Runtime;

use crate::{
    mods::modlist::{update_downloaded_mods_task, update_list_task},
    mods::structs::ModEntry,
    util::store::Store,
};

pub struct AppState {
    pub http: reqwest::Client,
    pub modlist: Mutex<Vec<ModEntry>>,
    // pub downloaded_mods: Mutex<Vec<ModEntry>>,
}

impl AppState {
    pub fn lock_modlist(&self) -> MutexGuard<'_, Vec<ModEntry>> {
        self.modlist.lock().unwrap()
    }

    // pub fn lock_downloaded_mods(&self) -> MutexGuard<'_, Vec<ModEntry>> {
    // self.downloaded_mods.lock().unwrap()
    // }
}

pub fn setup(app: &AppHandle) -> Result<()> {
    let http = reqwest::Client::builder()
        .user_agent(concat!("ModsInOrbit/", env!("CARGO_PKG_VERSION")))
        .build()
        .context("failed to init http client")?;

    let rt = Runtime::new().expect("Failed to create Tokio runtime");
    let modlist = rt.block_on(update_list_task(&http))?;

    // let rt = Runtime::new().expect("Failed to create Tokio runtime");
    // let downloaded_mods = rt
    // .block_on(update_downloaded_mods_task(modlist.clone()))
    // .expect("error getting downloaded mods");

    let state = AppState {
        http,
        modlist: Mutex::new(modlist),
        // downloaded_mods: Mutex::new(downloaded_mods),
    };

    app.manage(state);

    Ok(())
}

pub trait ManagerExt<R> {
    fn app_state(&self) -> &AppState;

    fn http(&self) -> &reqwest::Client {
        &self.app_state().http
    }

    fn lock_modlist(&self) -> MutexGuard<'_, Vec<ModEntry>> {
        self.app_state().lock_modlist()
    }

    // fn lock_downloaded_mods(&self) -> MutexGuard<'_, Vec<ModEntry>> {
    // self.app_state().lock_downloaded_mods()
    // }
}

impl<T, R> ManagerExt<R> for T
where
    T: tauri::Manager<R>,
    R: tauri::Runtime,
{
    fn app_state(&self) -> &AppState {
        self.state::<AppState>().inner()
    }
}

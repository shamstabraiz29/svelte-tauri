/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the account-client APIs.
use tauri::{Manager, Runtime};

#[cfg(desktop)]
use crate::desktop::account_client::AccountClient;

pub trait AccountClientExt<R: Runtime> {
    fn account_client(&self) -> &AccountClient<R>;
}

impl<R: Runtime, T: Manager<R>> AccountClientExt<R> for T {
    fn account_client(&self) -> &AccountClient<R> {
        self.state::<AccountClient<R>>().inner()
    }
}

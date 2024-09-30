/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the repo-client APIs.
use tauri::{Manager, Runtime};

#[cfg(desktop)]
use crate::desktop::editor_client::EditorClient;

pub trait RepoClientExt<R: Runtime> {
    fn repo_client(&self) -> &EditorClient<R>;
}

impl<R: Runtime, T: Manager<R>> RepoClientExt<R> for T {
    fn repo_client(&self) -> &EditorClient<R> {
        self.state::<EditorClient<R>>().inner()
    }
}

use tauri::{AppHandle, Runtime};

pub struct EditorClient<R: Runtime>(pub AppHandle<R>);

use serde::Serialize;
use specta::Type;

#[derive(Serialize, Clone, Debug, Type, tauri_specta::Event, Default)]
pub struct CommitModelChangesEvent;

// tauri-plugin-editor/rust/src/lib.rs
pub mod commands;
mod config;
mod error;
pub mod ext;
pub mod lifecycle;
pub mod models;
mod reorder;

#[cfg(desktop)]
pub mod desktop;

pub mod types;

pub use error::{EditorError, Result};

pub const PLUGIN_NAME: &str = "editor";

#[macro_export]
macro_rules! specta_builder_wry {
    () => {
        tauri_specta::Builder::<tauri::Wry>::new()
            .plugin_name($crate::PLUGIN_NAME)
            .events(tauri_specta::collect_events![
                $crate::models::branch_detail_event::BranchDetailEvent,
                frontend_tauri_plugins_common::events::commit_info_event::CommitInfoEvent,
                $crate::models::cloud_patterns_meta_event::CloudPatternsMetaEvent,
                $crate::models::branch_detail_event::commit_model_changes::CommitModelChangesEvent,
                $crate::models::branch_detail_event::upserted_viewports::UpsertedViewportsEvent,
                $crate::models::branch_detail_event::removed_viewports::RemovedViewportsEvent,
                $crate::models::branch_detail_event::upserted_viewport_items::UpsertedViewportItemsEvent,
                $crate::models::branch_detail_event::removed_viewport_items::RemovedViewportItemsEvent
            ])
            .commands(tauri_specta::collect_commands![
                $crate::commands::evaluate_cloud_pattern::<tauri::Wry>,
                $crate::commands::get_cloud_patterns,
                $crate::commands::load_model::<tauri::Wry>,
                $crate::commands::unload_model,
                $crate::commands::custom_cursor,
                $crate::commands::viewport_create::<tauri::Wry>,
                $crate::commands::viewport_upsert_properties::<tauri::Wry>,
                $crate::commands::viewport_remove_properties::<tauri::Wry>,
                $crate::commands::viewport_remove::<tauri::Wry>,
                $crate::commands::viewport_item_upsert_properties::<tauri::Wry>,
                $crate::commands::viewport_item_remove_properties::<tauri::Wry>,
                $crate::commands::viewport_item_remove::<tauri::Wry>,
                $crate::commands::model_node_upsert_properties::<tauri::Wry>,
                $crate::commands::model_node_remove_properties::<tauri::Wry>,
                $crate::commands::save_updates::<tauri::Wry>,
                $crate::commands::set_test_model::<tauri::Wry>,
                $crate::commands::get_resource_item_viewport_data::<tauri::Wry>,
                $crate::commands::get_resource_items_viewport_data::<tauri::Wry>,
                $crate::commands::get_resource_item_schema::<tauri::Wry>,
                $crate::commands::get_resource_item_partials::<tauri::Wry>,
                $crate::commands::get_resource_items_partials::<tauri::Wry>
            ])
    };
}

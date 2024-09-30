use common_dtos::editor_client::types::cloud_pattern::CategorizedCloudPatterns;

use frontend_tauri_plugins_common::notifying_sync::{
    notify_rw_lock::NotifyingRwLock, notifying_state::NotifyingState,
};
use tauri::{AppHandle, Runtime};

use tauri_specta::Event;

use crate::models::cloud_patterns_meta_event::CloudPatternsMetaEvent;

#[derive(Debug)]
pub struct CloudPatternsMetaState {
    cloud_patterns_meta: NotifyingRwLock<Option<CategorizedCloudPatterns>>,
}

impl CloudPatternsMetaState {
    pub fn new<R: Runtime>(app: AppHandle<R>) -> Self {
        let app_handle_write_callback = app.clone();
        let app_handle_read_callback = app.clone();

        let write_callback = Box::new(
            move |cloud_patterns_meta: &Option<CategorizedCloudPatterns>| {
                // This callback now captures app_clone and can use it to emit events
                let cps_meta_event = from(cloud_patterns_meta.clone());
                log::trace!("Emitted CloudPatterns' meta: {:?}", cps_meta_event);
                cps_meta_event
                    .emit(&app_handle_write_callback)
                    .expect("Failed to emit CloudPatterns' meta");
            },
        );

        let read_callback = Box::new(
            move |cloud_patterns_meta: &Option<CategorizedCloudPatterns>| {
                // This callback now captures app_clone and can use it to emit events
                let acct_event = from(cloud_patterns_meta.clone());
                if let CloudPatternsMetaEvent::Clear = acct_event {
                    return;
                }
                log::trace!("Emitted CloudPatterns' meta: {:?}", acct_event);
                acct_event
                    .emit(&app_handle_read_callback)
                    .expect("Failed to emit CloudPatterns' meta");
            },
        );

        Self {
            cloud_patterns_meta: NotifyingRwLock::new(None, write_callback, Some(read_callback)),
        }
    }
}

impl NotifyingState for CloudPatternsMetaState {
    type Dto = CategorizedCloudPatterns;

    fn get_notifying_lock(&self) -> &NotifyingRwLock<Option<Self::Dto>> {
        &self.cloud_patterns_meta
    }
}

fn from(categorized_cloud_patterns: Option<CategorizedCloudPatterns>) -> CloudPatternsMetaEvent {
    log::trace!("from - : {:?}", categorized_cloud_patterns);

    match categorized_cloud_patterns {
        Some(categorized_cloud_patterns) => CloudPatternsMetaEvent::Upsert {
            cloud_patterns_meta: categorized_cloud_patterns
                .into_iter()
                .map(|(k, xs)| (k, xs.into_iter().map(|v| v.into()).collect()))
                .collect(),
        },
        None => CloudPatternsMetaEvent::Clear,
    }
}

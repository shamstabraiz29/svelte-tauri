use crate::*;
use common_dtos::commit_client::types::branch::branch_dto::BranchAgDto;
use common_libs_editor_api_client::EditorApiClient;

use frontend_tauri_plugins_common::states::ag_commit_state::AgCommitState;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Wry,
};

use log::{debug, error, info};
use std::sync::Arc;
use types::{
    node_component_state::NodeComponentState,
    resource_items_partials_cache::ResourceItemsPartialsCache,
    resource_items_schema_cache::ResourceItemsSchemasCache,
    resource_items_viewport_data_cache::ResourceItemsViewportDataCache,
};

use crate::{
    config, desktop,
    reorder::{
        branch_ag_dto_subject::BranchAgDtoSubject, head_nmg_process::HeadNmgProcessSubject,
        model_changes_emitter_process::ModelChangesEmitterProcess, subject::Subject,
    },
    specta_builder_wry,
    types::{
        cloud_pattern_evaluator_state::CloudPatternEvaluatorState,
        cloud_patterns_meta_state::CloudPatternsMetaState,
    },
    PLUGIN_NAME,
};

/// Initializes the plugin.
pub fn init() -> TauriPlugin<Wry, config::EditorClientConfig> {
    let builder = specta_builder_wry!();

    Builder::<Wry, config::EditorClientConfig>::new(PLUGIN_NAME)
        .invoke_handler(builder.invoke_handler())
        .setup(move |app, api| {
            info!("Setting up {} plugin", PLUGIN_NAME);

            builder.mount_events(app);
            debug!("Registered events for {}", PLUGIN_NAME);

            let editor_client_config = api.config().clone();
            let url = editor_client_config.url.clone();
            let api_paths = editor_client_config.api_paths.clone();
            debug!("Editor client config loaded. URL: {}", url);

            let branch_commit_state: AgCommitState<BranchAgDto> = AgCommitState::new(app.clone());
            app.manage(branch_commit_state);
            debug!("AgCommitState managed");

            let editor_api_client = EditorApiClient::new(&url, api_paths).map_err(|e| {
                error!("Failed to create EditorApiClient: {:?}", e);
                Box::new(e) as Box<dyn std::error::Error>
            })?;
            app.manage(editor_api_client);
            debug!("EditorApiClient created and managed");

            let node_components_state = match NodeComponentState::new() {
                Ok(node_components_state) => node_components_state,
                Err(e) => {
                    error!("Failed to create NodeConfigsState: {:?}", e);
                    return Err(e);
                }
            };
            app.manage(node_components_state);
            debug!("NodeConfigsState managed");

            let cloud_patterns_meta_state = CloudPatternsMetaState::new(app.clone());
            app.manage(cloud_patterns_meta_state);
            debug!("CloudPatternsMetaState managed");

            let cloud_pattern_evaluator_state = CloudPatternEvaluatorState::default();
            app.manage(cloud_pattern_evaluator_state);
            debug!("CloudPatternEvaluatorState managed");

            let model_changes_emitter_process =
                Arc::new(ModelChangesEmitterProcess::new(app.clone()));
            debug!("ModelChangesEmitterProcess created");

            let head_nmg_process = Arc::new(HeadNmgProcessSubject::new());
            app.manage(head_nmg_process.clone());
            debug!("HeadNmgProcessSubject managed");

            let mut branch_ag_dto_subject = BranchAgDtoSubject::new();
            branch_ag_dto_subject
                .register_observer(model_changes_emitter_process.clone())
                .map_err(|e| {
                    error!(
                        "Failed to register ModelChangesEmitterProcess as observer: {:?}",
                        e
                    );
                    Box::new(e) as Box<dyn std::error::Error>
                })?;
            branch_ag_dto_subject
                .register_observer(head_nmg_process.clone())
                .map_err(|e| {
                    error!(
                        "Failed to register HeadNmgProcessSubject as observer: {:?}",
                        e
                    );
                    Box::new(e) as Box<dyn std::error::Error>
                })?;
            app.manage(branch_ag_dto_subject);
            debug!("BranchAgDtoSubject managed with observers registered");

            let cloud_pattern_evaluator_state = CloudPatternEvaluatorState::default();
            app.manage(cloud_pattern_evaluator_state);
            debug!("CloudPatternEvaluatorState managed");

            let viewport_items_static_data_cache = ResourceItemsViewportDataCache::default();
            app.manage(viewport_items_static_data_cache);
            debug!("ViewportItemsStaticDataCache managed");

            let resource_items_schemas = ResourceItemsSchemasCache::default();
            app.manage(resource_items_schemas);
            debug!("ResourceItemsSchemasCache managed");

            let resource_items_partials_cache = ResourceItemsPartialsCache::default();
            app.manage(resource_items_partials_cache);
            debug!("ResourceItemsPartialsCache managed");

            #[cfg(desktop)]
            let editor_client = desktop::init(app, api)?;
            #[cfg(desktop)]
            app.manage(editor_client);

            info!("{} plugin setup completed successfully", PLUGIN_NAME);
            Ok(())
        })
        .build()
}

use std::sync::Arc;

use common_wasm_evaluators::{
    cloud_pattern::{CloudPatternEvaluator, EvaluatorWasiView},
    node_info::NmgProxy,
};
use tauri::{AppHandle, Manager, Runtime};

use super::CloudPatternEvaluatorState;
use crate::{
    error::InternalApplicationError, reorder::head_nmg_process::HeadNmgProcessSubject,
    types::node_component_state::NodeComponentState, EditorError,
};

impl CloudPatternEvaluatorState {
    pub(super) async fn push_evaluator<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        cloud_pattern_id: String,
    ) -> Result<(), EditorError> {
        let node_component_state = app.state::<NodeComponentState>();
        let head_nmg_process = app.state::<Arc<HeadNmgProcessSubject>>();

        let cloud_pattern_meta = self.get_cloud_pattern_meta(&cloud_pattern_id)?;
        log::debug!("CloudPattern metadata: {:?}", cloud_pattern_meta);
        log::debug!("Received CloudPattern metadata: {:#?}", cloud_pattern_meta);

        let mut required_components = vec![cloud_pattern_id.to_owned()];
        required_components.extend(cloud_pattern_meta.referenced_modules);

        node_component_state
            .load_component_codes(app.app_handle(), required_components)
            .await?;

        // Set the CloudPattern ID
        self.set_cloud_pattern_id(cloud_pattern_id.to_owned())?;

        // Create and set the CloudPattern evaluator
        let nmg = head_nmg_process.get_model();
        let node_component_evaluator_store =
            node_component_state.new_wasi_store(nmg.clone()).into_data();
        let cloud_pattern_evaluator_store = EvaluatorWasiView::new(
            node_component_state.node_component_interface(),
            NmgProxy::new(nmg),
            node_component_state.component_store(),
            node_component_evaluator_store,
        );
        let evaluator = CloudPatternEvaluator::new(
            &node_component_state.component_store(),
            cloud_pattern_id.to_owned(),
            cloud_pattern_evaluator_store,
        )?;
        log::debug!("Pushing evaluator for CloudPattern: {}", cloud_pattern_id);
        {
            self.evaluator
                .lock()
                .map(|mut cid| {
                    (*cid).push(evaluator);
                })
                .map_err(|_| InternalApplicationError::CloudPatternEvaluatorLock.into())
        }
    }
}

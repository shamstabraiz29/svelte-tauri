use tauri::{AppHandle, Manager, Runtime};

use super::CloudPatternEvaluatorState;
use crate::{
    models::{
        evaluate_cloud_pattern::request::CloudPatternRequestResponse,
        wasm_evaluators_dtos::DropInfo,
    },
    reorder::branch_ag_dto_subject::BranchAgDtoSubject,
    EditorError,
};

impl CloudPatternEvaluatorState {
    pub(super) async fn update_scope_with_response<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        response: &CloudPatternRequestResponse,
    ) -> Result<(), EditorError> {
        let branch_ag_dto_subject = app.state::<BranchAgDtoSubject>();

        let branch_ag_dto =
            branch_ag_dto_subject
                .get_branch_ag_dto()?
                .ok_or(EditorError::RecoverableError(
                    "No open branch was found! Missing snapshot.".to_owned(),
                ))?;

        match response {
            CloudPatternRequestResponse::Init { cloud_pattern_id } => {
                self.reset()?;

                self.push_evaluator(app, cloud_pattern_id.to_owned())
                    .await?;
            }

            CloudPatternRequestResponse::DropLocationInfo(drop_location_info) => {
                let model_node_id = branch_ag_dto
                    .viewport_items
                    .get(&drop_location_info.dropped_on_node)
                    .ok_or(EditorError::RecoverableError(
                        "No model node ID found for the viewport item.".to_owned(),
                    ))?
                    .model_item_id
                    .to_owned();
                let viewport_type = branch_ag_dto
                    .viewports
                    .get(&drop_location_info.viewport_id)
                    .ok_or(EditorError::RecoverableError(format!(
                        "No viewport found with ID: '{}'.",
                        &drop_location_info.viewport_id
                    )))?
                    .r#type
                    .to_owned();
                self.push_to_context("viewport_type".to_owned(), viewport_type.to_owned())?;
                self.push_to_context(
                    drop_location_info.in_context_name.to_owned(),
                    DropInfo {
                        x: drop_location_info.x,
                        y: drop_location_info.y,
                        viewport_id: drop_location_info.viewport_id.to_owned(),
                        viewport_type,
                        dropped_on_node: model_node_id,
                    },
                )?;
            }
            CloudPatternRequestResponse::PropertiesValues(property_value) => {
                for property_value in property_value.responses.iter() {
                    self.push_to_context(
                        property_value.in_context_name.to_owned(),
                        property_value.value.to_owned(),
                    )?;
                }
            }
        }
        Ok(())
    }
}

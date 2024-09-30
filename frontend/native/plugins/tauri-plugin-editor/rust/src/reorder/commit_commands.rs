// frontend/tauri/plugins/tauri-plugin-editor/rust/src/reorder/commit_commands.rs
use common_commands::model::ModelCommand;
use frontend_tauri_plugins_common::types::bearer_tokens::BearerTokens;
use log::{debug, error, info, warn};
use tauri::{AppHandle, Manager, Runtime};

use crate::EditorError;

use super::{branch_ag_dto_subject::BranchAgDtoSubject, send_model_commands_to_cloud};

pub async fn commit_commands<R: Runtime>(
    app: AppHandle<R>,
    commands: Vec<ModelCommand>,
) -> Result<(), EditorError> {
    info!("Starting commit_commands process");
    debug!("Number of commands to commit: {}", commands.len());

    let bearer_tokens = app.state::<BearerTokens<R>>();
    let access_token = match bearer_tokens.access_token() {
        Some(token) => {
            debug!("Access token retrieved successfully");
            token
        }
        None => {
            warn!("No access token found");
            return Err(EditorError::RecoverableError(
                "No access token found".to_string(),
            ));
        }
    };

    info!("Sending model commands to cloud");
    let updated_branch_ag_dto =
        match send_model_commands_to_cloud(app.clone(), &access_token, commands).await {
            Ok(dto) => {
                debug!("Successfully received updated branch AG DTO from cloud");
                dto
            }
            Err(e) => {
                error!("Failed to send model commands to cloud: {:?}", e);
                return Err(e);
            }
        };

    let branch_ag_dto_subject = app.state::<BranchAgDtoSubject>();
    info!("Setting updated branch AG DTO in subject");
    if let Err(e) = branch_ag_dto_subject
        .set_value(Some(updated_branch_ag_dto))
        .await
    {
        error!("Failed to set updated branch AG DTO in subject: {:?}", e);
        return Err(e);
    }
    debug!("Successfully set updated branch AG DTO in subject");

    info!("commit_commands process completed successfully");
    Ok(())
}

// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/evaluate_cloud_pattern.rs
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use log::{debug, error, info, warn};
use tauri::{AppHandle, Runtime, State};

use crate::{
    models::evaluate_cloud_pattern::{
        request::EvaluateCloudPatternRequest, response::EvaluateCloudPatternResponse,
    },
    types::cloud_pattern_evaluator_state::CloudPatternEvaluatorState,
};

#[tauri::command]
#[specta::specta]
pub async fn evaluate_cloud_pattern<R: Runtime>(
    app: AppHandle<R>,
    bearer_tokens: State<'_, BearerTokens<R>>,
    cloud_patterns_evaluator_state: State<'_, CloudPatternEvaluatorState>,
    request: EvaluateCloudPatternRequest,
) -> Result<EvaluateCloudPatternResponse> {
    info!(
        "Starting evaluate_cloud_pattern with request ID: {}",
        request.req_id
    );
    debug!("Received evaluate cloud pattern request: {:?}", request);

    let access_token = match bearer_tokens.access_token() {
        Some(token) => {
            debug!("Access token found");
            token
        }
        None => {
            warn!("No access token found for request ID: {}", request.req_id);
            return Err(Error {
                req_id: request.req_id,
                message: "No access token found".to_string(),
            });
        }
    };

    info!("Calling evaluate_cloud_pattern_inner");
    evaluate_cloud_pattern_inner(app, cloud_patterns_evaluator_state, access_token, request).await
}

async fn evaluate_cloud_pattern_inner<R: Runtime>(
    app: AppHandle<R>,
    cloud_patterns_evaluator_state: State<'_, CloudPatternEvaluatorState>,
    access_token: String,
    request: EvaluateCloudPatternRequest,
) -> Result<EvaluateCloudPatternResponse> {
    info!(
        "Starting evaluate_cloud_pattern_inner with request ID: {}",
        request.req_id
    );
    debug!(
        "Evaluating cloud pattern with access token: {}",
        access_token
    );

    let req_id = request.req_id.to_owned();

    match cloud_patterns_evaluator_state
        .evaluate(app, access_token, request)
        .await
    {
        Ok(evaluate_cloud_pattern_type) => {
            info!(
                "Cloud pattern evaluation successful for request ID: {}",
                req_id
            );
            debug!("Evaluation result: {:?}", evaluate_cloud_pattern_type);
            Ok(EvaluateCloudPatternResponse {
                req_id,
                eval_result: evaluate_cloud_pattern_type,
            })
        }
        Err(e) => {
            error!(
                "Cloud pattern evaluation failed for request ID: {}. Error: {}",
                req_id, e
            );
            Err(Error {
                req_id,
                message: e.to_string(),
            })
        }
    }
}

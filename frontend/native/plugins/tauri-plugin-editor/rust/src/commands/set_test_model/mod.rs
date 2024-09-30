// frontend/tauri/plugins/tauri-plugin-editor/rust/src/commands/set_test_model.rs

use std::collections::HashMap;

use common_simple_types::ag_id::AgId;
use frontend_tauri_plugins_common::{
    error::{Error, Result},
    types::bearer_tokens::BearerTokens,
};
use regex::Regex;
use tauri::{AppHandle, Manager, Runtime};

use crate::{
    models::set_test_model::{request::SetTestModelRequest, response::SetTestModelResponse},
    reorder::{branch_ag_dto_subject::BranchAgDtoSubject, send_model_commands_to_cloud},
};

fn load_test_model(
    branch_id: &AgId,
    model_root_node_id: &AgId,
    canvas_node_id: &AgId,
    viewport_id: &AgId,
) -> String {
    let mut source_content = include_str!("test_model_commands.json").to_owned();

    // Replace {branch_id}, {model_root_node_id}, and {viewport_id} with the provided values
    source_content = source_content.replace("{branch_id}", &branch_id.to_string());
    source_content = source_content.replace("{canvas_id}", &canvas_node_id.to_string());
    source_content =
        source_content.replace("{model_root_node_id}", &model_root_node_id.to_string());
    source_content = source_content.replace("{viewport_id}", &viewport_id.to_string());

    // Find all strings matching "<identifier: generate_ag_id>" and generate AgIds
    let id_pattern = Regex::new(r"<(\w+):\s*generate_ag_id>").unwrap();
    let mut id_map = HashMap::new();

    // First pass: Generate UUIDs and store them in the map
    for captures in id_pattern.captures_iter(&source_content) {
        let identifier = captures.get(1).unwrap().as_str();
        let uuid = AgId::default();
        id_map.insert(identifier.to_string(), uuid);
    }

    // Second pass: Replace "<identifier: generate_ag_id>" patterns with generated AgIds
    source_content = id_pattern
        .replace_all(&source_content, |caps: &regex::Captures| {
            let identifier = caps.get(1).unwrap().as_str();
            id_map.get(identifier).unwrap().to_string()
        })
        .into_owned();

    // Replace all occurrences of "{identifier}" with the generated AgIds
    let replace_pattern = Regex::new(r"\{(\w+)\}").unwrap();
    let result = replace_pattern.replace_all(&source_content, |caps: &regex::Captures| {
        let identifier = &caps[1];
        id_map
            .get(identifier)
            .map_or(caps[0].to_string(), |ag_id| ag_id.to_string())
    });

    result.to_string()
}

#[tauri::command]
#[specta::specta]
pub async fn set_test_model<R: Runtime>(
    app: AppHandle<R>,
    request: SetTestModelRequest,
) -> Result<SetTestModelResponse> {
    log::info!(
        "Starting set_test_model function with request: {:?}",
        request
    );

    let bearer_tokens = app.state::<BearerTokens<R>>();
    let branch_ag_dto_subject = app.state::<BranchAgDtoSubject>();

    log::debug!("Retrieved all necessary app states");

    let access_token = match bearer_tokens.access_token() {
        Some(token) => token,
        None => {
            log::error!("No access token found");
            return Err(Error {
                req_id: request.req_id.clone(),
                message: "No access token found".to_string(),
            });
        }
    };

    log::trace!("Access token retrieved successfully");

    struct BranchData {
        branch_id: AgId,
        model_root_node_id: AgId,
        canvas_node_id: AgId,
    }

    let mut branch_data: Result<BranchData> = Err(Error {
        req_id: request.req_id.clone(),
        message: "No open branch was found".to_string(),
    });

    let eval_result = branch_ag_dto_subject.eval_with_value(|branch_ag_dto| {
        if let Some(branch_ag_dto) = branch_ag_dto {
            let canvas_id = branch_ag_dto.nodes.values().find_map(|node| {
                if node.node_type == "canvas" {
                    Some(node.id.clone())
                } else {
                    None
                }
            });
            let Some(canvas_node_id) = canvas_id else {
                branch_data = Err(Error {
                    req_id: request.req_id.clone(),
                    message: "No canvas node found".to_string(),
                });
                return;
            };
            branch_data = Ok(BranchData {
                branch_id: branch_ag_dto.id.clone(),
                model_root_node_id: branch_ag_dto.root_node_id.clone(),
                canvas_node_id,
            });
        }
    });

    if let Err(e) = eval_result {
        log::error!("Evaluating with BranchAgDto error: {}", e);
        return Err(Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        });
    };

    let BranchData {
        branch_id,
        model_root_node_id,
        canvas_node_id,
    } = branch_data?;

    let commands_str = load_test_model(
        &branch_id,
        &model_root_node_id,
        &canvas_node_id,
        &request.viewport_id,
    );

    let commands: Vec<common_commands::model::ModelCommand> =
        match serde_json::from_str(&commands_str) {
            Ok(commands) => commands,
            Err(e) => {
                log::error!("Error deserializing test model commands: {}", e);
                return Err(Error {
                    req_id: request.req_id.clone(),
                    message: e.to_string(),
                });
            }
        };

    let new_branch_ag_dto =
        match send_model_commands_to_cloud(app.clone(), &access_token, commands).await {
            Ok(branch_ag_dto) => branch_ag_dto,
            Err(e) => {
                log::error!("Error sending model commands to cloud: {}", e);
                return Err(Error {
                    req_id: request.req_id.clone(),
                    message: e.to_string(),
                });
            }
        };

    let branch_ag_dto_subject = app.state::<BranchAgDtoSubject>();

    if let Err(e) = branch_ag_dto_subject
        .set_value(Some(new_branch_ag_dto))
        .await
    {
        log::error!("Error updating the open branch's snapshot: {}", e);
        return Err(Error {
            req_id: request.req_id.clone(),
            message: e.to_string(),
        });
    };

    Ok(SetTestModelResponse {
        req_id: request.req_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_test_model() {
        let branch_id = AgId::default();
        let model_root_node_id = AgId::default();
        let viewport_id = AgId::default();
        let canvas_node_id = AgId::default();
        let yaml_content = load_test_model(
            &branch_id,
            &model_root_node_id,
            &canvas_node_id,
            &viewport_id,
        );

        assert!(yaml_content.contains(&branch_id.to_string()));
        assert!(yaml_content.contains(&model_root_node_id.to_string()));
        assert!(yaml_content.contains(&viewport_id.to_string()));
        assert!(yaml_content.contains(&canvas_node_id.to_string()));
    }

    #[test]
    fn test_parse_test_model_commands() {
        let branch_id = AgId::default();
        let model_root_node_id = AgId::default();
        let viewport_id = AgId::default();
        let canvas_node_id = AgId::default();
        let model_str = load_test_model(
            &branch_id,
            &model_root_node_id,
            &canvas_node_id,
            &viewport_id,
        );

        let commands: Vec<common_commands::model::ModelCommand> =
            serde_json::from_str(&model_str).unwrap();

        println!("{:#?}", commands);

        assert!(!commands.is_empty());
    }
}

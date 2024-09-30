use std::{collections::HashSet, sync::Arc};

use common_aggregates::BranchAg;
use common_commands::model::ModelCommand;
use common_dtos::{
    commit_client::types::branch::branch_dto::BranchAgDto,
    editor_client::types::resource_item_schema::ResourceItemSchema,
};

use common_simple_types::{ag_id::AgId, USER_ID_ATTR_NAME};

use common_wasm_evaluators::{
    cloud_pattern::exports::cloudcad::cloud_pattern::evaluator::EvaluationRequest,
    node_info::exports::cloudcad::node_info::description::{
        ValidateInsertRelationshipRequest, ValidateRelationshipsRequest,
    },
};
use frontend_tauri_plugins_common::{
    states::ag_commit_state::AgCommitState, types::bearer_tokens::BearerTokens,
};

use tauri::{AppHandle, Manager, Runtime};

use super::CloudPatternEvaluatorState;
use crate::{
    error::InternalApplicationError,
    models::evaluate_cloud_pattern::{
        request::EvaluateCloudPatternRequest,
        response::{
            CloudPatternEvaluationStep, DropInfoRequest, InputType, ItemType,
            PropertiesValuesRequest, PropertyValueRequest, SelectItemsRequest, ViewPortItem,
        },
    },
    reorder::{
        apply_commands_to_branch_ag_dto, branch_ag_dto_subject::BranchAgDtoSubject,
        head_nmg_process::HeadNmgProcessSubject, send_model_commands_to_cloud,
    },
    types::{
        cloud_pattern_evaluator_state::{
            create_node_request_into_commands::CreateNodeCmds,
            create_relationship_request_into_commands::CreateRelationshipCmds,
        },
        node_component_state::NodeComponentState,
        resource_items_schema_cache::ResourceItemsSchemasCache,
    },
    EditorError,
};

impl CloudPatternEvaluatorState {
    pub async fn evaluate<R: Runtime>(
        &self,
        app: AppHandle<R>,
        access_token: String,
        request: EvaluateCloudPatternRequest,
    ) -> Result<CloudPatternEvaluationStep, EditorError> {
        log::info!("evaluate");

        let branch_ag_dto_subject = app.state::<BranchAgDtoSubject>();

        let caller_id = {
            let bearer_tokens = app.state::<BearerTokens<R>>();
            let claims = bearer_tokens
                .id_token_claims()
                .ok_or(InternalApplicationError::BearerTokensIdClaimsNotPresent)?;
            claims
                .get(USER_ID_ATTR_NAME)
                .cloned()
                .ok_or(InternalApplicationError::BearerTokensUserIdClaimNotPresent)?
                .as_str()
                .ok_or(InternalApplicationError::BearerTokensUserIdClaimNotString)?
                .to_string()
        };

        // set up the scope with the responding_with value if it exists
        let responding_with = &request.responding_with;

        log::debug!("Response provided by the caller: {:#?}", responding_with);
        log::debug!("Setting up the scope with the response.");
        self.update_scope_with_response(app.app_handle(), responding_with)
            .await?;

        // TODO: Limit the number of iterations to prevent infinite loops
        loop {
            log::debug!("In CloudPattern evaluation loop.");

            let result = {
                let state = self.get_current_context()?;
                log::debug!("Current context: {:#?}", state);
                let evaluator_guard = self
                    .evaluator
                    .lock()
                    .map_err(|_| InternalApplicationError::CloudPatternEvaluatorLock)?;
                let evaluator =
                    (*evaluator_guard)
                        .iter()
                        .last()
                        .ok_or(EditorError::RecoverableError(
                            "CloudPattern evaluator not initialized!".to_string(),
                        ))?;
                evaluator.evaluate(&state)
            }?;

            let apply_model_changes = |branch_ag_dto: BranchAgDto,
                                       commands: Vec<ModelCommand>|
             -> Result<BranchAg, EditorError> {
                let branch_commit_state = app.state::<AgCommitState<BranchAgDto>>();
                let commit_info = branch_commit_state.get_ag_commit_info().ok_or(
                    EditorError::RecoverableError(
                        "No open branch was found! Missing commit info.".to_owned(),
                    ),
                )?;

                if commit_info.commit_id != branch_ag_dto.commit_id {
                    log::debug!("Commit info: {:#?}", commit_info);
                    log::debug!("BranchAgDto Commit ID: {}", branch_ag_dto.commit_id);
                    return Err(EditorError::RecoverableError(
                            "The commit ID in the branch_ag_dto does not match the commit ID in the commit info.".to_owned(),
                        ));
                }

                let updated_branch_ag = apply_commands_to_branch_ag_dto(
                    branch_ag_dto,
                    &commit_info.next_commit_id,
                    &caller_id,
                    &commands,
                )?;

                {
                    // Scoping for write lock
                    let mut cmds = self
                        .commands
                        .lock()
                        .map_err(|_| InternalApplicationError::CloudPatternEvaluatorCommandsLock)?;
                    match cmds.as_mut() {
                        Some(cmds) => {
                            cmds.extend(commands);
                        }
                        None => {
                            *cmds = Some(commands);
                        }
                    }
                }

                Ok(updated_branch_ag)
            };

            match result {
                EvaluationRequest::DropInfo(drop_info_request) => {
                    log::debug!("The CloudPattern evaluation resulted in a DropInfoRequest.");
                    log::debug!("DropInfoRequest: {:#?}", drop_info_request);
                    log::debug!("Getting the metadata of the node in the DropInfoRequest.");

                    let branch_ag_dto = branch_ag_dto_subject.get_branch_ag_dto()?.ok_or(
                        EditorError::RecoverableError(
                            "No open branch was found! Missing snapshot.".to_owned(),
                        ),
                    )?;

                    let resource_items_schema_cache = app.state::<ResourceItemsSchemasCache>();

                    let resource_items_schemas = resource_items_schema_cache
                        .get_resource_items_schemas(
                            app.app_handle(),
                            HashSet::from_iter([drop_info_request.node_type.to_owned()]),
                        )
                        .await?;
                    let node_meta = match resource_items_schemas.get(&drop_info_request.node_type) {
                        Some(node_meta) => match node_meta {
                            ResourceItemSchema::Node(node_meta) => node_meta,
                            _ => {
                                return Err(EditorError::RecoverableError(format!(
                                    "Expected '{}' to be a node.",
                                    drop_info_request.node_type
                                )))
                            }
                        },
                        None => {
                            return Err(InternalApplicationError::NodeMetaNotFound {
                                node_type: drop_info_request.node_type.to_owned(),
                            }
                            .into());
                        }
                    };

                    log::debug!("Node metadata: {:#?}", node_meta);
                    log::debug!("Returning a DropInfoRequest to the caller.");

                    return Ok(CloudPatternEvaluationStep::DropInfoRequest(
                        DropInfoRequest {
                            in_context_name: drop_info_request.in_context_name.to_owned(),
                            cursor_icon_url: Some(node_meta.cursor_icon_url.to_owned()),
                            valid_drop_locations: drop_info_request
                                .candidate_nodes
                                .into_iter()
                                .fold(Vec::new(), |mut viewport_items, model_node_id| {
                                    let model_node_id = AgId::from(model_node_id);
                                    let viewport_item_id = branch_ag_dto
                                        .viewport_items
                                        .values()
                                        .find_map(|viewport_item| {
                                            if viewport_item.model_item_id == model_node_id {
                                                Some(viewport_item.id.to_owned())
                                            } else {
                                                None
                                            }
                                        });
                                    if let Some(viewport_item_id) = viewport_item_id {
                                        viewport_items.push(ViewPortItem {
                                            model_item_id: model_node_id,
                                            item_type: ItemType::Node,
                                            viewport_item_id,
                                        });
                                    }
                                    viewport_items
                                }),
                        },
                    ));
                }
                EvaluationRequest::SelectItems(select_items_request) => {
                    log::debug!("The CloudPattern evaluation resulted in a SelectItemsRequest.");
                    let branch_ag_dto = branch_ag_dto_subject.get_branch_ag_dto()?.ok_or(
                        EditorError::RecoverableError(
                            "No open branch was found! Missing snapshot.".to_owned(),
                        ),
                    )?;
                    let viewport_type = select_items_request.viewport_type.to_owned();
                    let viewport_items_ids = branch_ag_dto
                        .viewport_items
                        .values()
                        .filter_map(|viewport_item| {
                            let viewport_item_viewport_type =
                                match branch_ag_dto.viewports.get(&viewport_item.viewport_id) {
                                    Some(viewport) => viewport.r#type.to_owned(),
                                    None => {
                                        return Some(Err(format!(
                                            "Viewport '{}' not found.",
                                            viewport_item.viewport_id
                                        )));
                                    }
                                };
                            if select_items_request
                                .resource_items_ids
                                .contains(&viewport_item.model_item_id.to_string())
                                && viewport_item_viewport_type == viewport_type
                            {
                                Some(Ok(viewport_item.id.to_owned()))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<Result<AgId, String>>>();

                    let mut unwrapped_viewport_items_ids: Vec<AgId> = Vec::new();

                    for viewport_item_id in viewport_items_ids {
                        match viewport_item_id {
                            Ok(viewport_item_id) => {
                                unwrapped_viewport_items_ids.push(viewport_item_id);
                            }
                            Err(error) => {
                                return Err(EditorError::RecoverableError(error));
                            }
                        }
                    }

                    log::debug!("Returning a SelectItemsRequest to the caller.");
                    return Ok(CloudPatternEvaluationStep::SelectItemsRequest(
                        SelectItemsRequest {
                            in_context_name: select_items_request.in_context_name.to_owned(),
                            viewport_items: unwrapped_viewport_items_ids,
                        },
                    ));
                }
                EvaluationRequest::PropertiesValues(properties_values_request) => {
                    log::debug!(
                        "The CloudPattern evaluation resulted in a PropertiesValuesRequest."
                    );
                    let event_payload: Vec<PropertyValueRequest> = properties_values_request
                        .into_iter()
                        .map(|property| PropertyValueRequest {
                            in_context_name: property.in_context_name.to_owned(),
                            value_type: InputType::from(property.value_type.to_owned()),
                        })
                        .collect();
                    log::debug!("Returning a PropertiesValuesRequest to the caller.");
                    return Ok(CloudPatternEvaluationStep::PropertiesValuesRequest(
                        PropertiesValuesRequest {
                            requests: event_payload,
                        },
                    ));
                }
                EvaluationRequest::CreateNode(create_node_request) => {
                    log::debug!("The CloudPattern evaluation resulted in a CreateNodeRequest.");

                    let node_component_state = app.state::<NodeComponentState>();
                    let head_nmg_process = app.state::<Arc<HeadNmgProcessSubject>>();
                    let resource_items_schema_cache = app.state::<ResourceItemsSchemasCache>();
                    let nmg = head_nmg_process.get_model();

                    let resource_items_schemas = resource_items_schema_cache
                        .get_resource_items_schemas(
                            app.app_handle(),
                            HashSet::from_iter([create_node_request.node_type.to_owned()]),
                        )
                        .await?;
                    let node_meta = match resource_items_schemas.get(&create_node_request.node_type)
                    {
                        Some(node_meta) => match node_meta {
                            ResourceItemSchema::Node(node_meta) => node_meta,
                            _ => {
                                return Err(EditorError::RecoverableError(format!(
                                    "Expected '{}' to be a node.",
                                    create_node_request.node_type
                                )))
                            }
                        },
                        None => {
                            return Err(InternalApplicationError::NodeMetaNotFound {
                                node_type: create_node_request.node_type.to_owned(),
                            }
                            .into());
                        }
                    };

                    node_component_state
                        .validate_properties(
                            app.app_handle(),
                            &create_node_request.node_type,
                            &create_node_request.properties,
                            nmg.clone(),
                        )
                        .await?;

                    node_component_state
                        .validate_insert_relationship(
                            app.app_handle(),
                            &create_node_request.node_type,
                            &ValidateInsertRelationshipRequest {
                                relationship_type: create_node_request
                                    .insert_relationship
                                    .relationship_type
                                    .to_owned(),
                                target_node_id: create_node_request
                                    .insert_relationship
                                    .node_id
                                    .to_owned(),
                            },
                            nmg.clone(),
                        )
                        .await?;

                    let branch_ag_dto = branch_ag_dto_subject.get_branch_ag_dto()?.ok_or(
                        EditorError::RecoverableError(
                            "No open branch was found! Missing snapshot.".to_owned(),
                        ),
                    )?;

                    let in_context_name = create_node_request.in_context_name.to_owned();

                    let CreateNodeCmds { node_id, commands } =
                        Self::create_node_request_into_commands(
                            branch_ag_dto.id.to_owned(),
                            node_meta.labels.to_owned(),
                            create_node_request.try_into()?,
                        );

                    log::debug!("Model commands: {:#?}", commands);

                    let updated_branch_ag = apply_model_changes(branch_ag_dto, commands)?;

                    // push the ID of the created node to the context
                    self.push_to_context(in_context_name, node_id)?;

                    let updated_branch_ag_dto = (&updated_branch_ag).into();

                    // should update the value in the branch_ag_dto_subject resulting in:
                    // - an update to the nmg
                    // - emission on upserted viewports and viewport_items if any
                    branch_ag_dto_subject
                        .set_value(Some(updated_branch_ag_dto))
                        .await?;
                }
                EvaluationRequest::CreateRelationship(create_relationship_request) => {
                    log::debug!(
                        "The CloudPattern evaluation resulted in a CreateRelationshipRequest."
                    );

                    let node_component_state = app.state::<NodeComponentState>();
                    let head_nmg_process = app.state::<Arc<HeadNmgProcessSubject>>();
                    let nmg = head_nmg_process.get_model();

                    let node_relationships = vec![ValidateRelationshipsRequest {
                        source_node_id: create_relationship_request.source_node_id.to_owned(),
                        target_node_id: create_relationship_request.target_node_id.to_owned(),
                        relationships: vec![create_relationship_request
                            .relationship_type
                            .to_owned()],
                    }];
                    node_component_state
                        .validate_relationships(
                            app.app_handle(),
                            &create_relationship_request.source_node_id,
                            &node_relationships,
                            nmg.clone(),
                        )
                        .await?;

                    let branch_ag_dto = branch_ag_dto_subject.get_branch_ag_dto()?.ok_or(
                        EditorError::RecoverableError(
                            "No open branch was found! Missing snapshot.".to_owned(),
                        ),
                    )?;

                    let in_context_name = create_relationship_request.in_context_name.to_owned();

                    let CreateRelationshipCmds {
                        relationship_id,
                        commands,
                    } = Self::create_relationship_request_into_commands(
                        branch_ag_dto.id.to_owned(),
                        create_relationship_request.try_into()?,
                    );

                    let updated_branch_ag = apply_model_changes(branch_ag_dto, commands)?;

                    // push the ID of the created relationship to the context
                    self.push_to_context(in_context_name, relationship_id)?;

                    // should update the value in the branch_ag_dto_subject resulting in:
                    // - an update to the nmg
                    // - emission on upserted viewports and viewport_items if any
                    branch_ag_dto_subject
                        .set_value(Some((&updated_branch_ag).into()))
                        .await?;
                }
                EvaluationRequest::SetVariable(set_variable_request) => {
                    self.push_to_context(
                        set_variable_request.in_context_name,
                        set_variable_request.value,
                    )?;
                }
                EvaluationRequest::Compose(compose_request) => {
                    if !self.current_context_contains(&compose_request.namespace_id)? {
                        self.push_context_namespace(compose_request.namespace_id)?;
                        self.push_evaluator(app.app_handle(), compose_request.cloud_pattern_id)
                            .await?;
                    }
                }
                EvaluationRequest::Done => {
                    self.pop_evaluator()?;
                    self.pop_context_namespace()?;
                    if self.evaluator_stack_length()? != 0 {
                        continue;
                    }

                    let commands = self.take_commands()?;

                    let cloud_pattern_id = self.get_cloud_pattern_id()?;
                    if commands.is_empty() {
                        return Err(EditorError::RecoverableError(format!(
                            "The '{}' CloudPattern generated 0 changes.",
                            cloud_pattern_id
                        )));
                    }

                    //TODO: Send the commands to the backend (receive the new head BranchAgDto)
                    let new_branch_ag_dto =
                        send_model_commands_to_cloud(app.clone(), &access_token, commands).await?;

                    let branch_ag_dto_subject = app.state::<BranchAgDtoSubject>();
                    //TODO: Set the new head BranchAgDto in the branch_ag_dto_subject
                    branch_ag_dto_subject
                        .set_value(Some(new_branch_ag_dto))
                        .await?;

                    self.reset()?;

                    return Ok(CloudPatternEvaluationStep::Complete);
                }
            }
        }
    }
}

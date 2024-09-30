use common_aggregates::ag::AgModificationError;
use common_events_player::error::EventPlayerError;
use common_libs_editor_api_client::Error as EditorResourcesApiClientError;
use common_nmg_core::error::{NmgOpError, NmgQueryError};
use common_simple_types::ag_id::AgId;
use common_wasm_components_store::Error as ComponentStoreError;
use common_wasm_evaluators::Error as WasmEvaluatorError;

use super::{
    head_nmg_process::HeadNmgProcessError, nmg_from_branch_ag_dto::NmgFromBranchAgDtoError,
};

#[derive(Debug)]
pub enum InternalApplicationError {
    NamespaceNotInContext {
        namespace: String,
    },
    NamespaceNotAnObject {
        namespace: String,
    },
    ResourceItemsPartialsLock,
    ResourceItemsSchemasLock,
    ResourceItemsViewportDataLock,
    NodeComponentCodeNotFound {
        node_type: String,
    },
    CloudPatternEvaluatorStateLock,
    BearerTokensAccessTokenNotPresent,
    BearerTokensIdClaimsNotPresent,
    BearerTokensUserIdClaimNotPresent,
    BearerTokensUserIdClaimNotString,
    MmcCmdsReadLock,
    MmcCmdsWriteLock,
    // TODO: ^^^^^^^^^^^^^^^^^  Remove
    SerdeJson(serde_json::Error),
    BranchAgHashMismatch,
    BranchModelModifyError {
        branch_id: AgId,
        message: String,
    },
    EmptyAgDelta,
    CloudPatternEvaluatorMetaNotFound {
        cloud_pattern_id: String,
    },
    CloudPatternEvaluatorMetaLock,
    CloudPatternEvaluatorIdReadLock,
    CloudPatternEvaluatorIdLock,
    CloudPatternEvaluatorLock,
    CloudPatternEvaluatorIdMismatch,
    FetchCloudPatternScript {
        script_path: String,
        message: String,
    },
    AgModification(AgModificationError),
    CloudPatternEvaluatorCommandsLock,
    MultiAgCommandsInTransaction,
    CloudPatternEvaluationResultReadLock,
    BranchAgValuesReadLock,
    BranchAgValuesWriteLock,
    HeadNmgWriteLock,
    HeadNmgClear(NmgQueryError),
    HeadNmgProcess(HeadNmgProcessError),
    ViewportItemViewportIdMismatch {
        old_viewport_id: AgId,
        new_viewport_id: AgId,
        viewport_item_id: AgId,
    },
    EmitEvent {
        event: &'static str,
        error: String,
    },
    NmgFromBranchAgDto(NmgFromBranchAgDtoError),
    WasmEvaluator(WasmEvaluatorError),
    ComponentStore(ComponentStoreError),
    EditorResourcesApi(EditorResourcesApiClientError),
    NmgNodeNotFound {
        node_id: String,
    },
    NodeMetaNotFound {
        node_type: String,
    },
    EventPlayer(EventPlayerError),
    ViewportNotFound {
        viewport_id: AgId,
    },
    Other(anyhow::Error),
}

impl std::fmt::Display for InternalApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NamespaceNotInContext { namespace } => {
                write!(f, "Namespace '{}' not found in the context.", namespace)
            }
            Self::NamespaceNotAnObject { namespace } => {
                write!(
                    f,
                    "Namespace '{}' is not an object in the context.",
                    namespace
                )
            }
            Self::NodeComponentCodeNotFound { node_type } => {
                write!(f, "Node component code not found: node_type: {}", node_type)
            }
            Self::BearerTokensAccessTokenNotPresent => {
                write!(f, "Access token not present in bearer tokens")
            }
            Self::BearerTokensIdClaimsNotPresent => {
                write!(f, "Id claims not present in bearer tokens")
            }
            Self::BearerTokensUserIdClaimNotPresent => {
                write!(f, "Sub claim not present in bearer tokens")
            }
            Self::BearerTokensUserIdClaimNotString => {
                write!(f, "Sub claim not string in bearer tokens")
            }
            Self::MmcCmdsReadLock => write!(f, "MMC commands read lock"),
            Self::MmcCmdsWriteLock => write!(f, "MMC commands write lock"),
            Self::ResourceItemsViewportDataLock => {
                write!(f, "Resource items viewport data lock")
            }
            Self::ResourceItemsSchemasLock => write!(f, "Resource items schemas lock"),
            Self::ResourceItemsPartialsLock => write!(f, "Resource items partials lock"),
            Self::SerdeJson(e) => write!(f, "Serde JSON error: {}", e),
            Self::BranchAgHashMismatch => write!(f, "Branch AG hash mismatch"),
            Self::BranchModelModifyError { branch_id, message } => {
                write!(
                    f,
                    "Branch model modify error: branch_id: {}, message: {}",
                    branch_id, message
                )
            }
            Self::EmptyAgDelta => write!(f, "Empty AG delta"),
            Self::CloudPatternEvaluatorMetaNotFound { cloud_pattern_id } => {
                write!(
                    f,
                    "Cloud pattern evaluator meta not found: cloud_pattern_id: {}",
                    cloud_pattern_id
                )
            }
            Self::CloudPatternEvaluatorMetaLock => {
                write!(f, "Cloud pattern evaluator meta lock")
            }
            Self::CloudPatternEvaluatorIdReadLock => {
                write!(f, "Cloud pattern evaluator ID read lock")
            }
            Self::CloudPatternEvaluatorIdLock => {
                write!(f, "Cloud pattern evaluator ID write lock")
            }
            Self::CloudPatternEvaluatorLock => write!(f, "Cloud pattern evaluator lock"),
            Self::CloudPatternEvaluatorIdMismatch => {
                write!(f, "Cloud pattern evaluator ID mismatch")
            }
            Self::FetchCloudPatternScript {
                script_path,
                message,
            } => {
                write!(
                    f,
                    "Fetch cloud pattern script: script_path: {}, message: {}",
                    script_path, message
                )
            }
            Self::AgModification(e) => write!(f, "AG modification error: {}", e),
            Self::CloudPatternEvaluatorCommandsLock => {
                write!(f, "Cloud pattern evaluator commands lock")
            }
            Self::MultiAgCommandsInTransaction => write!(f, "Multiple AG commands in transaction"),
            Self::CloudPatternEvaluatorStateLock => {
                write!(f, "Cloud pattern evaluator state lock")
            }
            Self::CloudPatternEvaluationResultReadLock => {
                write!(f, "Cloud pattern evaluation result read lock")
            }
            Self::BranchAgValuesReadLock => write!(f, "Branch AG values read lock"),
            Self::BranchAgValuesWriteLock => write!(f, "Branch AG values write lock"),
            Self::HeadNmgClear(e) => write!(f, "Head NMG clear: {}", e),
            Self::HeadNmgWriteLock => write!(f, "Head NMG write lock"),
            Self::HeadNmgProcess(e) => write!(f, "Head NMG process error: {}", e),
            Self::ViewportItemViewportIdMismatch {
                old_viewport_id,
                new_viewport_id,
                viewport_item_id,
            } => {
                write!(f, "Viewport item viewport ID mismatch: old_viewport_id: {}, new_viewport_id: {}, viewport_item_id: {}", old_viewport_id, new_viewport_id, viewport_item_id)
            }
            Self::EmitEvent { event, error } => {
                write!(f, "Emit event: event: {}, error: {}", event, error)
            }
            Self::NmgFromBranchAgDto(e) => write!(f, "NMG from branch AG DTO error: {}", e),
            Self::WasmEvaluator(e) => write!(f, "Wasm evaluator error: {}", e),
            Self::ComponentStore(e) => write!(f, "Component store error: {}", e),
            Self::EditorResourcesApi(e) => write!(f, "Editor resources API error: {}", e),
            Self::NmgNodeNotFound { node_id } => {
                write!(f, "NMG node not found: node_id: {}", node_id)
            }
            Self::NodeMetaNotFound { node_type } => {
                write!(f, "Node meta not found: node_type: {}", node_type)
            }
            Self::EventPlayer(e) => write!(f, "Event player error: {}", e),
            Self::ViewportNotFound { viewport_id } => {
                write!(f, "Viewport not found: viewport_id: {}", viewport_id)
            }
            Self::Other(e) => write!(f, "{}", e),
        }
    }
}

impl From<NmgOpError> for InternalApplicationError {
    fn from(value: NmgOpError) -> Self {
        Self::NmgFromBranchAgDto(NmgFromBranchAgDtoError::NmgOp(value))
    }
}

impl From<WasmEvaluatorError> for InternalApplicationError {
    fn from(value: WasmEvaluatorError) -> Self {
        Self::WasmEvaluator(value)
    }
}

impl From<ComponentStoreError> for InternalApplicationError {
    fn from(value: ComponentStoreError) -> Self {
        Self::ComponentStore(value)
    }
}

impl From<EditorResourcesApiClientError> for InternalApplicationError {
    fn from(value: EditorResourcesApiClientError) -> Self {
        Self::EditorResourcesApi(value)
    }
}

impl From<AgModificationError> for InternalApplicationError {
    fn from(value: AgModificationError) -> Self {
        Self::AgModification(value)
    }
}

impl From<EventPlayerError> for InternalApplicationError {
    fn from(value: EventPlayerError) -> Self {
        Self::EventPlayer(value)
    }
}

impl From<serde_json::Error> for InternalApplicationError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl<T: Into<InternalApplicationError>> From<T> for crate::EditorError {
    fn from(value: T) -> Self {
        let internal_application_error: InternalApplicationError = value.into();
        log::error!("{}", internal_application_error.to_string());
        Self::InternalApplicationError(internal_application_error.to_string())
    }
}

impl From<anyhow::Error> for InternalApplicationError {
    fn from(value: anyhow::Error) -> Self {
        Self::Other(value)
    }
}

use common_nmg_core::error::NmgOpError;
use common_simple_types::ag_id::AgId;

use super::InternalApplicationError;

#[derive(Debug, thiserror::Error)]
pub enum NmgFromBranchAgDtoError {
    #[error(transparent)]
    NmgOp(NmgOpError),
    #[error("The origin node({node_id}) for relationship({rel_id}) was not found!")]
    RelFromNodeNotFound { rel_id: AgId, node_id: AgId },
    #[error("The destination node({node_id}) for relationship({rel_id}) was not found!")]
    RelToNodeNotFound { rel_id: AgId, node_id: AgId },
    #[error("The model's root node({id}) was not found!")]
    RootNodeNotFound { id: AgId },
}

impl From<NmgFromBranchAgDtoError> for InternalApplicationError {
    fn from(value: NmgFromBranchAgDtoError) -> Self {
        Self::NmgFromBranchAgDto(value)
    }
}

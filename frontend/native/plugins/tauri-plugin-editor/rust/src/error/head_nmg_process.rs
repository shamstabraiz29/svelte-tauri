use super::{nmg_from_branch_ag_dto::NmgFromBranchAgDtoError, InternalApplicationError};

#[derive(Debug, thiserror::Error)]
pub enum HeadNmgProcessError {
    #[error(transparent)]
    NmgFromBranchAgDto(NmgFromBranchAgDtoError),
    #[error("Head Nmg write lock error!")]
    HeadNmgWriteLock,
}

impl From<HeadNmgProcessError> for InternalApplicationError {
    fn from(value: HeadNmgProcessError) -> Self {
        Self::HeadNmgProcess(value)
    }
}

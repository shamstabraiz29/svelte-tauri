mod head_nmg_process;
mod internal_application;
mod nmg_from_branch_ag_dto;

use serde::{Serialize, Serializer};
use specta::Type;

pub use self::{
    head_nmg_process::HeadNmgProcessError, internal_application::InternalApplicationError,
    nmg_from_branch_ag_dto::NmgFromBranchAgDtoError,
};

pub type Result<T> = std::result::Result<T, EditorError>;

#[derive(Debug, thiserror::Error, Type)]
pub enum EditorError {
    #[error("{0}")]
    RecoverableError(String),
    #[error("Internal application error!")]
    InternalApplicationError(String),
}

impl Serialize for EditorError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

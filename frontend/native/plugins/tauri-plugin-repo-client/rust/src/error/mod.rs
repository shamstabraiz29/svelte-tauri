use serde::{ser::Serializer, Serialize};
use specta::Type;

pub type Result<T> = std::result::Result<T, RepoError>;

#[derive(Debug, thiserror::Error, Type)]
pub enum RepoError {
    #[error("Repo error: {0}")]
    Repo(String),
}

impl From<common_libs_repo_api_client::Error> for RepoError {
    fn from(error: common_libs_repo_api_client::Error) -> Self {
        RepoError::Repo(error.to_string())
    }
}

impl Serialize for RepoError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

use serde::{ser::Serializer, Serialize};
use specta::Type;

pub type Result<T> = std::result::Result<T, AccountError>;

#[derive(Debug, thiserror::Error, Type)]
pub enum AccountError {
    #[error("IO error: {0}")]
    Account(String),
}

impl Serialize for AccountError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

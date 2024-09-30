use std::fmt::Display;

use serde::{ser::Serializer, Serialize};
use specta::Type;

#[derive(Debug, thiserror::Error, Type)]
pub enum LoginError {
    Io(String),
}

impl From<std::io::Error> for LoginError {
    fn from(e: std::io::Error) -> Self {
        LoginError::Io(e.to_string())
    }
}

impl Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginError::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl Serialize for LoginError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

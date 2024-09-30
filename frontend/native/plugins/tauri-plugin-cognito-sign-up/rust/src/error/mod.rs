// frontend/tauri/plugins/tauri-plugin-cognito-sign-up/rust/src/error/mod.rs

use serde::{ser::Serializer, Serialize};
use specta::Type;

pub type Result<T> = std::result::Result<T, SignUpError>;

#[derive(Debug, thiserror::Error, Type)]
pub enum SignUpError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Cognito error: {0}")]
    CognitoError(String),
    // #[cfg(mobile)]
    // #[error("Plugin invoke error: {0}")]
    // PluginInvoke(String),
}

impl From<std::io::Error> for SignUpError {
    fn from(error: std::io::Error) -> Self {
        SignUpError::Io(error.to_string())
    }
}

// Implement From for String to easily create ConfigError and CognitoError
impl From<String> for SignUpError {
    fn from(error: String) -> Self {
        SignUpError::CognitoError(error)
    }
}

// #[cfg(mobile)]
// impl From<tauri::plugin::mobile::PluginInvokeError> for SignUpError {
//     fn from(error: tauri::plugin::mobile::PluginInvokeError) -> Self {
//         SignUpError::PluginInvoke(error.to_string())
//     }
// }

impl Serialize for SignUpError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

use serde::Serialize;
use specta::Type;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Type, Serialize)]
pub struct Error {
    pub req_id: String,
    pub message: String,
}

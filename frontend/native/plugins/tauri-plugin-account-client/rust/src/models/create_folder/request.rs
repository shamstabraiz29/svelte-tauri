use common_simple_types::ag_id::AgId;
use serde::Deserialize;
use specta::Type;

#[derive(Debug, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub req_id: String,
    #[specta(type = String)]
    pub parent_folder_id: AgId,
}

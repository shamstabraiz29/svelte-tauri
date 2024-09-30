use common_dtos::commit_client::types::account::{
    account_dto::AccountAgDto, folder_dto::FolderDto, repo_meta_dto::RepoMetaDto,
};
use common_simple_types::ag_id::AgId;
use serde_json::Value as JsonValue;
use specta::Type;
use specta_util::Unknown;
use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Clone, Debug, Type, tauri_specta::Event)]
#[serde(tag = "type")]

pub enum AcctDetailEvent {
    #[serde(rename = "account")]
    Account {
        #[serde(rename = "accountDetail")]
        account_detail: AccountDetail,
    },
    Clear,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct AccountDetail {
    pub id: String,
    pub name: String,
    pub root_folder_id: String,
    pub root_folders: Vec<Folder>,
    pub root_repos: Vec<RepoMeta>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct Folder {
    pub id: String,
    pub name: String,

    #[specta(type = HashMap<String, Unknown>)]
    pub properties: HashMap<String, JsonValue>,
    pub folders: Vec<Folder>,
    pub repo_metas: Vec<RepoMeta>,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]

pub struct RepoMeta {
    #[specta(type = String)]
    pub id: AgId,
    pub name: String,
    pub archived: bool,

    #[specta(type = HashMap<String, Unknown>)]
    pub properties: HashMap<String, JsonValue>,
}

impl From<AccountAgDto> for AccountDetail {
    fn from(account: AccountAgDto) -> Self {
        let root_folders = account
            .folder_tree
            .root
            .children
            .values()
            .map(Folder::from)
            .collect();

        let root_repos = account
            .folder_tree
            .root
            .repo_refs
            .into_values()
            .map(RepoMeta::from)
            .collect();

        AccountDetail {
            id: account.id.to_string(),
            name: account.name,
            root_folder_id: account.folder_tree.root.id.to_string(),
            root_folders,
            root_repos,
        }
    }
}

impl From<&FolderDto> for Folder {
    fn from(folder: &FolderDto) -> Self {
        Folder {
            id: folder.id.to_string(),
            name: folder.name.to_string(),
            properties: folder.properties.clone().into_iter().collect(),
            folders: folder.children.values().map(Folder::from).collect(),
            repo_metas: folder
                .repo_refs
                .values()
                .map(RepoMeta::from)
                .collect::<Vec<RepoMeta>>(),
        }
    }
}

impl From<FolderDto> for Folder {
    fn from(folder_dto: FolderDto) -> Self {
        Folder {
            id: folder_dto.id.to_string(),
            name: folder_dto.name,
            properties: folder_dto.properties.into_iter().collect(),
            folders: folder_dto.children.values().map(Folder::from).collect(),
            repo_metas: folder_dto
                .repo_refs
                .into_values()
                .map(RepoMeta::from)
                .collect(),
        }
    }
}

impl From<&RepoMetaDto> for RepoMeta {
    fn from(repo_meta: &RepoMetaDto) -> Self {
        Self::from(repo_meta.clone())
    }
}

impl From<RepoMetaDto> for RepoMeta {
    fn from(repo_meta: RepoMetaDto) -> Self {
        RepoMeta {
            id: repo_meta.id,
            name: repo_meta.name,
            archived: repo_meta.archived,
            properties: repo_meta.properties.into_iter().collect(),
        }
    }
}

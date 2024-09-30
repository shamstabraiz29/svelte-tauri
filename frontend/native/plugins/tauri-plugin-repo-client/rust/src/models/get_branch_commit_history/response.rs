use common_dtos::commit_client::types::repo::commit_history::{
    AuthorDto, BranchCommitDto, RepoCommitHistoryPageDto,
};
use common_simple_types::{ag_id::AgId, commit_id::CommitId};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type", rename_all = "camelCase")]

pub struct RepoGetCommitHistoryResponse {
    pub history: RepoCommitHistoryPage,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RepoCommitHistoryPage {
    pub page_number: i32,
    #[specta(type=String)]
    pub repo_id: AgId,
    pub branch_infos: Vec<BranchInfo>,
    pub branches_commit_history: Vec<BranchCommitDetail>,
    pub next_page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct BranchInfo {
    #[specta(type=String)]
    branch_id: AgId,
    name: String,
}

impl From<RepoCommitHistoryPageDto> for RepoCommitHistoryPage {
    fn from(dto: RepoCommitHistoryPageDto) -> Self {
        Self {
            page_number: dto.page_number,
            repo_id: dto.repo_id,
            branch_infos: dto
                .branch_infos
                .into_iter()
                .map(|branch_info_dto| BranchInfo {
                    branch_id: branch_info_dto.branch_id,
                    name: branch_info_dto.name,
                })
                .collect(),
            branches_commit_history: dto
                .branches_commit_history
                .into_iter()
                .map(BranchCommitDetail::from)
                .collect(),
            next_page_token: dto.next_page_token,
        }
    }
}

pub fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = value.to_string();
    String::serialize(&s, serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<i64>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct BranchCommitDetail {
    #[specta(type=String)]
    pub branch_id: AgId,
    #[specta(type=String)]
    pub id: CommitId,
    pub op_type: OpType,
    pub message: String,
    pub author: CommitAuthorDetail,
    #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
    #[specta(type = String)]
    pub timestamp: i64,
}

impl From<BranchCommitDto> for BranchCommitDetail {
    fn from(dto: BranchCommitDto) -> Self {
        Self {
            branch_id: dto.branch_id,
            id: dto.id,
            op_type: dto.op_type.into(),
            message: dto.message,
            timestamp: dto.timestamp,
            author: dto.author.into(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CommitAuthorDetail {
    #[specta(type=String)]
    pub user_id: AgId,
    pub given_name: String,
    pub email: String,
}

impl From<AuthorDto> for CommitAuthorDetail {
    fn from(dto: AuthorDto) -> Self {
        Self {
            user_id: dto.user_id,
            given_name: dto.given_name,
            email: dto.email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[serde(tag = "type")]
pub enum OpType {
    Create,
    #[serde(rename_all = "camelCase")]
    Branch {
        #[specta(type=String)]
        from: AgId,
        #[specta(type=String)]
        at: CommitId,
    },
    Commit,
}

impl From<common_complex_types::commit_history::OpType> for OpType {
    fn from(dto: common_complex_types::commit_history::OpType) -> Self {
        match dto {
            common_complex_types::commit_history::OpType::Create => OpType::Create,
            common_complex_types::commit_history::OpType::Branch { from, at } => {
                OpType::Branch { from, at }
            }
            common_complex_types::commit_history::OpType::Commit => OpType::Commit,
        }
    }
}

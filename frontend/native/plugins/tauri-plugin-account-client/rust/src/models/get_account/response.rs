use common_dtos::commit_client::types::account::account_dto::AccountAgDto;
use serde::Serialize;
use specta::Type;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "type")]

pub struct AccountSummaryResponse {
    #[serde(rename = "reqId")]
    pub req_id: String,
    #[serde(rename = "acctSummary")]
    pub acct_summary: AccountSummary,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "type")]

pub struct AccountSummary {
    #[serde(rename = "acctId")]
    acct_id: String,
    #[serde(rename = "name")]
    name: String,
}

impl From<&AccountAgDto> for AccountSummary {
    fn from(dto: &AccountAgDto) -> Self {
        AccountSummary {
            acct_id: dto.id.to_string(),
            name: dto.name.to_string(),
        }
    }
}

impl From<AccountAgDto> for AccountSummary {
    fn from(dto: AccountAgDto) -> Self {
        Self::from(&dto)
    }
}

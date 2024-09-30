use common_dtos::commit_client::types::subscriber::subscriber_dto::SubscriberAgDto;
use serde::Serialize;
use serde_json::Value as JsonValue;
use specta::Type;
use specta_util::Unknown;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SubscriberResponse {
    pub req_id: String,
    pub subscriber_detail: SubscriberDetail,
}

#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SubscriberDetail {
    pub id: String,
    pub acct_ids: Vec<String>,
    #[specta(type = HashMap<String, Unknown>)]
    pub properties: HashMap<String, JsonValue>,
}

impl From<SubscriberAgDto> for SubscriberDetail {
    fn from(subscriber: SubscriberAgDto) -> Self {
        SubscriberDetail {
            id: subscriber.id.to_string(),
            acct_ids: subscriber
                .acct_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect(),
            properties: subscriber.properties,
        }
    }
}

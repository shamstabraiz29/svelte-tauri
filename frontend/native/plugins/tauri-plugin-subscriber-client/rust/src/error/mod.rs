// src/error/mod.rs
use serde::{Serialize, Serializer};
use specta::Type;

#[derive(Debug, thiserror::Error, Type)]
pub enum SubscriberError {
    #[error("Invalid state: Subscriber not initialized.")]
    NoSubscriber,
    #[error("Loaded subscriber lock is poisoned.")]
    LoadedSubscriberLock,
    #[error("Unloaded subscriber lock is poisoned.")]
    UnloadedSubscriberLock,
    #[error("Subscriber error: {0}")]
    Subscriber(String),
    #[error("CloudFront distribution error: {0}")]
    CloudFrontDistribution(String),
    #[error("Subscriber client DTO lock error.")]
    SubscriberClientDtoLock,
}

impl From<common_libs_subscriber_api_client::Error> for SubscriberError {
    fn from(error: common_libs_subscriber_api_client::Error) -> Self {
        SubscriberError::Subscriber(error.to_string())
    }
}

impl From<common_libs_subscriber_api_client::CloudFrontDistributionError> for SubscriberError {
    fn from(error: common_libs_subscriber_api_client::CloudFrontDistributionError) -> Self {
        SubscriberError::CloudFrontDistribution(error.to_string())
    }
}

impl Serialize for SubscriberError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

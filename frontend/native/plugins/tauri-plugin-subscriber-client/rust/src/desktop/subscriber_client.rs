// src/desktop/subscriber_client.rs
use std::sync::RwLock;

use common_dtos::commit_client::types::subscriber::subscriber_dto::SubscriberAgDto;
use common_libs_subscriber_api_client::SubscriberApiClient;
use log::{debug, error, info, warn};
use tauri::{AppHandle, Manager, Runtime, State};

use crate::SubscriberError;

/// Access to the subscriber-client APIs.
pub struct SubscriberClient<R: Runtime> {
    app: AppHandle<R>,
    dto: RwLock<Option<SubscriberAgDto>>,
}

impl<R: Runtime> SubscriberClient<R> {
    pub fn new(app: AppHandle<R>) -> Self {
        debug!("Creating new SubscriberClient");
        Self {
            app,
            dto: RwLock::new(None),
        }
    }

    pub async fn load(&self, access_token: &str) -> Result<SubscriberAgDto, SubscriberError> {
        info!("Loading subscriber data");
        let subscriber_api_client: State<'_, SubscriberApiClient> = self.app.state();

        debug!("Retrieved SubscriberApiClient state");

        let dto = match subscriber_api_client.load(access_token).await {
            Ok(dto) => {
                info!("Successfully loaded subscriber data");
                dto
            }
            Err(e) => {
                error!("Failed to load subscriber data: {:?}", e);
                return Err(e.into());
            }
        };

        match self.dto.write() {
            Ok(mut write_guard) => {
                debug!("Updating internal DTO");
                write_guard.replace(dto.clone());
            }
            Err(e) => {
                warn!("Failed to acquire write lock for DTO: {}", e);
                return Err(SubscriberError::SubscriberClientDtoLock);
            }
        }

        debug!("Returning loaded DTO");
        Ok(dto)
    }
}

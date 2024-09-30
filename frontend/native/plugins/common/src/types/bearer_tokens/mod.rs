// frontend/tauri/plugins/common/src/types/bearer_tokens/mod.rs
mod inner_tokens;

// DO NOT STORE THE ACCESS_TOKEN (in memory only)
// DO NOT STORE THE ID_TOKEN (in memory only)
// ***************************************************************
// ***************************************************************
// DO STORE THE REFRESH_TOKEN IN OS KEYCHAIN (or equiv) ONLY!!!!
// DO NOT SEND REFRESH_TOKEN to UI (or anywhere else)!!!!
// ***************************************************************
// ***************************************************************
// The access_token is used to authenticate API calls to AWS
// The id_token is used as signed proof of if user's identity
// The refresh_token is used to get new access_tokens and id_tokens

use std::{collections::HashMap, sync::Mutex};

use serde_json::Value;
use tauri::{AppHandle, Runtime};

use tauri_specta::Event;

use crate::events::bearer_token_event::BearerTokenEvent;

use self::inner_tokens::InnerBearerTokens;

pub struct BearerTokens<R: Runtime> {
    app: AppHandle<R>,
    bearer_tokens: Mutex<Option<InnerBearerTokens>>,
}

impl<R: Runtime> BearerTokens<R> {
    pub fn is_token_set(&self) -> bool {
        let lock = self.bearer_tokens.lock().unwrap();
        lock.is_some()
    }
}

fn from<R: Runtime>(bearer_tokens: &BearerTokens<R>) -> BearerTokenEvent {
    let access_token = bearer_tokens.access_token();
    let id_token = bearer_tokens.id_token();

    if access_token.is_none() || id_token.is_none() {
        return BearerTokenEvent::Clear;
    }

    let access_claims = bearer_tokens.access_token_claims();
    let id_claims = bearer_tokens.id_token_claims();

    BearerTokenEvent::Tokens {
        access_token: access_token.unwrap(),
        access_token_claims: access_claims.unwrap(),
        id_token: id_token.unwrap(),
        id_token_claims: id_claims.unwrap(),
    }
}

impl<R: Runtime> BearerTokens<R> {
    pub fn new(app: AppHandle<R>) -> Self {
        Self {
            app,
            bearer_tokens: Mutex::new(None),
        }
    }

    pub fn access_token(&self) -> Option<String> {
        match self.bearer_tokens.lock() {
            Ok(lock) => match lock.as_ref() {
                Some(bearer_tokens) => {
                    let token = bearer_tokens.access_token().to_string();
                    if token.trim().is_empty() {
                        None
                    } else {
                        Some(token)
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn access_token_claims(&self) -> Option<HashMap<String, Value>> {
        match self.bearer_tokens.lock() {
            Ok(lock) => lock
                .as_ref()
                .map(|bearer_tokens| bearer_tokens.access_token_claims().to_owned()),
            Err(_) => None,
        }
    }

    pub fn id_token(&self) -> Option<String> {
        match self.bearer_tokens.lock() {
            Ok(lock) => match lock.as_ref() {
                Some(bearer_tokens) => {
                    let token = bearer_tokens.id_token().to_string();
                    if token.trim().is_empty() {
                        None
                    } else {
                        Some(token)
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }

    pub fn id_token_claims(&self) -> Option<HashMap<String, Value>> {
        match self.bearer_tokens.lock() {
            Ok(lock) => lock
                .as_ref()
                .map(|bearer_tokens| bearer_tokens.id_token_claims().to_owned()),
            Err(_) => None,
        }
    }

    pub fn refresh_token(&self) -> Option<String> {
        match self.bearer_tokens.lock() {
            Ok(lock) => match lock.as_ref() {
                Some(bearer_tokens) => {
                    let token = bearer_tokens.refresh_token().to_string();
                    if token.trim().is_empty() {
                        None
                    } else {
                        Some(token)
                    }
                }
                None => None,
            },
            Err(_) => None,
        }
    }
    pub async fn set_tokens(
        &self,
        user_pool_id: &str,
        client_id: &str,
        region: &str,
        access_token: &str,
        id_token: &str,
        refresh_token: &str,
    ) {
        let inner_bearer_tokens = InnerBearerTokens::new(
            user_pool_id,
            client_id,
            region,
            access_token,
            id_token,
            refresh_token,
        )
        .await;

        {
            let mut lock = self.bearer_tokens.lock().unwrap();
            *lock = Some(inner_bearer_tokens);
        }
        self.emit_tokens().unwrap();
    }

    pub fn clear_tokens(&self) {
        {
            let mut lock = self.bearer_tokens.lock().unwrap();
            *lock = None;
        }
        self.emit_tokens().unwrap();
    }

    fn emit_tokens(&self) -> Result<(), tauri::Error> {
        let bearer_token_event = from(self);

        log::trace!("emitting bearer_token_event: {:?}", bearer_token_event);

        bearer_token_event.emit(&self.app)
    }
}

use serde_json::Value;
use std::collections::HashMap;
use zeroize::{Zeroize, ZeroizeOnDrop};

use verified_token::{
    verified_token::VerifiedToken,
    verifier_kind::{access_verifier::AccessTokenVerifierKind, id_verifier::IdTokenVerifierKind},
};

#[derive(Zeroize, ZeroizeOnDrop)]
pub(super) struct InnerBearerTokens {
    access_token: VerifiedToken<AccessTokenVerifierKind>,
    id_token: VerifiedToken<IdTokenVerifierKind>,
    refresh_token: String,
}

impl InnerBearerTokens {
    pub(super) async fn new(
        user_pool_id: &str,
        client_id: &str,
        region: &str,
        access_token: &str,
        id_token: &str,
        refresh_token: &str,
    ) -> Self {
        log::debug!("InnerBearerTokens::user_pool_id: {}", user_pool_id);
        log::debug!("InnerBearerTokens::client_id: {}", client_id);
        Self {
            access_token: VerifiedToken::new(access_token, user_pool_id, client_id, region)
                .await
                .unwrap(),
            id_token: VerifiedToken::new(id_token, user_pool_id, client_id, region)
                .await
                .unwrap(),
            refresh_token: refresh_token.to_string(),
        }
    }

    pub fn access_token(&self) -> &str {
        self.access_token.get_verified_token()
    }

    pub fn access_token_claims(&self) -> &HashMap<String, Value> {
        self.access_token.get_verified_claims()
    }

    pub fn id_token(&self) -> &str {
        self.id_token.get_verified_token()
    }

    pub fn id_token_claims(&self) -> &HashMap<String, Value> {
        self.id_token.get_verified_claims()
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }
}

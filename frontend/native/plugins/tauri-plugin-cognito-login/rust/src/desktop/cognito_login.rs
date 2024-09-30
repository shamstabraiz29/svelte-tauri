use tauri::{AppHandle, Runtime};

// Access to the cognito-login APIs.
pub struct CognitoLogin<R: Runtime>(pub AppHandle<R>);

impl<R: Runtime> CognitoLogin<R> {
    // pub fn access_token(&self) -> Option<String> {
    //     let bearer_tokens: State<'_, BearerTokens<R>> = self.0.state();

    //     bearer_tokens.access_token()
    // }

    // pub fn id_token(&self) -> Option<String> {
    //     let bearer_tokens: State<'_, BearerTokens<R>> = self.0.state();

    //     bearer_tokens.id_token()
    // }

    // // Error is if there is a system, configuration, communication error, etc.
    // // If all calls work but there user is not authorized, that should be return
    // // in the TokenRefreshResult
    // pub fn refresh_tokens(&self) -> Result<TokenRefreshResult, crate::Error> {
    //     let bearer_tokens: State<'_, BearerTokens<R>> = self.0.state();

    //     // if the bearer token is None, return None
    //     // if the bearer token is Some, return the id_token
    //     let refresh_token = bearer_tokens.refresh_token();

    //     let Some(_refresh_token) = refresh_token else {
    //         return Ok(TokenRefreshResult::AuthenticationRequired);
    //     };

    //     todo!("Implement refresh_tokens()")
    // }

    // This is more than just checking if the access_token is None
    // we need to look at expiration, etc.
    pub fn is_authenticated(&self) -> bool {
        todo!("Implement is_authenticated()")
    }
}

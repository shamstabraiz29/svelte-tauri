#[cfg(desktop)]
use desktop::cognito_login::CognitoLogin;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the cognito-login APIs.
pub trait CognitoLoginExt<R: Runtime> {
    fn cognito_login(&self) -> &CognitoLogin<R>;
}

impl<R: Runtime, T: Manager<R>> crate::CognitoLoginExt<R> for T {
    fn cognito_login(&self) -> &CognitoLogin<R> {
        self.state::<CognitoLogin<R>>().inner()
    }
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the cognito-sign-up APIs.
use tauri::{Manager, Runtime};

#[cfg(desktop)]
use crate::desktop::cognito_sign_up::CognitoSignUp;

pub trait CognitoSignUpExt<R: Runtime> {
    fn cognito_sign_up(&self) -> &CognitoSignUp<R>;
}

impl<R: Runtime, T: Manager<R>> CognitoSignUpExt<R> for T {
    fn cognito_sign_up(&self) -> &CognitoSignUp<R> {
        self.state::<CognitoSignUp<R>>().inner()
    }
}

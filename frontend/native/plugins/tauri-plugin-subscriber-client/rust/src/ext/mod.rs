// src/ext/mod.rs
use tauri::{Manager, Runtime};

#[cfg(desktop)]
use crate::desktop::subscriber_client::SubscriberClient;

pub trait SubscriberClientExt<R: Runtime> {
    fn subscriber_client(&self) -> &SubscriberClient<R>;
}

impl<R: Runtime, T: Manager<R>> SubscriberClientExt<R> for T {
    fn subscriber_client(&self) -> &SubscriberClient<R> {
        self.state::<SubscriberClient<R>>().inner()
    }
}

use std::{future::Future, pin::Pin, sync::Arc};

use super::observer::Observer;

pub trait Subject<P, E> {
    fn register_observer(&mut self, observer: Arc<dyn Observer<P, E>>) -> Result<(), E>;

    fn notify_observers(&self) -> Pin<Box<dyn Future<Output = Result<(), E>> + Send + '_>>;
}

use std::{future::Future, pin::Pin};

pub trait Observer<P, E>: Send + Sync {
    fn update(
        &self,
        old_value: P,
        new_value: P,
    ) -> Pin<Box<dyn Future<Output = Result<(), E>> + Send + '_>>;
}

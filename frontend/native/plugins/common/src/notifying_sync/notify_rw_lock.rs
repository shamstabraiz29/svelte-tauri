use std::fmt;
use std::ops::{Deref, DerefMut};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

//  FIXME *********************************************************************************************************
//  ***************************************************************************************************************
//  ****** This is placed here for quick experimentation and will be moved to the appropriate location later ******
//  ***************************************************************************************************************
//  ***************************************************************************************************************

type WriteCallback<T> = Box<dyn Fn(&T) + Send + Sync>;
type ReadCallback<T> = Box<dyn Fn(&T) + Send + Sync>;

pub struct NotifyingRwLock<T> {
    rwlock: RwLock<T>,
    write_callback: WriteCallback<T>,
    read_callback: Option<ReadCallback<T>>,
}

// This guard struct is specifically for write access
pub struct NotifyingWriteGuard<'a, T> {
    write_guard: RwLockWriteGuard<'a, T>,
    callback: &'a WriteCallback<T>,
}

impl<T> NotifyingRwLock<T> {
    // Updated constructor to accept an optional read callback
    pub fn new(
        data: T,
        write_callback: WriteCallback<T>,
        read_callback: Option<ReadCallback<T>>,
    ) -> Self {
        NotifyingRwLock {
            rwlock: RwLock::new(data),
            write_callback,
            read_callback,
        }
    }

    // Method to obtain a write lock
    pub fn write(&self) -> NotifyingWriteGuard<T> {
        let write_guard = self
            .rwlock
            .write()
            .expect("NotifyingWriteGuard: poisoned lock");
        NotifyingWriteGuard {
            write_guard,
            callback: &self.write_callback,
        }
    }

    // Method to obtain a read lock
    pub fn read(&self) -> NotifyingReadGuard<T> {
        let read_guard = self
            .rwlock
            .read()
            .expect("NotifyingReadGuard: poisoned lock");
        NotifyingReadGuard {
            read_guard,
            callback: self.read_callback.as_ref(),
        }
    }
}

impl<T> Deref for NotifyingWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.write_guard
    }
}

impl<T> DerefMut for NotifyingWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.write_guard
    }
}

impl<T> Drop for NotifyingWriteGuard<'_, T> {
    fn drop(&mut self) {
        (self.callback)(&*self.write_guard);
        log::debug!("Write lock released, notification sent.");
    }
}

impl<T: std::fmt::Debug> fmt::Debug for NotifyingRwLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotifyingRwLock")
            .field("rwlock", &self.rwlock)
            .field("write_callback", &"WriteCallback<T>")
            .field("read_callback", &"ReadCallback<T>")
            .finish()
    }
}

impl<'a, T: std::fmt::Debug> fmt::Debug for NotifyingWriteGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotifyingWriteGuard")
            .field("write_guard", &self.write_guard)
            .field("callback", &"WriteCallback<T>")
            .finish()
    }
}

impl<'a, T: std::fmt::Debug> fmt::Debug for NotifyingReadGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotifyingReadGuard")
            .field("read_guard", &self.read_guard)
            .field("callback", &"ReadCallback<T>")
            .finish()
    }
}

pub struct NotifyingReadGuard<'a, T> {
    read_guard: RwLockReadGuard<'a, T>,
    callback: Option<&'a ReadCallback<T>>,
}

impl<'a, T> Deref for NotifyingReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.read_guard
    }
}

impl<'a, T> Drop for NotifyingReadGuard<'a, T> {
    fn drop(&mut self) {
        if let Some(callback) = self.callback {
            callback(&*self.read_guard); // Call the read notification callback with the guarded data
            log::debug!("Read lock released, notification sent.");
        }
    }
}

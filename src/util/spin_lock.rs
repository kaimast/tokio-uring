use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

pub(crate) struct SpinLock<T> {
    inner: UnsafeCell<T>,
    is_locked: AtomicBool,
}

pub(crate) struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    pub fn new(inner: T) -> Self {
        Self{
            is_locked: AtomicBool::new(false),
            inner: UnsafeCell::new(inner),
        }
    }

    pub fn lock<'a>(&'a self) -> SpinLockGuard<'a, T> {
        loop {
            match self.is_locked.compare_exchange(false, true, SeqCst, SeqCst) {
                Ok(_) => {
                    return SpinLockGuard{ lock: self };
                },
                Err(_) => {},
            }
        }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.is_locked.store(false, SeqCst);
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{ &*self.lock.inner.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{ &mut *self.lock.inner.get() }
    }
}

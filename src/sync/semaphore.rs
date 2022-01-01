use super::batch_semaphore as ll; // low level implementation
use super::{AcquireError, TryAcquireError};

#[derive(Debug)]
pub struct Semaphore {
    /// The low level semaphore
    ll_sem: ll::Semaphore,
}

/// A permit from the semaphore.
///
/// This type is created by the [`acquire`] method.
///
/// [`acquire`]: crate::sync::Semaphore::acquire()
#[must_use]
#[derive(Debug)]
pub struct SemaphorePermit<'a> {
    sem: &'a Semaphore,
    permits: u32,
}

impl Semaphore {
    /// Creates a new semaphore with the initial number of permits.
    #[track_caller]
    pub fn new(permits: usize) -> Self {
        let ll_sem = ll::Semaphore::new(permits);

        Self {
            ll_sem,
        }
    }

    /// Returns the current number of available permits.
    pub fn available_permits(&self) -> usize {
        self.ll_sem.available_permits()
    }

    /// Adds `n` new permits to the semaphore.
    ///
    /// The maximum number of permits is `usize::MAX >> 3`, and this function will panic if the limit is exceeded.
    pub fn add_permits(&self, n: usize) {
        self.ll_sem.release(n);
    }

    /// Acquires a permit from the semaphore.
    pub async fn acquire(&self) -> Result<SemaphorePermit<'_>, AcquireError> {
        let inner = self.ll_sem.acquire(1);

        inner.await?;
        Ok(SemaphorePermit {
            sem: self,
            permits: 1,
        })
    }

    pub fn try_acquire(&self) -> Result<SemaphorePermit<'_>, TryAcquireError> {
        match self.ll_sem.try_acquire(1) {
            Ok(_) => Ok(SemaphorePermit {
                sem: self,
                permits: 1,
            }),
            Err(e) => Err(e),
        }
    }
}

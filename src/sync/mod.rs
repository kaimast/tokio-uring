mod semaphore;
pub use semaphore::Semaphore;

pub(crate) mod batch_semaphore;
pub use batch_semaphore::{AcquireError, TryAcquireError};

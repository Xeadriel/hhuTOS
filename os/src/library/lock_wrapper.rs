use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::{devices::cga::CGA, kernel::threads::scheduler::{get_scheduler, unlock_scheduler}, library::{mutex::{Mutex, MutexGuard}}};

// this method tracks which thread owns the lock
// this cant normally be checked 
pub struct LockWrapper<T> {
    lock: Mutex<T>,           // actual locking primitive
    owner: AtomicUsize,        // thread ID of owner
}

impl<T> LockWrapper<T> {
    pub const fn new(c: T) -> Self {
        LockWrapper {
            lock: Mutex::new(c),
            owner: AtomicUsize::new(0),       
        }
    }

    /// Lock the CGA and record the owning thread ID.
    pub fn lock(&self) -> MutexGuard<T> {
        let lock = self.lock.lock();

        let thread_id = get_scheduler().get_active_tid();

        self.owner.store(thread_id, Ordering::Release);

        return lock;
    }

    /// Try to acquire the lock only if it is not already locked.
    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        if let Some(guard) = self.lock.try_lock() {

            let thread_id = get_scheduler().get_active_tid();

            self.owner.store(thread_id, Ordering::Release);

            return Some(guard);
        }

        None
    }

    /// Forcefully release the T, regardless of who owns it.
    pub fn force_unlock(&self) {
        self.owner.store(0, Ordering::Release);
        unsafe {
            self.lock.force_unlock();
        }
    }

    /// Check whether the given thread owns the T lock.
    pub fn is_held_by(&self, thread_id: usize) -> bool {
        self.owner.load(Ordering::Acquire) == thread_id
    }

    /// Check if the wait queue is currently locked.
    pub fn is_queue_locked(&self) -> bool {
        self.lock.is_queue_locked()
    }

    /// Check whether T is currently locked.
    pub fn is_locked(&self) -> bool {
        self.lock.is_locked()
    }
}

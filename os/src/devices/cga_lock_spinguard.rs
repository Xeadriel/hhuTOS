use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use spin::{Mutex, MutexGuard};
use crate::{devices::cga::CGA, kernel::threads::scheduler::{get_scheduler, unlock_scheduler}, library::spinlock::{Spinlock, SpinlockGuard}};

pub struct CgaLock {
    cga: Spinlock<CGA>,           // actual locking primitive
    owner: AtomicUsize,        // thread ID of owner
}

impl CgaLock {
    pub const fn new(c: CGA) -> Self {
        CgaLock {
            cga: Spinlock::new(c),
            owner: AtomicUsize::new(0),       
        }
    }

    /// Lock the CGA and record the owning thread ID.
    pub fn lock(&self) -> SpinlockGuard<CGA> {
        let cga = self.cga.lock();

        let thread_id = get_scheduler().get_active_tid();

        self.owner.store(thread_id, Ordering::Release);

        return cga;
    }

    /// Try to acquire the lock only if it is not already locked.
    pub fn try_lock(&self) -> Option<SpinlockGuard<CGA>> {
        if let Some(guard) = self.cga.try_lock() {

            let thread_id = get_scheduler().get_active_tid();

            self.owner.store(thread_id, Ordering::Release);

            return Some(guard);
        }

        None
    }

    /// Forcefully release the CGA, regardless of who owns it.
    pub fn force_unlock(&self) {
        self.owner.store(0, Ordering::Release);
        unsafe {
            self.cga.force_unlock();
        }
    }

    /// Check whether the given thread owns the CGA lock.
    pub fn is_held_by(&self, thread_id: usize) -> bool {
        self.owner.load(Ordering::Acquire) == thread_id
    }

    /// Check whether CGA is currently locked.
    pub fn is_locked(&self) -> bool {
        self.cga.is_locked()
    }
}

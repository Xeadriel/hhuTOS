use alloc::boxed::Box;
use core::arch::asm;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::ptr;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use crate::kernel::cpu;
use crate::kernel::threads::scheduler::{get_scheduler, unlock_scheduler, SCHEDULER};
use crate::kernel::threads::thread::Thread;
use crate::library::queue::LinkedQueue;
use crate::library::spinlock::Spinlock;


/// A more sophisticated lock implementation than `Spinlock`, that blocks waiting threads
/// when the lock is already held. This improves performance, as no time is wasted by threads
/// spinning in a loop while waiting for the lock to be released.
pub struct Mutex<T> {
    /// The lock is represented by an atomic boolean that indicates whether the lock is held.
    lock: AtomicBool,
    /// The data protected by the mutex, stored in an `UnsafeCell` to allow mutable access.
    /// See `Spinlock` for more details on why we use `UnsafeCell`.
    data: UnsafeCell<T>,
    /// A queue of threads waiting for the lock to be released.
    wait_queue: Spinlock<LinkedQueue<Box<Thread>>>,
    owner: AtomicUsize, // Thread ID of the current owner (0 means unlocked)
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}
unsafe impl<T> Send for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
            wait_queue: Spinlock::new(LinkedQueue::new()),
            owner: AtomicUsize::new(0),
        }
    }
    
    /// Try to acquire the lock once without blocking.
    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        // Attempt to acquire the lock using atomic compare-and-exchange.
        if self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            let tid = get_scheduler().get_active_tid();
            self.owner.store(tid, Ordering::Release);
            Some(MutexGuard { lock: self })
        } else {
            None
        }
    }

    /// Try to acquire the lock once without blocking.
    pub fn try_lock_no_sched(&self) -> Option<MutexGuard<T>> {
        // Attempt to acquire the lock using atomic compare-and-exchange.
        if self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            Some(MutexGuard { lock: self })
        } else {
            None
        }
    }
    
    /// Acquire the lock, blocking if necessary until it is available.
    /// This method will dequeue the current thread from the scheduler if the lock is already held
    /// and store it in the `wait_queue`.
    /// Once the lock is available, the next thread in the `wait_queue` will be woken up
    /// so it can try to acquire the lock again.
    pub fn lock(&self) -> MutexGuard<T> {
        // has been initialized?
        if !SCHEDULER.is_completed() {
            while self.try_lock_no_sched().is_none() {
                unsafe {
                    asm!("pause", options(nomem, nostack, preserves_flags));
                }
            }
            return MutexGuard { lock: self };
        }

        loop {
            unsafe {
                unlock_scheduler();
            }
            let scheduler = get_scheduler();
            // if scheduler not initialized, behave like spinlock

            if let Some(guard) = self.try_lock() {
                return guard;
            }
            
            
            
            let (mut thread, interrupts_enabled) = scheduler.prepare_block();
            let thread_ptr: *mut Thread = thread.as_mut();
            self.wait_queue.lock().enqueue(thread);
            unsafe {
                scheduler.switch_from_blocked_thread(thread_ptr, interrupts_enabled);
            }
        }
    }
    
    pub fn is_locked(&self) -> bool {
        self.lock.load(Ordering::Relaxed)
    }
    
    pub fn is_queue_locked(&self) -> bool {
        self.wait_queue.is_locked()
    }
    
    /// Unlock the mutex, allowing other threads to acquire it.
    /// If there are threads waiting for the lock, the next thread in the wait queue is woken up.
    pub fn unlock(&self) {
        self.owner.store(0, Ordering::Release); // Clear owner
        self.lock.store(false, Ordering::Release);
        
        if !SCHEDULER.is_completed() {
            return;
        }

        unsafe {
            unlock_scheduler();
        }
        
        let scheduler = get_scheduler();
        
        let maybe_thread = {
            let mut queue = self.wait_queue.lock();
            queue.dequeue()
        };
        
        if let Some(thread) = maybe_thread {
            get_scheduler().ready(thread);
        }
    }
    
    /// Forcefully unlock the mutex without waking up any waiting threads.
    /// This should only be used in exceptional cases.
    pub unsafe fn force_unlock(&self) {
        self.owner.store(0, Ordering::Release); // Clear owner
        self.lock.store(false, Ordering::Release);
    }
    
    /// Get the thread ID of the current lock holder.
    pub fn owner(&self) -> usize {
        self.owner.load(Ordering::Acquire)
    }
    
    /// Check if the given thread ID is currently holding the lock.
    pub fn is_held_by(&self, tid: usize) -> bool {
        self.owner() == tid
    }
}


/// A guard that provides access to the data protected by the mutex.
/// It implements `Deref` and `DerefMut` to allow transparent access to the data.
/// It also implements `Drop` to automatically unlock the mutex when it goes out of scope.
pub struct MutexGuard<'a, T> {
    lock: &'a Mutex<T>
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            self.lock.data.get().as_ref().unwrap()
        }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            self.lock.data.get().as_mut().unwrap()
        }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.unlock();
    }
}

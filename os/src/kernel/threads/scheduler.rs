/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: scheduler                                                       ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: A basic round-robin scheduler for cooperative threads.          ║
   ║         No priorities supported.                                        ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Autor:  Michael Schoettner, 15.05.2023                                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt::Display;
use core::{fmt, ptr};
use core::sync::atomic::AtomicUsize;
use spin::{Mutex, Once};
use crate::devices::cga::CGA;
use crate::kernel::{allocator, cpu};
use crate::kernel::interrupts::intdispatcher::INT_VECTORS;
use crate::kernel::threads::idle_thread::idle_thread;
use crate::kernel::threads::thread;
use crate::kernel::threads::thread::Thread;
use crate::library::queue::LinkedQueue;

/// Global scheduler instance
static SCHEDULER: Once<Scheduler> = Once::new();

/// Global access to the scheduler.
pub fn get_scheduler() -> &'static Scheduler {
    SCHEDULER.call_once(|| { Scheduler::new() })
}

/// Unlock the scheduler state.
/// This function is called from assembly code.
/// Usually, the mutex would be unlocked automatically when going out of scope.
/// However, since we switch to a different thread in `yield_cpu()` and `exit()`,
/// the scope is not left and the mutex remains locked.
/// As a workaround, we provide this function to unlock the scheduler manually.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unlock_scheduler() {
    unsafe {
        get_scheduler().state.force_unlock();
    }
}

/// The state of the scheduler.
/// It contains the active thread and the ready queue with all other threads.
/// The state is contained in its own struct so that it can be locked via a mutex.
struct SchedulerState {
    active_thread: Option<Box<Thread>>,
    ready_queue: LinkedQueue<Box<Thread>>,
    initialized: bool,
}

/// Represents the scheduler.
/// It is round-robin-based and uses a queue to manage the threads.
pub struct Scheduler {
    state: Mutex<SchedulerState>,
    active_thread_id: AtomicUsize,
}

impl Scheduler {
    /// Create a new scheduler instance with an empty ready queue
    /// and an idle thread as the active thread.
    pub fn new() -> Self {
        let state = SchedulerState {
            active_thread: Some(Thread::new(idle_thread)),
            ready_queue: LinkedQueue::new(),
            initialized: false,
        };
        
        Scheduler { state:  Mutex::new(state), active_thread_id: AtomicUsize::new(0)}
    }

    pub fn is_locked(&self) -> bool {
        self.state.is_locked()
    }

    pub fn is_initialized(&self) -> bool {
        self.state.lock().initialized
    }

    /// Prepare the current thread for blocking.
    /// This functions disables interrupts and return the current thread,
    /// as well as the return value from `cpu::disable_int_nested()`.
    /// To complete the blocking operation call `switch_from_blocked_thread()`,
    /// which will enable interrupts again and resume the scheduler.
    pub fn prepare_block(&self) -> (Box<Thread>, bool) {
        (self.state.lock().active_thread.take().unwrap(), cpu::disable_int_nested())
    }

    /// Complete a blocking operation begun with `prepare_block()`.
    /// This resumes the scheduler and switches to the next thread in the ready queue.
    pub unsafe fn switch_from_blocked_thread(&self, blocked_thread: *mut Thread, interrupts_enabled: bool) {
        let mut state = self.state.lock();

        // Dequeue next thread from the ready queue
        let mut next = state.ready_queue.dequeue().unwrap();
        let next_ptr: *mut Thread = next.as_mut();

        // Store next as the active thread
        self.active_thread_id.store(next.get_id(), core::sync::atomic::Ordering::SeqCst);
        state.active_thread = Some(next);

        // Thread switch: from blocked to next
        unsafe {
            Thread::switch(blocked_thread, next_ptr);
        }

        // After switch: restore interrupt state
        cpu::enable_int_nested(interrupts_enabled);

    }

    /// Get the ID of the currently active thread.
    pub fn get_active_tid(&self) -> usize {
        self.active_thread_id.load(core::sync::atomic::Ordering::SeqCst)
    }

    /// Start the scheduler.
    /// This function must only be called once.
    pub fn schedule(&self) {
        let mut state = self.state.lock();

        state.initialized = true;
        // The active thread is never None, since we must at least have the idle thread.
        state.active_thread.as_mut().unwrap().start();
    }

    /// Register a new thread in the ready queue.
    pub fn ready(&self, thread: Box<Thread>) {
        let mut state = self.state.lock();
        state.ready_queue.enqueue(thread);
    }

    /// Terminate the current (calling) thread and switch to the next one.
    pub fn exit(&self) {
        if CGA.is_held_by(self.get_active_tid()){
            unsafe { CGA.force_unlock() };
        }
        
        let mut state = self.state.lock();

        // The active thread is never None, since we must at least have the idle thread.
        let mut current = state.active_thread.take().unwrap();
        // The idle thread never exits, so there must be at least one thread in the queue.
        let next = state.ready_queue.dequeue().unwrap();
            
        // Set the dequeued thread as the active thread,
        // overwriting the current one, which we want to exit.
        self.active_thread_id.store(next.get_id(), core::sync::atomic::Ordering::SeqCst);
        state.active_thread = Some(next);
        

        unsafe {
            // Switch to the next thread.
            // `current` still contains the old thread we want to exit,
            // while `state.active_thread` contains the next one.
            Thread::switch(current.as_mut(), state.active_thread.as_mut().unwrap().as_mut());
        }
    }

    /// Yield the CPU and switch to the next thread in the ready queue.
    pub fn yield_cpu(&self) {
        if allocator::is_locked() {
            // kprintln!("alloc fail");
            return;
        }
        
        unsafe {
            unlock_scheduler();
        }
        
        let mut state = self.state.lock();

        // Only yield if the scheduler has already started
        if !state.initialized {
            // kprintln!("not init state");
            return;
        }

        // sometimes active thread is temporarily None i dont know why
        let Some(mut current) = state.active_thread.take() else {
            return;
        };

        let current_ptr: *mut Thread = current.as_mut();

        // Enqueue the current thread
        state.ready_queue.enqueue(current);

        // Dequeue the next thread
        let mut next = state.ready_queue.dequeue().unwrap();
        let next_ptr: *mut Thread = next.as_mut();
        
        // No-op if the thread is switching to itself
        self.active_thread_id.store(next.get_id(), core::sync::atomic::Ordering::SeqCst);
        state.active_thread = Some(next);
        
        if current_ptr == next_ptr {
            return;
        }

        unsafe {
            Thread::switch(current_ptr, next_ptr);
        }
        
    }




    /// Kill the thread with the given ID by removing it from the ready queue.
    pub fn kill(&self, to_kill_id: usize) {

        let mut state = self.state.lock();

        state.ready_queue.remove(|thread| thread.get_id() == to_kill_id);
        
        if CGA.is_held_by(to_kill_id){
            unsafe { CGA.force_unlock() };
        }
    }
}

impl Display for Scheduler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = self.state.lock();
        let active = state.active_thread.as_ref().unwrap();
        
        write!(f, "active: {}, ready: {}", active, state.ready_queue)
    }
}

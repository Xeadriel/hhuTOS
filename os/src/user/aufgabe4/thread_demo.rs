use crate::devices::cga::{self, CGA};
use crate::devices::{pcspk, pit};
use crate::kernel::cpu;
use crate::kernel::interrupts::pic::{self, Irq};
use crate::kernel::threads::scheduler::{self, get_scheduler, unlock_scheduler, Scheduler};
use crate::kernel::threads::thread::{self, Thread};

fn thread_entry() {

    let mut i = 0;
    let tid = get_scheduler().get_active_tid();
    loop {
        // kprintln!("Thread {}: Counter: {}", tid, i);
        // if let Some(mut cga) = CGA.try_lock() {
        {
            let mut cga = CGA.lock();
            let old_pos = cga.getpos();
            cga.setpos(0, tid);

            print_cga!(&mut cga, "Thread {}: Counter: {}", tid, i);
            cga.setpos(old_pos.0, old_pos.1);
        }
        // }
        
        if (i) % 10 == 0 {
            get_scheduler().yield_cpu();
        }
        
        if tid == 1 {
            if i == 1000 {
                get_scheduler().kill(2)
            } else if i == 2000 {
                get_scheduler().kill(3)
            } else if i == 10000 {
                get_scheduler().exit();
            }
        }
        
        i+=1;
    }

}

pub fn run() {

    get_scheduler().ready(Thread::new(thread_entry));
    get_scheduler().ready(Thread::new(thread_entry));
    get_scheduler().ready(Thread::new(thread_entry));
    get_scheduler().ready(Thread::new(pcspk::zelda));

}
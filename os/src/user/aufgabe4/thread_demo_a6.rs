use crate::devices::cga::{self, CGA};
use crate::devices::{pcspk, pit};
use crate::kernel::cpu;
use crate::kernel::interrupts::pic::{self, Irq};
use crate::kernel::threads::scheduler::{self, get_scheduler, unlock_scheduler, Scheduler};
use crate::kernel::threads::thread::{self, Thread};

fn thread_entry() {

    let mut i = 0;
    let tid = get_scheduler().get_active_tid();

    let start = pit::get_system_time();

    loop {
        {
            let mut cga = CGA.lock();
            let (x, y) = cga.getpos();
            cga.setpos(0, tid);

            print_cga!(&mut cga, "Thread {}: Counter: {}", tid, i);
            cga.setpos(x, y);
        }
        
        if (i) % 10 == 0 {
            get_scheduler().yield_cpu();
        }
        
        if i == 10000 {
            let end = pit::get_system_time();
            
            {
                let mut cga = CGA.lock();
                let (x, y) = cga.getpos();
                cga.setpos(0, tid+5);

                print_cga!(&mut cga, "Thread {} completed time spent: {}", tid, end - start);
                cga.setpos(x, y);
            }

            get_scheduler().exit()
        }
        
        i+=1;
    }

}

pub fn run() {
    get_scheduler().ready(Thread::new(thread_entry));
    get_scheduler().ready(Thread::new(thread_entry));
    get_scheduler().ready(Thread::new(thread_entry));
}
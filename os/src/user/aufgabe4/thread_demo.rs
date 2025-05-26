use crate::devices::cga;
use crate::kernel::threads::scheduler::{get_scheduler, Scheduler};
use crate::kernel::threads::thread::{self, Thread};

fn thread_entry() {

    let mut i = 0;
    let tid = get_scheduler().get_active_tid();
    loop {

        cga::CGA.lock().setpos(0, tid);

        print!("Thread {}:, Counter: {}", tid, i);
        
        if tid == 2 {
            if i == 1000 {
                kprintln!("Thread 3 kill squad");
                get_scheduler().kill(3)
            } else if i == 2000 {
                kprintln!("Thread 4 kill squad");
                get_scheduler().kill(4)
            } else if i == 3000 {
                kprintln!("suicide squad");
                get_scheduler().exit();
            }
        }

        i+=1;
        get_scheduler().yield_cpu();
    }

}

pub fn run() {

    get_scheduler().ready(Thread::new(thread_entry));
    get_scheduler().ready(Thread::new(thread_entry));
    get_scheduler().ready(Thread::new(thread_entry));

}
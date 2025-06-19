use crate::kernel::threads::{scheduler::get_scheduler, thread::Thread};

pub fn hello_world() {
    println!("Hello world from a thread!");
    get_scheduler().exit();
}
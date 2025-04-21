use alloc::boxed::Box;
use alloc::vec;

use crate::kernel::allocator;

pub fn run () {

    allocator::dump_free_list();

    let x = Box::new(1); // allocate memory on the heap
    allocator::dump_free_list();

    drop(x); // deallocate memory on the heap
    allocator::dump_free_list();

    // let x = Box::new([0; 10]); // allocate memory on the heap
    // allocator::dump_free_list();

    // let x = Box::new([0; 15000]); // allocate memory on the heap
    // allocator::dump_free_list();

    // let x = vec![0; 247133]; // allocate memory on the heap
    // allocator::dump_free_list();

    // let x = vec![0; 700000]; // allocate memory on the heap
    // allocator::dump_free_list();
}

/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: startup                                                         ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Here is the main function called first from the boot code as    ║
   ║         well as the panic handler. All features are set and all modules ║
   ║         are imported.                                                   ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoettner, Univ. Duesseldorf, 5.2.2024                 ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
#![no_std]
#![allow(dead_code)] // avoid warnings
#![allow(unused_variables)] // avoid warnings
#![allow(unused_imports)]
#![allow(unused_macros)]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]

extern crate alloc;
extern crate spin; // we need a mutex in devices::cga_print

// insert other modules
#[macro_use] // import macros, too
mod devices;
mod kernel;
mod user;
mod consts;
mod library;

use core::arch::asm;
use core::panic::PanicInfo;

use devices::cga; // shortcut for cga
use devices::cga_print; // used to import code needed by println! 
use devices::keyboard; // shortcut for keyboard

use devices::kprint;
use kernel::cpu;

use kernel::interrupts::idt;
use kernel::interrupts::intdispatcher;
use kernel::interrupts::pic::PIC;
use kernel::threads::thread;
use kernel::threads::scheduler::get_scheduler;
use library::input::getch;
use user::aufgabe1::text_demo;
use user::aufgabe1::keyboard_demo;

use kernel::allocator;
use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;
use user::aufgabe4::coroutine_demo;
use user::aufgabe4::hello_world_thread;
use user::aufgabe4::thread_demo;

use crate::devices::pit;
use crate::user::aufgabe4::thread_demo_a6;

fn aufgabe1() {
    text_demo::run();
    println!("\nNow it's time to test the keyboard.");
    keyboard_demo::run();
}

fn aufgabe2() {
    heap_demo::run();
    println!("");
    sound_demo::run();
}

fn aufgabe3() {
    devices::pcspk::zelda();
    println!("Speaker test successful.");
    println!("Now it's time to test the interrupt-based keyboard. press return to to exit loop");
    loop {
        let c = getch();   
        if c == '\n' {
            break;
        }
        print!("{}", c);
    }
    println!("")
}

fn aufgabe4() {
    println!("");
    println!("");
    println!("");
    println!("");
    library::queue::test_linked_queue();
    println!("Linked Queue test successful.");

    coroutine_demo::run();
    println!("Coroutine test successful.");
}

fn aufgabe4_part2() {
    // get_scheduler().ready(thread::Thread::new(hello_world_thread::hello_world));
    
    thread_demo::run();
}

fn aufgabe6() {
    // counting up to 10000
    //      - using the spinlock method takes ~10000 ticks
    //      - using the new mutex method takes ~3400 ticks
    //      - using the old mutex method takes ~10000 ticks
    // the new method is awesome
    thread_demo_a6::run();
}
    

#[unsafe(no_mangle)]
pub extern "C" fn startup() {
    allocator::init();
    kprintln!("Heap Allocator initialized.");

    PIC.lock().init();
    kprintln!("Programmable Interrupt Controller initialized.");

    idt::get_idt().load();
    kprintln!("Interrupt Descriptor Table loaded.");

    intdispatcher::INT_VECTORS.lock().init();
    kprintln!("Interrupt Dispatcher INT_VECTORS initialized.");

    cga::CGA.lock().clear();
    cga::CGA.lock().enable_cursor();
    kprintln!("CGA cleared and ready.");

    keyboard::plugin();
    kprintln!("Keyboard plugged in.");
    
    pit::plugin();
    kprintln!("PIT initialized.");
    
    cpu::enable_int();
    kprintln!("Interrupts enabled.");
    
    // aufgabe4_part2();
    aufgabe6();
    
    get_scheduler().schedule();
    

    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    //	kprintln!("{:?}", Backtrace::new());
    loop {}
}


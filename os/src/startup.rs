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

use kernel::cpu;

use kernel::interrupts::idt;
use kernel::interrupts::intdispatcher;
use kernel::interrupts::pic::PIC;
use user::aufgabe1::text_demo;
use user::aufgabe1::keyboard_demo;

use kernel::allocator;
use user::aufgabe2::heap_demo;
use user::aufgabe2::sound_demo;

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
    
    cpu::enable_int();
    kprintln!("Interrupts enabled.");
    
    // unsafe {
    //     asm!(
    //         "INT 100" 
    //     );
    // }
    // aufgabe1();
    
    // Speicherverwaltung initialisieren

    // aufgabe2();

    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    //	kprintln!("{:?}", Backtrace::new());
    loop {}
}


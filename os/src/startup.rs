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

use crate::devices::lfb::init_lfb;
use crate::devices::pci::get_pci_bus;
use crate::devices::pci::Command;
use crate::devices::pit;
use crate::kernel::cpu::IoPort;
use crate::kernel::multiboot::FramebufferType;
use crate::kernel::multiboot::MultibootInfo;
use crate::kernel::threads::scheduler;
use crate::user::aufgabe4::thread_demo_a6;
use crate::user::aufgabe7::graphic_demo;
use crate::user::text_mode_demo;
use crate::user::the_lost_sword::the_lost_sword;

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
pub extern "C" fn startup(multiboot_info: &MultibootInfo) {
    
    /* Hier steht der existierende startup() Code bis `cpu::enable_interrupts()` */
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

    kprintln!("Scanning PCI bus");
    for device in get_pci_bus().iter() {
        kprintln!("Found PCI device {:04x}:{:04x}", device.read_vendor_id(), device.read_device_id());
    }

    // Just a short demo to show how to access PCI devices
    // For more information, see the OsDev Wiki: https://wiki.osdev.org/PCI, https://wiki.osdev.org/RTL8139
    let rtl8139 = get_pci_bus().iter().find(|device| {
        device.read_vendor_id() == 0x10ec && device.read_device_id() == 0x8139
    });

    if let Some(rtl8139) = rtl8139 {
        kprintln!("Found Realtek RTL8139 network controller");

        // Read the I/O base address from BAR0
        let bar0 = rtl8139.read_bar(0);
        if bar0 & 0x1 == 0 {
            // The address in BAR0 is a 32-bit memory-mapped I/O address.
            // This means that the registers are accessed via memory addresses instead of I/O ports.
            // The card emulated by QEMU uses 16-bit I/O ports,
            // so this code path is never executed in QEMU and is just here as a showcase.
            let mmio_base = bar0 & 0xfffffff0;
            kprintln!("RTL8139 MMIO base address: 0x{:x}", mmio_base);

            // Enable MMIO access by setting the correct command bits in the PCI command register
            rtl8139.write_command(rtl8139.read_command() | Command::MemEnable as u16);

            // Read mac address from the RTL8139 registers -> Always at offset 0x00-0x05
            // MMIO access is done via volatile reads to ensure the compiler does not optimize them away
            let mac_address_ptr = (mmio_base) as *const u8;
            let mac_address = unsafe {[
                mac_address_ptr.add(0).read_volatile(),
                mac_address_ptr.add(1).read_volatile(),
                mac_address_ptr.add(2).read_volatile(),
                mac_address_ptr.add(3).read_volatile(),
                mac_address_ptr.add(4).read_volatile(),
                mac_address_ptr.add(5).read_volatile()
            ]};
            kprintln!("MAC address: {:x?}", mac_address);
        } else {
            // The address in BAR0 is a 16-bit I/O port address
            let io_base = (bar0 & 0xfffc) as u16;
            kprintln!("RTL8139 I/O base address: 0x{:x}", io_base);

            // Enable I/O access by setting the correct command bits in the PCI command register
            rtl8139.write_command(rtl8139.read_command() | Command::IoEnable as u16);

            // Read mac address from the RTL8139 registers -> Always at offset 0x00-0x05
            let mac_address = unsafe {[
                IoPort::new(io_base + 0).inb(),
                IoPort::new(io_base + 1).inb(),
                IoPort::new(io_base + 2).inb(),
                IoPort::new(io_base + 3).inb(),
                IoPort::new(io_base + 4).inb(),
                IoPort::new(io_base + 5).inb()
            ]};
            kprintln!("MAC address: {:x?}", mac_address);
        }
    }


    // Check the framebuffer type and either show the CGA menu or initialize the linear framebuffer (LFB)
    if let Some(framebuffer_info) = multiboot_info.get_framebuffer_info() {
        match framebuffer_info.typ {
            FramebufferType::Indexed => {
                panic!("Color palette framebuffer not supported!");
            }
            FramebufferType::RGB => {
                init_lfb(
                    framebuffer_info.addr as *mut u8,
                    framebuffer_info.pitch,
                    framebuffer_info.width,
                    framebuffer_info.height,
                    framebuffer_info.bpp
                );
                the_lost_sword::run();
            }
            FramebufferType::Text => {
                /* Hier können Sie ihren existierenden Code, der auf dem CGA-Modus basiert aufrufen */
                 
                text_mode_demo::text_mode_demo::run();
            }
        }
    } else {
        // No framebuffer info available -> Probably CGA mode

        /* Hier können Sie ihren existierenden Code, der auf dem CGA-Modus basiert aufrufen */

    }

}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("Panic: {}", info);
    //	kprintln!("{:?}", Backtrace::new());
    loop {}
}


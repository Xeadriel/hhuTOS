/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: pit                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Programmable Interval Timer.                                    ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author:  Michael Schoettner, HHU, 15.6.2023                             ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use alloc::boxed::Box;
use core::arch::asm;
use core::sync::atomic::AtomicUsize;
use spin::Once;
use crate::devices::cga;
use crate::devices::cga::{Color, CGA, CGA_COLUMNS, CGA_ROWS};
use crate::kernel::{allocator, cpu};
use crate::kernel::cpu::IoPort;
use crate::kernel::interrupts::{intdispatcher, pic};
use crate::kernel::interrupts::intdispatcher::{InterruptVector, INT_VECTORS};
use crate::kernel::interrupts::isr::ISR;
use crate::kernel::interrupts::pic::{Irq, PIC};
use crate::kernel::threads::scheduler::{self, get_scheduler};

// Ports
const PORT_CTRL: u16 = 0x43;
const PORT_DATA0: u16 = 0x40;

const TIMER_FREQ: usize = 1193182; // Timer frequency in Hz
const NANOSECONDS_PER_TICK: usize = 1_000_000_000 / TIMER_FREQ; // Nanoseconds per timer tick

/// Global timer instance.
/// Not accessible from outside the module.
/// To get the current system time, use `get_system_time()`.
static TIMER: Once<Timer> = Once::new();

/// Global system time in milliseconds.
static SYSTEM_TIME: AtomicUsize = AtomicUsize::new(0);

/// Characters used for the spinner animation.
static SPINNER_CHARS: &[char] = &['|', '/', '-', '\\'];

/// Get the current system time in milliseconds.
pub fn get_system_time() -> usize {
    SYSTEM_TIME.load(core::sync::atomic::Ordering::Relaxed)
}

/// Wait for a specified number of milliseconds using the system time.
pub fn wait(ms: usize) {
    let start_time = get_system_time();
    loop {
        let current_time = get_system_time();
        if current_time - start_time >= ms {
            return;
        }
    }

}

/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Interrupt service routine implementation.                               ║
   ╚═════════════════════════════════════════════════════════════════════════╝ */

/// Register the timer interrupt handler.
pub fn plugin() {
    TIMER.call_once(|| {
        let mut timer = Timer::new();
        timer.set_interrupt_interval(1);
        timer
    });

    // Register the timer interrupt handler for IRQ 0 (Timer)
    intdispatcher::INT_VECTORS.lock().register(
        InterruptVector::Pit, 
        Box::new(TimerISR { interval_ms: 1 })
    );

    PIC.lock().allow(Irq::Pit);
}

/// The timer interrupt service routine.
struct TimerISR {
    /// The interval between timer interrupts in milliseconds.
    interval_ms: usize,
}

impl ISR for TimerISR {
    fn trigger(&self) {
        
        // Increment system time atomically
        let current_time = SYSTEM_TIME.fetch_add(1, core::sync::atomic::Ordering::Relaxed) + 1;
        
        unsafe { INT_VECTORS.force_unlock() };

        if current_time % 10 == 0 {
            get_scheduler().yield_cpu(); 
        }

        if !get_scheduler().is_locked() && !allocator::is_locked() && !CGA.is_queue_locked() {
            
            if let Some(mut cga) = CGA.try_lock() {
                // Calculate spinner char index (rotating)
                let index = (current_time / 250) % SPINNER_CHARS.len();
                let spinner_char = SPINNER_CHARS[index];
                
                // Print spinner at fixed position (top right corner)
                // Columns are 0..CGA_COLUMNS-1, rows 0..CGA_ROWS-1
                let col = CGA_COLUMNS - 1;
                let row = 0;
                
                // Backup current cursor position
                let pos = cga.getpos();
                let x = pos.0;
                let y = pos.1;
                
                // kprintln!("{} ", spinner_char);
                cga.setpos(col, row);
                
                print_cga!(&mut cga, "{spinner_char}");
                
                cga.setpos(x, y);
            }
                    
        }

        
    }
}

/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Implementation of the PIT driver itself.                                ║
   ╚═════════════════════════════════════════════════════════════════════════╝ */

/// Represents the programmable interval timer.
struct Timer {
    control_port: IoPort,
    data_port0: IoPort
}

impl Timer {
    /// Create a new Timer instance.
    pub const fn new() -> Timer {
        Timer {
            control_port: IoPort::new(PORT_CTRL),
            data_port0: IoPort::new(PORT_DATA0)
        }
    }

    /// Set the timer interrupt interval in milliseconds.
    pub fn set_interrupt_interval(&mut self, interval_ms: usize) {
        let reload_value = (TIMER_FREQ * interval_ms) / 1000;
        unsafe {
            self.control_port.outb(0x36); // Counter 0, access mode: low/high byte, mode 3, binary counting
            self.data_port0.outb((reload_value & 0xFF) as u8); // low byte
            self.data_port0.outb(((reload_value >> 8) & 0xFF) as u8); // high byte
        }

    }
}

/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: pcspk                                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Implementation for beep sound using the pc speaker. Works in    ║
   ║         qemu only if started with the correct audio settings.           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author:  Michael Schoettner, HHU, 22.9.2016                             ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
#![allow(dead_code)]

use crate::library::mutex::Mutex;
use crate::devices::pit;
use crate::kernel::cpu;
use crate::kernel::cpu::IoPort;

pub static SPEAKER: Mutex<Speaker> = Mutex::new(Speaker::new());

// Ports
const PORT_CTRL: u16 = 0x43;
const PORT_DATA0: u16 = 0x40;
const PORT_DATA2: u16 = 0x42;
const PORT_PPI: u16 = 0x61;

// Frequency of musical notes
// (Our OS does not really support floating point, so we convert the numbers to usize)
pub const C0: usize = 130.81 as usize;
pub const C0X: usize = 138.59 as usize;
pub const D0: usize = 146.83 as usize;
pub const D0X: usize = 155.56 as usize;
pub const E0: usize = 164.81 as usize;
pub const F0: usize = 174.61 as usize;
pub const F0X: usize = 185.00 as usize;
pub const G0: usize = 196.00 as usize;
pub const G0X: usize = 207.65 as usize;
pub const A0: usize = 220.00 as usize;
pub const A0X: usize = 233.08 as usize;
pub const B0: usize = 246.94 as usize;

pub const C1: usize = 261.63 as usize;
pub const C1X: usize = 277.18 as usize;
pub const D1: usize = 293.66 as usize;
pub const D1X: usize = 311.13 as usize;
pub const E1: usize = 329.63 as usize;
pub const F1: usize = 349.23 as usize;
pub const F1X: usize = 369.99 as usize;
pub const G1: usize = 391.00 as usize;
pub const G1X: usize = 415.30 as usize;
pub const A1: usize = 440.00 as usize;
pub const A1X: usize = 466.16 as usize;
pub const B1: usize = 493.88 as usize;

pub const C2: usize = 523.25 as usize;
pub const C2X: usize = 554.37 as usize;
pub const D2: usize = 587.33 as usize;
pub const D2X: usize = 622.25 as usize;
pub const E2: usize = 659.26 as usize;
pub const F2: usize = 698.46 as usize;
pub const F2X: usize = 739.99 as usize;
pub const G2: usize = 783.99 as usize;
pub const G2X: usize = 830.61 as usize;
pub const A2: usize = 880.00 as usize;
pub const A2X: usize = 923.33 as usize;
pub const B2: usize = 987.77 as usize;
pub const C3: usize = 1046.50 as usize;

pub struct Speaker {
    pit_ctrl_port: IoPort,
    pit_data2_port: IoPort,
    ppi_port: IoPort,
}

impl Speaker {
    /// Create a new Speaker instance.
    pub const fn new() -> Self {
        Speaker {
            pit_ctrl_port: IoPort::new(PORT_CTRL),
            pit_data2_port: IoPort::new(PORT_DATA2),
            ppi_port: IoPort::new(PORT_PPI),
        }
    }

    /// Play a specific frequency for a given amount of time (milliseconds).
    pub fn play(&mut self, frequency: usize, duration: usize) {

        if frequency == 0 {
            self.off();
            return;
        }
    
        let divisor = 1193182 / frequency;
    
        unsafe {
            // Set PIT counter 2 to mode 3 (square wave generator)
            self.pit_ctrl_port.outb(0b10110110); // Channel 2, Access: lobyte/hibyte, Mode 3, Binary
    
            // Send frequency divisor (lo-byte first, then hi-byte)
            self.pit_data2_port.outb(divisor as u8);         // Low byte
            self.pit_data2_port.outb((divisor >> 8) as u8);  // High byte
    
            // Turn the speaker on (enable bits 0 and 1 in PPI port)
            let mut val = self.ppi_port.inb();
            val |= 0x03; // Set bits 0 and 1
            self.ppi_port.outb(val);
        }
    
        pit::wait(duration);
        self.off();

    }

    /// Turn on the speaker.
    /// The played tone is dependent on counter 2 of the PIT.
    pub fn on(&mut self) {

        unsafe {
            
            let mut port_value = self.ppi_port.inb();

            port_value |= 0b11;

            self.ppi_port.outb(port_value);
        }

    }

    /// Turn off the speaker.
    pub fn off(&mut self) {
        unsafe {
            let mut val = self.ppi_port.inb();
            val &= !0x03; // Clear bits 0 and 1
            self.ppi_port.outb(val);
        }

    }
}

/// plays the Zelda theme using the PC speaker.
pub fn zelda() {
    SPEAKER.lock().play(440, 500);
    SPEAKER.lock().play(0, 5);
    SPEAKER.lock().play(329, 750);
    SPEAKER.lock().play(440, 250);
    SPEAKER.lock().play(0, 5);
    SPEAKER.lock().play(440, 125);
    SPEAKER.lock().play(493,125);
    SPEAKER.lock().play(523, 125);
    SPEAKER.lock().play(587, 125);
    SPEAKER.lock().play(659, 500);
}

/// Plays the Tetris theme using the PC speaker.
/// Kévin Rapaille, August 2013, https://gist.github.com/XeeX/6220067
pub fn tetris() {
    let mut speaker = SPEAKER.lock();
    
    speaker.play(658, 125);
    speaker.play(1320, 500);
    speaker.play(990, 250);
    speaker.play(1056, 250);
    speaker.play(1188, 250);
    speaker.play(1320, 125);
    speaker.play(1188, 125);
    speaker.play(1056, 250);
    speaker.play(990, 250);
    speaker.play(880, 500);
    speaker.play(880, 250);
    speaker.play(1056, 250);
    speaker.play(1320, 500);
    speaker.play(1188, 250);
    speaker.play(1056, 250);
    speaker.play(990, 750);
    speaker.play(1056, 250);
    speaker.play(1188, 500);
    speaker.play(1320, 500);
    speaker.play(1056, 500);
    speaker.play(880, 500);
    speaker.play(880, 500);
    pit::wait(250);
    speaker.play(1188, 500);
    speaker.play(1408, 250);
    speaker.play(1760, 500);
    speaker.play(1584, 250);
    speaker.play(1408, 250);
    speaker.play(1320, 750);
    speaker.play(1056, 250);
    speaker.play(1320, 500);
    speaker.play(1188, 250);
    speaker.play(1056, 250);
    speaker.play(990, 500);
    speaker.play(990, 250);
    speaker.play(1056, 250);
    speaker.play(1188, 500);
    speaker.play(1320, 500);
    speaker.play(1056, 500);
    speaker.play(880, 500);
    speaker.play(880, 500);
    pit::wait(500);
    speaker.play(1320, 500);
    speaker.play(990, 250);
    speaker.play(1056, 250);
    speaker.play(1188, 250);
    speaker.play(1320, 125);
    speaker.play(1188, 125);
    speaker.play(1056, 250);
    speaker.play(990, 250);
    speaker.play(880, 500);
    speaker.play(880, 250);
    speaker.play(1056, 250);
    speaker.play(1320, 500);
    speaker.play(1188, 250);
    speaker.play(1056, 250);
    speaker.play(990, 750);
    speaker.play(1056, 250);
    speaker.play(1188, 500);
    speaker.play(1320, 500);
    speaker.play(1056, 500);
    speaker.play(880, 500);
    speaker.play(880, 500);
    pit::wait(250);
    speaker.play(1188, 500);
    speaker.play(1408, 250);
    speaker.play(1760, 500);
    speaker.play(1584, 250);
    speaker.play(1408, 250);
    speaker.play(1320, 750);
    speaker.play(1056, 250);
    speaker.play(1320, 500);
    speaker.play(1188, 250);
    speaker.play(1056, 250);
    speaker.play(990, 500);
    speaker.play(990, 250);
    speaker.play(1056, 250);
    speaker.play(1188, 500);
    speaker.play(1320, 500);
    speaker.play(1056, 500);
    speaker.play(880, 500);
    speaker.play(880, 500);
    pit::wait(500);
    speaker.play(660, 1000);
    speaker.play(528, 1000);
    speaker.play(594, 1000);
    speaker.play(495, 1000);
    speaker.play(528, 1000);
    speaker.play(440, 1000);
    speaker.play(419, 1000);
    speaker.play(495, 1000);
    speaker.play(660, 1000);
    speaker.play(528, 1000);
    speaker.play(594, 1000);
    speaker.play(495, 1000);
    speaker.play(528, 500);
    speaker.play(660, 500);
    speaker.play(880, 1000);
    speaker.play(838, 2000);
    speaker.play(660, 1000);
    speaker.play(528, 1000);
    speaker.play(594, 1000);
    speaker.play(495, 1000);
    speaker.play(528, 1000);
    speaker.play(440, 1000);
    speaker.play(419, 1000);
    speaker.play(495, 1000);
    speaker.play(660, 1000);
    speaker.play(528, 1000);
    speaker.play(594, 1000);
    speaker.play(495, 1000);
    speaker.play(528, 500);
    speaker.play(660, 500);
    speaker.play(880, 1000);
    speaker.play(838, 2000);
    speaker.off();
}

/// Plays part of the song "Aerodynamic" by Daft Punk using the PC speaker.
/// https://www.kirrus.co.uk/2010/09/linux-beep-music
pub fn aerodynamic() {
    let mut speaker = SPEAKER.lock();
    
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(440, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(440, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(440, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(440, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(440, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(440, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(440, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(370, 122);
    speaker.play(493, 122);
    speaker.play(370, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(587, 122);
    speaker.play(415, 122);
    speaker.play(493, 122);
    speaker.play(415, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(784, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(493, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(440, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(440, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(440, 122);
    speaker.play(659, 122);
    speaker.play(440, 122);
    speaker.play(554, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(740, 122);
    speaker.play(987, 122);
    speaker.play(740, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1174, 122);
    speaker.play(830, 122);
    speaker.play(987, 122);
    speaker.play(830, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1568, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(987, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.play(1318, 122);
    speaker.play(880, 122);
    speaker.play(1108, 122);
    speaker.play(880, 122);
    speaker.off();
}

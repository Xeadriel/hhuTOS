/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: cga                                                             ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: This module provides functions for doing output on the CGA text ║
   ║         screen. It also supports a text cursor position stored in the   ║
   ║         hardware using ports.                                           ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 6.2.2024                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/
use spin::Mutex;
use crate::kernel::cpu as cpu;

/// Global CGA instance, used for screen output in the whole kernel.
/// Usage: let mut cga = cga::CGA.lock();
///        cga.print_byte(b'X');
pub static CGA: Mutex<CGA> = Mutex::new(CGA::new());

/// All 16 CGA colors.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)] // store each enum variant as an u8
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

pub const CGA_STD_ATTR: u8 = (Color::Black as u8) << 4 | (Color::White as u8);

const CGA_BASE_ADDR: *mut u8 = 0xb8000 as *mut u8;
const CGA_ROWS: usize = 25;
const CGA_COLUMNS: usize = 80;

const CGA_INDEX_PORT: u16 = 0x3d4; // select register
const CGA_DATA_PORT: u16 = 0x3d5;  // read/write register
const CGA_HIGH_BYTE_CMD: u8 = 14;  // cursor high byte
const CGA_LOW_BYTE_CMD: u8 = 15;   // cursor low byte

pub struct CGA {
    index_port: cpu::IoPort,
    data_port: cpu::IoPort,
}

impl CGA {
    const fn new() -> CGA {
        CGA {
            index_port: cpu::IoPort::new(CGA_INDEX_PORT),
            data_port: cpu::IoPort::new(CGA_DATA_PORT),
        }
    }

    /// Clear CGA screen and cursor to 0,0 position.
    pub fn clear(&mut self) {
        /* Hier muss Code eingefuegt werden */

        for y in 0..CGA_ROWS {
            for x in 0..CGA_COLUMNS {
                // write each character from the current row to the previous row
                self.show(x, y, ' ', CGA_STD_ATTR);
            }
        }
        self.setpos(0, 0);
    }

    /// Display the `character` at the given position `x`,`y` with attribute `attrib`.
    pub fn show(&mut self, x: usize, y: usize, character: char, attrib: u8) {
        if x > CGA_COLUMNS || y > CGA_ROWS {
            return;
        }

        let pos = (y * CGA_COLUMNS + x) * 2;

        // Write character and attribute to the screen buffer.
        //
        // Unsafe because we are writing directly to memory using a pointer.
        // We ensure that the pointer is valid by using CGA_BASE_ADDR
        // and checking the bounds of x and y.
        unsafe {
            CGA_BASE_ADDR.offset(pos as isize).write(character as u8);
            CGA_BASE_ADDR.offset((pos + 1) as isize).write(attrib);
        }
    }

    pub fn enable_cursor(&mut self) {
        /* Hier muss Code eingefuegt werden */
        unsafe {
            self.index_port.outb(0x0A); //scanline start
            self.data_port.outb(0x0D);
            
            self.index_port.outb(0x0B); //scanline end
            self.data_port.outb(0x0F);
        }
    }

    /// Return cursor position `x`,`y`
    pub fn getpos(&mut self) -> (usize, usize) {
        /* Hier muss Code eingefuegt werden */
        let mut pos : u16;
        unsafe {
            self.index_port.outb(CGA_LOW_BYTE_CMD);
            pos = self.data_port.inb() as u16;
            self.index_port.outb(CGA_HIGH_BYTE_CMD);
            pos |= (self.data_port.inb() as u16) << 8;
        }
        
        let x = pos as usize % CGA_COLUMNS;
        let y = pos as usize / CGA_COLUMNS;

        (x,y)
    }

    /// Set cursor position `x`,`y` 
    pub fn setpos(&mut self, mut x: usize, mut y: usize) {
        /* Hier muss Code eingefuegt werden */

        if x >= CGA_COLUMNS {
            x = CGA_COLUMNS - 1;
        }
        if y >= CGA_ROWS {
            y = CGA_ROWS - 1;
        }


        let pos : u16 = (y * CGA_COLUMNS + x) as u16;

        // set cursor position
        unsafe {
            self.index_port.outb(CGA_LOW_BYTE_CMD);
            self.data_port.outb((pos & 0xFF) as u8);
            self.index_port.outb(CGA_HIGH_BYTE_CMD);
            self.data_port.outb(((pos >> 8) & 0xFF) as u8);
        }
    }

    /// Print byte `b` at actual position cursor position `x`,`y`
    pub fn print_byte(&mut self, b : u8, bg: Color, fg: Color, blink: bool) {
        let (mut x, mut y) = self.getpos();

        if b == ('\n' as u8) {
            x = 0;
            y += 1;
            if y >= CGA_ROWS {
                self.scrollup();
            }
        } else {
            if x >= CGA_COLUMNS{
                x = 0;
                y += 1;

                if y >= CGA_ROWS{
                    y = CGA_ROWS-1;
                    self.scrollup();
                }
            }
            let attribute = self.attribute(bg, fg, blink);
            self.show(x, y, b as char, attribute);
            x += 1;
        }
        self.setpos(x, y);
    }

    /// Scroll text lines by one to the top.
    pub fn scrollup(&mut self) {
        /* Hier muss Code eingefuegt werden */
        for y in 1..CGA_ROWS {
            for x in 0..CGA_COLUMNS {
                // write each character from the current row to the previous row
                unsafe {
                    let offset = (y * CGA_COLUMNS + x) * 2;
                    let prev_offset = ((y-1) * CGA_COLUMNS + x ) * 2;
                    
                    CGA_BASE_ADDR.offset(prev_offset as isize).write(CGA_BASE_ADDR.offset(offset as isize).read());
                    CGA_BASE_ADDR.offset(prev_offset as isize +1 ).write(CGA_BASE_ADDR.offset(offset as isize +1).read());
                } 
            }
        }
        
        for x in 0..CGA_COLUMNS{
            self.show(x, CGA_ROWS-1, ' ', CGA_STD_ATTR);
        }
        self.setpos(0, CGA_ROWS-1);
    }

    /// Helper function returning an attribute byte for the given parameters `bg`, `fg`, and `blink`
    pub fn attribute(&mut self, bg: Color, fg: Color, blink: bool) -> u8 {
        /* Hier muss Code eingefuegt werden */
        let blink_bit = (blink as u8) << 7;
        
        let attr = ((bg as u8 & 0x7) << 4 | (fg as u8 & 0xf) ) | blink_bit;
        
        attr
    }
}
/* ╔═════════════════════════════════════════════════════════════════════════╗
   ║ Module: keyboard                                                        ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Descr.: Here are the public functions of all modules implemented in the ║
   ║         keyboard sub directory.                                         ║
   ╟─────────────────────────────────────────────────────────────────────────╢
   ║ Author: Michael Schoetter, Univ. Duesseldorf, 6.2.2024                  ║
   ╚═════════════════════════════════════════════════════════════════════════╝
*/

use spin::Mutex;

use crate::kernel::cpu as cpu;
use crate::devices::key as key;
use crate::devices::key::Key;
use crate::kernel::cpu::IoPort;

/// Global thread-safe access to keyboard.
/// Usage: let mut keyboard = keyboard::KEYBOARD.lock();
///        let key = keyboard.key_hit();
pub static KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard::new());

/// Represents the keyboard.
pub struct Keyboard {
    code: u8,       // Keyboard byte
    prefix: u8,     // Keyboard prefix
    gather: Key,    // Last decoded key
    leds: u8,       // LED status
    control_port: IoPort,
    data_port: IoPort
}

// Translation tables for ASCII codes
static NORMAL_TAB: [u8;89] =
    [
        0, 0, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 225, 39, 8, 0, 113,
        119, 101, 114, 116, 122, 117, 105, 111, 112, 129, 43, 13, 0, 97,
        115, 100, 102, 103, 104, 106, 107, 108, 148, 132, 94, 0, 35, 121,
        120, 99, 118, 98, 110, 109, 44, 46, 45, 0, 42, 0, 32, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, 43, 0, 0, 0, 0,
        0, 0, 0, 60, 0, 0
    ];

static SHIFT_TAB: [u8;89] =
    [
        0, 0, 33, 34, 21, 36, 37, 38, 47, 40, 41, 61, 63, 96, 0, 0, 81,
        87, 69, 82, 84, 90, 85, 73, 79, 80, 154, 42, 0, 0, 65, 83, 68,
        70, 71, 72, 74, 75, 76, 153, 142, 248, 0, 39, 89, 88, 67, 86, 66,
        78, 77, 59, 58, 95, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0
    ];

static ALT_TAB: [u8; 89] =
    [
        0, 0, 0, 253, 0, 0, 0, 0, 123, 91, 93, 125, 92, 0, 0, 0, 64, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 230, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 124, 0, 0
    ];

static ASC_NUM_TAB:[u8; 13] = [ 55, 56, 57, 45, 52, 53, 54, 43, 49, 50, 51, 48, 44 ];

static SCAN_NUM_TAB: [u8; 13] = [  8, 9, 10, 53, 5, 6, 7, 27, 2, 3, 4, 11, 51 ];

// LED names
const LED_CAPS_LOCK: u8 = 4;
const LED_NUM_LOCK: u8 = 2;
const LED_SCROLL_LOCK: u8 = 1;

// Constants needed for key decoding
const BREAK_BIT: u8 = 0x80;
const PREFIX1: u8   = 0xe0;
const PREFIX2:u8    = 0xe1;

// Keyboard IO-ports
const KBD_CTRL_PORT:u16 = 0x64;    // Status- (R) u. Steuerregister (W)
const KBD_DATA_PORT:u16 = 0x60;    // Ausgabe- (R) u. Eingabepuffer (W)

// Bits in the keyboard status register
const KBD_OUTB: u8 = 0x01;
const KBD_INPB: u8 = 0x02;
const KBD_AUXB: u8 = 0x20;

// Keyboard commands
const KBD_CMD_SET_LED: u8 = 0xed;
const KBD_CMD_SET_SPEED: u8 = 0xf3;
const KBD_CMD_CPU_RESET: u8 = 0xfe;

// Keyboard replies
const KBD_REPLY_ACK:u8 = 0xfa;

impl Keyboard {
    pub const fn new() -> Keyboard {
        Keyboard {
            code: 0,
            prefix: 0,
            gather: Key::new(0, 0, 0),
            leds: 0,
            control_port: IoPort::new(KBD_CTRL_PORT),
            data_port: IoPort::new(KBD_DATA_PORT)
        }
    }
    
    /// Interpret the make and break codes of the keyboard.
    /// Return true if the key is complete, false if codes are missing.
    fn key_decoded(&mut self) -> bool {
        let mut done: bool = false;

        // Keys that are new in the MF II keyboard (compared to the old AT keyboard)
        // send a prefix byte first.
        if self.code == PREFIX1 || self.code == PREFIX2 {
            self.prefix = self.code;
            return false;
        }

        // Releasing a key is only of interest for the "Modifier" keys SHIFT, CTRL and ALT.
        // For the others, the break code can be ignored.
        if (self.code & BREAK_BIT) != 0 {
            self.code &= !BREAK_BIT; // A key's break code is the same as its make code, but with the break bit set.
            match self.code {
                42 | 54 => {
                    self.gather.set_shift (false);
                }
                56 => {
                    if self.prefix == PREFIX1 { self.gather.set_alt_right(false); }
                    else                      { self.gather.set_alt_left(false);  }
                }
                29 => {
                    if self.prefix == PREFIX1 { self.gather.set_ctrl_right(false);}
                    else                      { self.gather.set_ctrl_left(false); }
                }
                _ => { // All other keys
                }
            }

            // A prefix is only valid for the next key. So it is now handled.
            self.prefix = 0;
            return false;
        }
        
        // A key has been pressed. For the modifier keys like SHIFT, ALT, NUM_LOCK etc.
        // only the internal state is changed. The return value 'false' indicates
        // that the keyboard input is not yet complete. For the other keys, ASCII
        // and scancode are set and 'true' is returned for a successful keyboard query,
        // although technically the break code of the key is still missing.
        match self.code {
            42 | 54 => {
                self.gather.set_shift(true);
            }
            56 => {
                if self.prefix == PREFIX1 { self.gather.set_alt_right(true);  }
                else                      { self.gather.set_alt_left(true);   }
            }
            29 => {
                if self.prefix == PREFIX1 { self.gather.set_ctrl_right(true); }
                else                      { self.gather.set_ctrl_left(true);  }
            }
            58 => {
                self.gather.set_caps_lock( !self.gather.get_caps_lock() );
                self.set_led(LED_CAPS_LOCK, self.gather.get_caps_lock());
            }
            70 => {
                self.gather.set_scroll_lock( !self.gather.get_scroll_lock() );
                self.set_led(LED_SCROLL_LOCK, self.gather.get_scroll_lock());
            }
            69 => { // Numlock or Break
                if self.gather.get_ctrl_left() { // Break Key
                    // On old keyboards, the Break function could only be reached via
                    // Ctrl+NumLock. Modern MF-II keyboards send this code combination
                    // when Break is meant. The Break key normally does not deliver an
                    // ASCII code, but looking it up does not hurt. In any case, the
                    // key is now complete.
                    self.get_ascii_code();
                    done = true;
                }
                else { // NumLock
                    self.gather.set_num_lock( !self.gather.get_num_lock() );
                    self.set_led(LED_NUM_LOCK, self.gather.get_num_lock());
                }
            }

            _ => { // All other keys
                // Read ASCII code from the appropriate table -> Key is decoded
                self.get_ascii_code();
                done = true;
            }
        }

        // A prefix is only valid for the next key. So it is now handled.
        self.prefix = 0;
        done
    }


    /// Calculate the ASCII code from the scancode and modifier bits.
    fn get_ascii_code(&mut self) {
        // Special case Scancode 53: This code is sent by both the minus key
        // of the normal keyboard area and the division key of the numeric
        // keypad. In order to get the correct code in both cases, a conversion
        // to the correct code of the division key must be performed
        // in the case of the numeric keypad.
        if self.code == 53 && self.prefix == PREFIX1 { // Division key of numpad
            self.gather.set_ascii('/' as u8);
            self.gather.set_scancode(key::SCAN_DIV);
        }

        // Choose the right table based on the modifier bits. For simplicity,
        // NumLock takes precedence over Alt, Shift and CapsLock. There is
        // no separate table for Ctrl.
        else if self.gather.get_num_lock() && self.prefix == 0 &&
            self.code >= 71 && self.code <= 83 {
            // If numlock is enabled and one of the keys of the separate number block
            // (codes 71-83) is pressed, the ASCII and scancodes of the corresponding
            // number keys should be delivered instead of the scancodes of the cursor
            // keys. The keys of the cursor block (prefix == prefix1) should of course
            // still be able to be used for cursor control. By the way, they still send
            // a shift, but that should not matter.
            self.gather.set_ascii(ASC_NUM_TAB[ (self.code - 71) as usize]);
            self.gather.set_scancode(SCAN_NUM_TAB[ (self.code - 71) as usize]);
        }
        else if self.gather.get_alt_right() {
            self.gather.set_ascii(ALT_TAB[self.code as usize]);
            self.gather.set_scancode(self.code);
        }
        else if self.gather.get_shift() {
            self.gather.set_ascii(SHIFT_TAB[self.code as usize]);
            self.gather.set_scancode(self.code);
        }
        else if self.gather.get_caps_lock() {
            // CapsLock is only active for the letters A-Z and 0-9.
            if (self.code >= 16 && self.code <= 26) ||
                (self.code >= 30 && self.code<= 40) ||
                (self.code >= 44 && self.code <= 50) {
                self.gather.set_ascii (SHIFT_TAB[self.code as usize]);
                self.gather.set_scancode(self.code);
            }
            else {
                self.gather.set_ascii(NORMAL_TAB[self.code as usize]);
                self.gather.set_scancode(self.code);
            }
        }
        else {
            self.gather.set_ascii(NORMAL_TAB[self.code as usize]);
            self.gather.set_scancode(self.code);
        }
    }
    
    /// Poll the keyboard controller until a key is pressed.
    /// Decode and return the key if it is complete.
    pub fn key_hit(&mut self) -> Key {
        let invalid: Key = Default::default();  // nicht explizit initialisierte Tasten sind ungueltig
        let mut status : u8;

        /*****************************************************************************
         * Funktion:        key_hit                                                  *
         *---------------------------------------------------------------------------*
         * Beschreibung:    Diese Methode soll einen Tastendruck zurueckliefern.     *
         *                  Hierzu soll die Tastatur in einer Schleife "gepollt"     *
         *                  werden, bis ein Zeichen eingegebn wurde.                 *
         *                                                                           *
         *                  Das Byte von der Tastatur soll in dem Attribut 'code'    *
         *                  (siehe Keyboard.h) gespeichert werden. Die Dekodierung   *
         *                  soll mithilfe der vorgegebenen Funktion 'key_decoded'    *
         *                  erfolgen.                                                *
         *                                                                           *
         * Rückgabewert:    Wenn der Tastendruck abgeschlossen ist und ein Scancode, *
         *                  sowie gegebenenfalls ein ASCII-Code emittelt werden      *
         *                  konnte, werden diese in 'gather' (siehe Keyboard.h)      *
         *                  zurueckgeliefert. Anderenfalls liefert key_hit () einen  *
         *                  ungueltigen Wert zurueck, was mit Key::valid ()          *
         *                  ueberprueft werden kann.                                 *
         *****************************************************************************/

        unsafe{
            loop{
                    status = self.control_port.inb();
                    if status & KBD_OUTB != 0 { // ready
                        // read keyboard code
                        self.code = self.data_port.inb();
                        break;
                    }
                }
        }
            
        // check if not mouse and if decoding complete
        if (status & KBD_AUXB) == 0 && self.key_decoded() {
            return self.gather
        }

        // otherwise invalid key
        return invalid
    }
    
    /// Set the repeat rate of the keyboard (determined by the speed and delay).
    /// 
    /// The speed determines how fast repeated keys are sent.
    /// Valid values are between 0 (very fast) and 31 (very slow).
    /// 
    /// The delay determines how long a key must be pressed before the keyboard starts repeating it.
    /// Valid values are between 0 (minimum delay) and 3 (maximum delay).
    /// 0 = 250ms, 1 = 500ms, 2 = 750ms, 3 = 1000ms
    pub fn set_repeat_rate(&mut self, speed: u8, delay: u8) -> i8 {

        /* Hier muss Code eingefuegt werden. */

        /*****************************************************************************
         * Funkion:         set_repeat_rate                                          *
         *---------------------------------------------------------------------------*
         * Beschreibung:    Einstellen der Wiederholungsrate der Tastatur.           *
         *                                                                           *
         * Parameter:                                                                *
         *      delay:      Bestimmt, wie lange eine Taste gedrueckt werden muss,    *
         *                  bevor die Wiederholung einsetzt. Erlaubt sind Werte      *
         *                  zw. 0 (minimale Wartezeit) und 3 (maximale Wartezeit).   *
         *                  0=250ms, 1=500ms, 2=750ms, 3=1000ms                      *
         *                                                                           *
         *      speed:      Bestimmt, wie schnell die Tastencodes aufeinander folgen *
         *                  sollen. Erlaubt sind Werte zwischen 0 (sehr schnell)     *
         *                  und 31 (sehr langsam).                                   *
         *                                                                           *
         *                  ((2 ^ B) * (D + 8) / 240 sec                             *
         *                  Bits 4-3 = B; Bits 2-0 = D;                              *
         *****************************************************************************/

         loop{
            unsafe{
                let status = self.control_port.inb();
                if status & KBD_INPB == 0 { // ready
                    // write repeat ratecommand to keyboard
                    self.data_port.outb(0xF3);
                    break;
                }
            }
        }

        // wait until command answer arrives
        loop{
            unsafe{
                let status = self.control_port.inb();
                if status & KBD_OUTB != 0 { 
                    break;

                }
            }
        }
        
        unsafe {
            let answer = self.data_port.inb();
            if answer != 0xfa { // command not accepted
                return -1
            }
        }
        

        // set repeat rate
        let mut command = 0x00;
        command |= (speed << 2) & 0x1f;
        command |= delay & 0x03;

        unsafe{
            self.data_port.outb(command);
        }

        // wait until command answer arrives
        loop{
            unsafe{
                let status = self.control_port.inb();
                if status & KBD_OUTB != 0 { 
                    break;
                    }

            }
        }
        
        unsafe {
        let answer = self.data_port.inb();
            if answer != 0xfa { // command not accepted
                return -1
            }
        }

        return 0

    }
    
    /// Enable/Disable the LEDs on the keyboard.
    /// Multiple LEDs can be set at the same time as a bit mask.
    /// 1 = Caps Lock, 2 = Num Lock, 4 = Scroll Lock
    pub fn set_led(&mut self, led: u8, on: bool) -> i8{

        /* Hier muss Code eingefuegt werden. */
        
        /*****************************************************************************
         * Funktion:        set_led                                                  *
         *---------------------------------------------------------------------------*
         * Beschreibung:    Setzt oder loescht die angegebene Leuchtdiode.           *
         *                                                                           *
         * Parameter:                                                                *
         *      led:        Welche LED? (caps_lock, num_lock, scroll_lock)           *
         *      on:         0 = aus, 1 = an                                          *
         *****************************************************************************/
    
         loop{
            unsafe{
                let status = self.control_port.inb();
                if status & KBD_INPB == 0 { // ready
                    // write repeat ratecommand to keyboard
                    self.data_port.outb(0xED);
                    break;
                }
            }
        }

        // wait until command answer arrives
        loop{
            unsafe{
                let status = self.control_port.inb();
                if status & KBD_OUTB != 0 { 
                    break;

                }
            }
        }
        
        unsafe {
            let answer = self.data_port.inb();
            if answer != 0xfa { // command not accepted
                return -1
            }
        }
        

        // set repeat rate
        let command = 0x00 | led & on as u8;

        unsafe{
            self.data_port.outb(command);
        }

        // wait until command answer arrives
        loop{
            unsafe{
                let status = self.control_port.inb();
                if status & KBD_OUTB != 0 { 
                    break;
                    }

            }
        }
        
        unsafe {
        let answer = self.data_port.inb();
            if answer != 0xfa { // command not accepted
                return -1
            }
        }

        return 0
    }
}

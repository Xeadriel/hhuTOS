use crate::devices::cga; // shortcut for cga
use crate::devices::cga_print; // used to import code needed by println! 
use crate::devices::key as key; // shortcut for key
use crate::devices::keyboard; // shortcut for keyboard
use crate::cga::Color;


pub fn run() {

    /* Hier muss Code einfgeügt werden */
    let mut keyboard = keyboard::KEYBOARD.lock();
    let mut cga = cga::CGA.lock();

    keyboard.set_repeat_rate(2, 2);
    // 'key_hit' aufrufen und Zeichen ausgeben
    loop {
        let mut c = keyboard.key_hit();
        let mut ascii = c.get_ascii();

        if ascii >= 0x20 && ascii <= 0x7e || ascii == 13 { // 13 == return
            if ascii == 13 {ascii = b'\n'}
            cga.print_byte(ascii, Color::Black, Color::White, false);
        }
    }
    
}


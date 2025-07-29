use crate::devices::keyboard;

/// Wait for a key press and return the character if it is a valid ASCII character.
pub fn getch() -> char {
   loop {
      let mut key = keyboard::get_key_buffer().wait_for_key();
      if key.valid() && key.get_ascii() != 0 {
         if key.get_ascii() == 13 { // enter key
             return '\n';
         }

         if key.get_ascii() == 8 { // backspace key
             return '\x08';
         }

         return char::from_u32(key.get_ascii() as u32).unwrap();
      }
   }
}

pub fn get_last_ch() -> char {
   let maybe_key = keyboard::get_key_buffer().get_last_key();
   if let Some(mut key) = maybe_key {
      if key.valid() && key.get_ascii() != 0 {
         if key.get_ascii() == 13 { // enter key
             return '\n';
         }
         if key.get_ascii() == 8 { // backspace key
             return '\x08';
         }

         return char::from_u32(key.get_ascii() as u32).unwrap();
      }
   }

   return ' ';
}

/// Wait for the Enter key to be pressed.
pub fn wait_for_return() {
   loop {
      if getch() == '\r' {
         break;
      }
   }
}

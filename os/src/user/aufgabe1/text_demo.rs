use core::fmt::Write;

use alloc::format;
use alloc::string::ToString;
use nolock::queues::mpsc::jiffy::queue;

use crate::devices::cga; // shortcut for cga
use crate::devices::cga_print;
use crate::devices::cga_print::WRITER; // used to import code needed by println!
use crate::devices::cga::Color;

pub fn run () {
    println!("  | dec | hex | bin  |");
    println!("------------------------");
    for i in 0..16 {
        println!("  | {:>3} | {:#x} | {:04b} |", i, i, i);
    }
    
    
}

use core::fmt::Write;

use alloc::string::ToString;
use nolock::queues::mpsc::jiffy::queue;

use crate::devices::cga; // shortcut for cga
use crate::devices::cga_print;
use crate::devices::cga_print::WRITER; // used to import code needed by println!
use crate::devices::cga::Color;

pub fn run () {
    let mut writer = WRITER.lock();

    writer.write_str("  | dec | hex | bin   |\n");
    writer.write_str("  ---------------------\n");
    writer.write_str("  |   0 |   0 |     0 |\n");
    writer.write_str("  |   1 |   1 |     1 |\n");
    writer.write_str("  |   2 |   2 |    10 |\n");
    writer.write_str("  |   3 |   3 |    11 |\n");
    writer.write_str("  |   4 |   4 |   100 |\n");
    writer.write_str("  |   5 |   5 |   101 |\n");
    writer.write_str("  |   6 |   6 |   110 |\n");
    writer.write_str("  |   7 |   7 |   111 |\n");
    writer.write_str("  |   8 |   8 |  1000 |\n");
    writer.write_str("  |   9 |   9 |  1001 |\n");
    writer.write_str("  |  10 |   a |  1010 |\n");
    writer.write_str("  |  11 |   b |  1011 |\n");
    writer.write_str("  |  12 |   c |  1100 |\n");
    writer.write_str("  |  13 |   d |  1101 |\n");
    writer.write_str("  |  14 |   e |  1110 |\n");
    writer.write_str("  |  15 |   f |  1111 |\n");
    writer.write_str("  |  16 |  10 | 10000 |\n");
    
}

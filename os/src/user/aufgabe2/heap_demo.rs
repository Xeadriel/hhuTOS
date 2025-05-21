use alloc::boxed::Box;
use alloc::vec;

use crate::devices::cga::{self, Color};
use crate::devices::cga_print::{self, print};
use crate::devices::keyboard;
use crate::kernel::allocator;

pub fn run () {

    struct S {
        a: u32,
        b: u32,
    }


    println!("Heap demo 1/4: allocate structs via box");
    println!("===========================");
    println!("");

    allocator::dump_free_list();

    unsafe {cga_print::FG_COLOR = Color::LightGreen;}
    let s1 = Box::new(S { a: 1, b: 2 });
    println!("s1.a={}, s1.b={}", s1.a, s1.b);
    unsafe {cga_print::FG_COLOR = Color::White;}
    allocator::dump_free_list();
    
    unsafe {cga_print::FG_COLOR = Color::LightGreen;}
    let s2 = Box::new(S { a: 3, b: 4 });
    println!("s2.a={}, s2.b={}", s2.a, s2.b);
    unsafe {cga_print::FG_COLOR = Color::White;}
    allocator::dump_free_list();

    println!("");
    println!("Press <Return> to continue");
    while 13 != keyboard::KEYBOARD.lock().key_hit().get_ascii(){}
    
    
    cga::CGA.lock().clear();
    println!("Heap demo 2/4: drop structs");
    println!("===========================");
    println!("");

    unsafe {cga_print::FG_COLOR = Color::LightRed;}
    drop(s1);
    unsafe {cga_print::FG_COLOR = Color::White;}
    allocator::dump_free_list();

    unsafe {cga_print::FG_COLOR = Color::LightRed;}
    drop(s2);
    unsafe {cga_print::FG_COLOR = Color::White;}
    allocator::dump_free_list();
    
    println!("");
    println!("Press <Return> to continue");
    while 13 != keyboard::KEYBOARD.lock().key_hit().get_ascii(){}
    
    
    cga::CGA.lock().clear();
    println!("Heap demo 3/4: allocate 3 structs in 1 vec");
    println!("===========================");
    println!("");

    unsafe {cga_print::FG_COLOR = Color::LightGreen;}
    let s1 = vec![S { a: 1, b: 2 }, S { a: 3, b: 4 }, S { a: 5, b: 6 }];
    unsafe {cga_print::FG_COLOR = Color::White;}
    allocator::dump_free_list();

    println!("");
    println!("Press <Return> to continue");
    while 13 != keyboard::KEYBOARD.lock().key_hit().get_ascii(){}
    
    
    cga::CGA.lock().clear();
    println!("Heap demo 4/4: drop vec with structs");
    println!("===========================");
    println!("");

    unsafe {cga_print::FG_COLOR = Color::LightRed;}
    drop(s1);
    unsafe {cga_print::FG_COLOR = Color::White;}
    allocator::dump_free_list();
    
    println!("");
    println!("Press <Return> to continue");
    while 13 != keyboard::KEYBOARD.lock().key_hit().get_ascii(){}
    cga::CGA.lock().clear();
}

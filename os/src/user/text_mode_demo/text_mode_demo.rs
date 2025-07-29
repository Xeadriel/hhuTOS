use alloc::{boxed::Box, vec};

use crate::{devices::{self, cga::{self, Color, CGA}, cga_print, pit}, kernel::{allocator, threads::{scheduler::get_scheduler, thread::Thread}}, library::input::getch};

pub fn run() {
    loop {
        cga::CGA.lock().clear();
        cga::CGA.lock().setpos(0, 0);
        println!("Welcome to the text mode demo!");
        println!("Please select an option:");
        println!("");
        println!("1 - Text I/O");
        println!("2 - Sound Demo");
        println!("3 - Memory Allocation Demo");
        println!("4 - Thread Demo");
        match getch() {
            '1' => {
                cga::CGA.lock().clear();
                cga::CGA.lock().setpos(0, 0);
                text_output();
                int_keyboard();
            }
            '2' => {
                cga::CGA.lock().clear();
                cga::CGA.lock().setpos(0, 0);
                sound_demo();
            }
            '3' => {
                cga::CGA.lock().clear();
                cga::CGA.lock().setpos(0, 0);
                memory_allocation_demo();
            }
            '4' => {
                cga::CGA.lock().clear();
                cga::CGA.lock().setpos(0, 0);
                thread_demo();
            }
            _ => {
                
            }
        }
    }
}

pub fn text_output () {
    println!("  | dec | hex | bin  |");
    println!("------------------------");
    for i in 0..16 {
        println!("  | {:>3} | {:#x} | {:04b} |", i, i, i);
    }
    
    println!("");
    println!("<Press return to continue to show scrolling>");
    getch();
    
    for i in 0..10 {
        println!("Line {}", i);
    }

    println!("");
    println!("<Press return to continue>");
    getch();
}

pub fn int_keyboard() {
    println!("Now it's time to test the interrupt-based keyboard.");
    println!("<Press return to to exit loop>");
    loop {
        let c = getch();   
        if c == '\n' {
            break;
        }
        print!("{}", c);
    }
    println!("")
}

pub fn sound_demo() {
    println!("Now playing the beginning of the main theme from The Legend of Zelda:");
    
    devices::pcspk::zelda();
    
    println!("");
    println!("<Press return to exit the sound demo>");
    getch();
}

pub fn memory_allocation_demo() {
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

    println!("");
    println!("<Press Return to continue>");
    getch();
    
    unsafe {cga_print::FG_COLOR = Color::LightGreen;}
    let s2 = Box::new(S { a: 3, b: 4 });
    println!("s2.a={}, s2.b={}", s2.a, s2.b);
    unsafe {cga_print::FG_COLOR = Color::White;}
    allocator::dump_free_list();

    println!("");
    println!("<Press Return to continue>");
    loop {
        let c = getch();   
        if c == '\n' {
            break;
        }
    }
    
    
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
    println!("<Press Return to continue>");
    loop {
        let c = getch();   
        if c == '\n' {
            break;
        }
    }
    
    
    cga::CGA.lock().clear();
    println!("Heap demo 3/4: allocate 3 structs in 1 vec");
    println!("===========================");
    println!("");

    unsafe {cga_print::FG_COLOR = Color::LightGreen;}
    let s1 = vec![S { a: 1, b: 2 }, S { a: 3, b: 4 }, S { a: 5, b: 6 }];
    unsafe {cga_print::FG_COLOR = Color::White;}
    allocator::dump_free_list();

    println!("");
    println!("<Press Return to continue>");
    loop {
        let c = getch();   
        if c == '\n' {
            break;
        }
    }
    
    
    cga::CGA.lock().clear();
    println!("Heap demo 4/4: drop vec with structs");
    println!("===========================");
    println!("");

    unsafe {cga_print::FG_COLOR = Color::LightRed;}
    drop(s1);
    unsafe {cga_print::FG_COLOR = Color::White;}
    allocator::dump_free_list();
    
    println!("");
    println!("<Press Return to continue>");
    loop {
        let c = getch();   
        if c == '\n' {
            break;
        }
    }
    cga::CGA.lock().clear();
}

fn thread_entry() {

    let mut i = 0;
    let tid = get_scheduler().get_active_tid();

    let start = pit::get_system_time();

    loop {
        {
            let mut cga = CGA.lock();
            let (x, y) = cga.getpos();
            cga.setpos(0, tid);

            print_cga!(&mut cga, "Thread {}: Counter: {}", tid, i);
            cga.setpos(x, y);
        }
        
        if (i) % 10 == 0 {
            get_scheduler().yield_cpu();
        }
        
        if i == 10000 {
            let end = pit::get_system_time();
            
            {
                let mut cga = CGA.lock();
                let (x, y) = cga.getpos();
                cga.setpos(0, tid+5);

                print_cga!(&mut cga, "Thread {} completed time spent: {}", tid, end - start);
                cga.setpos(x, y);
            }

            get_scheduler().exit()
        }
        
        i+=1;
    }

}

pub fn thread_demo() {
    println!("Thread demo: create 3 threads that count up and");
    println!("kill themselves after 10000 iterations");
    println!("Restart OS after this as this starts the scheduler.");
    println!("<Press Return to continue>");
    getch();
    cga::CGA.lock().clear();
    cga::CGA.lock().setpos(0, 0);

    get_scheduler().ready(Thread::new(thread_entry));
    get_scheduler().ready(Thread::new(thread_entry));
    get_scheduler().ready(Thread::new(thread_entry));
    get_scheduler().schedule();
}




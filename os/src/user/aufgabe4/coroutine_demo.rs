use crate::devices::cga;
use crate::kernel::coroutines::coroutine::Coroutine;

fn coroutine_loop(c: &mut Coroutine) {
    let mut i = 0;
    loop {

        cga::CGA.lock().setpos(0, c.get_id());
        // switch to next coroutine

        print!("Couroutine {}:, Counter: {}", c.get_id(), i);
        i+=1;
        Coroutine::switch(c)
    }
}

pub fn run() {
    // create three coroutines
    let mut a = Coroutine::new(coroutine_loop);
    let mut b = Coroutine::new(coroutine_loop);
    let mut c = Coroutine::new(coroutine_loop);

    // chain them in a cycle a -> b -> d -> a
    a.set_next(&mut b);
    b.set_next(&mut c);
    c.set_next(&mut a);
    // start the first
    a.start();
}
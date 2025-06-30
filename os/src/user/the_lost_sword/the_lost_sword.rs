use crate::devices::lfb::{get_lfb, HHU_BLUE, HHU_GREEN, LFB};
use crate::devices::pit::get_system_time;
use crate::devices::{pcspk, pit};
use crate::kernel::threads::scheduler::get_scheduler;
use crate::kernel::threads::thread::Thread;
use crate::user::the_lost_sword::player::{self, Player};
use crate::user::the_lost_sword::{player_sprite, slime};
use crate::user::the_lost_sword::grass;
use crate::user::the_lost_sword::rng::RNG;
use crate::user::the_lost_sword::slime::Slime;

const MESSAGE: &str = "Welcome to hhuTOS!";

pub fn run() {
    let draw_thread = Thread::new(game_loop);
    let sound_thread = Thread::new(pcspk::tetris);
    
    let scheduler = get_scheduler();
    scheduler.ready(draw_thread);
    // scheduler.ready(sound_thread);
    scheduler.schedule();
}

fn game_loop() {
    let mut player = Player::new();
    let mut slime1 = Slime::new();
    player.x = 200;
    player.y = 200;

    slime1.x = 600;
    slime1.y = 500;
    
    let mut last_time = pit::get_system_time();
    let mut draw_timer: isize = 30;

    loop {
        let delta = (pit::get_system_time() - last_time) as isize;
        last_time = pit::get_system_time();
        draw_timer -= delta;

        if draw_timer <= 0 { // reduces flickering a little, implementing double/triple buffering would be better but I have no time for that
            draw_timer = 30;
            let mut lfb = get_lfb().lock();
            lfb.draw_bitmap(0, 0, grass::WIDTH, grass::HEIGHT, grass::DATA);
        }

        if player.hp <= 0 {
            
        } else {
            player.process(delta);
    
            slime1.do_damage_check(player.get_attack_rect(), 1);

        }
        
        if slime1.hp <= 0 {
            slime1.attack_rect = (-10, -10, -20, -20);
        }
        else {
            player.do_damage_check(slime1.get_attack_rect(), 1);
            slime1.process(delta);
        }
        
    }
}
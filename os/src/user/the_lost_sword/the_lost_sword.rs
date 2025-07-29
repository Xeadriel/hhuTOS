use alloc::vec::Vec;

use crate::devices::lfb::{get_lfb, HHU_BLUE, HHU_GREEN, LFB};
use crate::devices::pit::get_system_time;
use crate::devices::{pcspk, pit};
use crate::kernel::threads::scheduler::get_scheduler;
use crate::kernel::threads::thread::Thread;
use crate::user::the_lost_sword::drag_tul::DragTul;
use crate::user::the_lost_sword::player::{self, Player};
use crate::user::the_lost_sword::sound_effects::{play, play_no_thread, SoundEffect};
use crate::user::the_lost_sword::{drag_tul, dungeon, player_sprite, slime, sound_effects, title_screen};
use crate::user::the_lost_sword::grass;
use crate::user::the_lost_sword::rng::RNG;
use crate::user::the_lost_sword::slime::Slime;
use crate::user::the_lost_sword::story;

const DRAW_COOLDOWN : isize= 10;

enum GameState {
    TitleScreen,
    Intro,
    Level0,
    Level1,
    Level2,
    VillageStory,
    GameOver,
    FlameFistTraining,
    Level3,
    Level4,
    Level5,
    Level6,
    BossStory,
    BossFight,
    Victory,    
}

pub fn run() {
    get_scheduler().ready(Thread::new(game_loop));
    get_scheduler().schedule();
}

fn game_loop() {

    let mut story = story::Story::new();

    let mut player = Player::new();
    let mut dragtul = DragTul::new();
    
    let mut enemies: Vec<Slime> = Vec::new();
    
    let mut last_time = pit::get_system_time();
    let mut background_draw_timer: isize = 0;
    
    let mut game_state = GameState::Victory;
    let mut current_state_initialized = false;
    let mut jingle_played = false; //music at level start

    loop {
        let delta = (pit::get_system_time() - last_time) as isize;
        last_time = pit::get_system_time();
        background_draw_timer -= delta;
        
        match game_state {
            GameState::TitleScreen => {
                story.play_title_screen();
                game_state = GameState::Intro;
            }
            GameState::Intro => {
                story.play_intro();
                game_state = GameState::Level0;
            }
            GameState::Level0 => {
                if !current_state_initialized {
                    current_state_initialized = true;

                    player.x = 400;
                    player.y = 400;
                    player.hp = 3;
                    
                    while enemies.len() > 0{
                        enemies.remove(0);
                    }

                    let mut slime = Slime::new();
                    slime.x = 600;
                    slime.y = 500;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 200;
                    slime.y = 500;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 200;
                    slime.y = 50;
                    enemies.push(slime);
                }
                
                if background_draw_timer <= 0 { // reduces flickering a little, implementing double/triple buffering would be better but I have no time for that
                    background_draw_timer = DRAW_COOLDOWN;
                    let mut lfb = get_lfb().lock();
                    lfb.draw_bitmap(0, 0, grass::WIDTH, grass::HEIGHT, grass::DATA);
                    // lfb.draw_bitmap_rgba(30, 30, test::WIDTH, test::HEIGHT, test::DATA);
                }
                
                
                if player.hp <= 0 {
                    // game over screen + game over sound
                    // button check for restarting game?
                    game_state = GameState::GameOver;
                    current_state_initialized = false;
                    get_scheduler().yield_cpu();
                    pit::wait(1000);
                } else {
                    player.process(delta);
                }
                
                {
                    let mut i = 0;
                    while i < enemies.len() {
                        if enemies[i].hp <= 0 {
                            enemies.remove(i);
                            continue; // not incrementing i because we removed an element and it shifts in place
                        }

                        enemies[i].process(delta);
                        enemies[i].do_damage_check(player.get_attack_rect(), 1);
                        player.do_damage_check(enemies[i].get_attack_rect(), 1);
                        i += 1;
                    }

                    if enemies.len() == 0 {
                        game_state = GameState::Level1; // next level
                        current_state_initialized = false;
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                    }

                    if jingle_played == false {
                        play_no_thread(SoundEffect::SwitchSceneTheme);
                        jingle_played = true;
                    }
                }

            }
            GameState::Level1 => {
                if !current_state_initialized {
                    current_state_initialized = true;
                    jingle_played = false;

                    // add some story? 

                    player.x = 400;
                    player.y = 400;
                    
                    let mut slime = Slime::new();
                    slime.x = 300;
                    slime.y = 100;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 100;
                    slime.y = 50;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 700;
                    slime.y = 500;
                    enemies.push(slime);
                }

                if background_draw_timer <= 0 { // reduces flickering a little, implementing double/triple buffering would be better but I have no time for that
                    background_draw_timer = DRAW_COOLDOWN;
                    let mut lfb = get_lfb().lock();
                    lfb.draw_bitmap(0, 0, grass::WIDTH, grass::HEIGHT, grass::DATA);
                }
                
                if player.hp <= 0 {
                    // game over screen + game over sound
                    // button check for restarting game?
                    game_state = GameState::GameOver;
                    current_state_initialized = false;
                    get_scheduler().yield_cpu();
                    pit::wait(1000);
                } else {
                    player.process(delta);
                }
                
                {
                    let mut i = 0;
                    while i < enemies.len() {
                        if enemies[i].hp <= 0 {
                            enemies.remove(i);
                            continue; // not incrementing i because we removed an element and it shifts in place
                        }

                        enemies[i].process(delta);
                        enemies[i].do_damage_check(player.get_attack_rect(), 1);
                        player.do_damage_check(enemies[i].get_attack_rect(), 1);
                        i += 1;
                    }

                    if enemies.len() == 0 {
                        game_state = GameState::Level2; // next level
                        current_state_initialized = false;
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                        // victory sound?
                    }

                    if jingle_played == false {
                        play_no_thread(SoundEffect::SwitchSceneTheme);
                        jingle_played = true;
                    }
                }
            }
            GameState::Level2 => {
                if !current_state_initialized {
                    current_state_initialized = true;
                    jingle_played = false;

                    player.x = 400;
                    player.y = 400;
                    
                    let mut slime = Slime::new();
                    slime.x = 500;
                    slime.y = 100;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 300;
                    slime.y = 470;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 700;
                    slime.y = 300;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 700;
                    slime.y = 150;
                    enemies.push(slime);
                }

                if background_draw_timer <= 0 { // reduces flickering a little, implementing double/triple buffering would be better but I have no time for that
                    background_draw_timer = DRAW_COOLDOWN;
                    let mut lfb = get_lfb().lock();
                    lfb.draw_bitmap(0, 0, grass::WIDTH, grass::HEIGHT, grass::DATA);
                }
                
                if player.hp <= 0 {
                    // game over screen + game over sound
                    // button check for restarting game?
                    game_state = GameState::GameOver;
                    current_state_initialized = false;
                    get_scheduler().yield_cpu();
                    pit::wait(1000);
                } else {
                    player.process(delta);
                }
                
                {
                    let mut i = 0;
                    while i < enemies.len() {
                        if enemies[i].hp <= 0 {
                            enemies.remove(i);
                            continue; // not incrementing i because we removed an element and it shifts in place
                        }

                        enemies[i].process(delta);
                        enemies[i].do_damage_check(player.get_attack_rect(), 1);
                        player.do_damage_check(enemies[i].get_attack_rect(), 1);
                        i += 1;
                    }

                    if enemies.len() == 0 {
                        game_state = GameState::VillageStory; // next level
                        current_state_initialized = false;
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                        // victory sound?
                    }

                    if jingle_played == false {
                        play_no_thread(SoundEffect::SwitchSceneTheme);
                        jingle_played = true;
                    }
                }
            }
            GameState::VillageStory => {
                story.play_village_story();
                game_state = GameState::FlameFistTraining;
            }
            GameState::FlameFistTraining => {
                if !current_state_initialized {
                    current_state_initialized = true;
                    jingle_played = false;
                    story.play_flame_fist_training1();
                }
                if background_draw_timer <= 0 { // reduces flickering a little, implementing double/triple buffering would be better but I have no time for that
                    background_draw_timer = DRAW_COOLDOWN;
                    let mut lfb = get_lfb().lock();
                    lfb.draw_bitmap(0, 0, grass::WIDTH, grass::HEIGHT, grass::DATA);
                }

                player.hp = 3;
                player.x = 400;
                player.y = 300;
                player.flame_fist_count = 0;
                player.process(delta);

                if player.flame_fist_count > 0 {
                    story.play_flame_fist_training2();
                    player.flame_fist_count = 0;
                    game_state = GameState::Level3;
                    current_state_initialized = false;
                }
            }
            GameState::Level3 => {
                if !current_state_initialized {
                    current_state_initialized = true;
                    jingle_played = false;

                    player.x = 400;
                    player.y = 400;
                    
                    let mut slime = Slime::new();
                    slime.x = 500;
                    slime.y = 100;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 300;
                    slime.y = 470;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 700;
                    slime.y = 300;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 700;
                    slime.y = 150;
                    enemies.push(slime);
                }

                if background_draw_timer <= 0 { // reduces flickering a little, implementing double/triple buffering would be better but I have no time for that
                    background_draw_timer = DRAW_COOLDOWN;
                    let mut lfb = get_lfb().lock();
                    lfb.draw_bitmap(0, 0, dungeon::WIDTH, dungeon::HEIGHT, dungeon::DATA);
                    lfb.draw_bitmap(400, 0, dungeon::WIDTH, dungeon::HEIGHT, dungeon::DATA);
                }
                
                if player.hp <= 0 {
                    // game over screen + game over sound
                    // button check for restarting game?
                    game_state = GameState::GameOver;
                    current_state_initialized = false;
                    get_scheduler().yield_cpu();
                    pit::wait(1000);
                } else {
                    player.process(delta);
                }
                
                {
                    let mut i = 0;
                    while i < enemies.len() {
                        if enemies[i].hp <= 0 {
                            enemies.remove(i);
                            continue; // not incrementing i because we removed an element and it shifts in place
                        }

                        enemies[i].process(delta);
                        enemies[i].do_damage_check(player.get_attack_rect(), 1);
                        player.do_damage_check(enemies[i].get_attack_rect(), 1);
                        i += 1;
                    }

                    if enemies.len() == 0 {
                        game_state = GameState::Level4; // next level
                        current_state_initialized = false;
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                        // victory sound?
                    }

                    if player.flame_fist_count > 0 {
                        game_state = GameState::GameOver;
                        current_state_initialized = false;
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                    }

                    if jingle_played == false {
                        play_no_thread(SoundEffect::SwitchSceneTheme);
                        jingle_played = true;
                    }
                }
            }
            GameState::Level4 => {
                if !current_state_initialized {
                    current_state_initialized = true;
                    jingle_played = false;

                    player.x = 400;
                    player.y = 400;
                    
                    let mut slime = Slime::new();
                    slime.x = 100;
                    slime.y = 500;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 200;
                    slime.y = 370;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 700;
                    slime.y = 200;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 500;
                    slime.y = 500;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 700;
                    slime.y = 500;
                    enemies.push(slime);
                }

                if background_draw_timer <= 0 { // reduces flickering a little, implementing double/triple buffering would be better but I have no time for that
                    background_draw_timer = DRAW_COOLDOWN;
                    let mut lfb = get_lfb().lock();
                    lfb.draw_bitmap(0, 0, dungeon::WIDTH, dungeon::HEIGHT, dungeon::DATA);
                    lfb.draw_bitmap(400, 0, dungeon::WIDTH, dungeon::HEIGHT, dungeon::DATA);
                }
                
                if player.hp <= 0 {
                    // game over screen + game over sound
                    // button check for restarting game?
                    game_state = GameState::GameOver;
                    current_state_initialized = false;
                    get_scheduler().yield_cpu();
                    pit::wait(1000);
                } else {
                    player.process(delta);
                }
                
                {
                    let mut i = 0;
                    while i < enemies.len() {
                        if enemies[i].hp <= 0 {
                            enemies.remove(i);
                            continue; // not incrementing i because we removed an element and it shifts in place
                        }

                        enemies[i].process(delta);
                        enemies[i].do_damage_check(player.get_attack_rect(), 1);
                        player.do_damage_check(enemies[i].get_attack_rect(), 1);
                        i += 1;
                    }

                    if enemies.len() == 0 {
                        game_state = GameState::Level5; // next level
                        current_state_initialized = false;
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                        // victory sound?
                    }

                    if player.flame_fist_count > 0 {
                        game_state = GameState::GameOver;
                        current_state_initialized = false;
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                    }

                    if jingle_played == false {
                        play_no_thread(SoundEffect::SwitchSceneTheme);
                        jingle_played = true;
                    }
                }
            }
            GameState::Level5 => {
                if !current_state_initialized {
                    current_state_initialized = true;
                    jingle_played = false;

                    player.x = 400;
                    player.y = 400;
                    
                    let mut slime = Slime::new();
                    slime.x = 100;
                    slime.y = 200;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 500;
                    slime.y = 370;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 700;
                    slime.y = 300;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 500;
                    slime.y = 400;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 300;
                    slime.y = 500;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 200;
                    slime.y = 300;
                    enemies.push(slime);
                }

                if background_draw_timer <= 0 { // reduces flickering a little, implementing double/triple buffering would be better but I have no time for that
                    background_draw_timer = DRAW_COOLDOWN;
                    let mut lfb = get_lfb().lock();
                    lfb.draw_bitmap(0, 0, dungeon::WIDTH, dungeon::HEIGHT, dungeon::DATA);
                    lfb.draw_bitmap(400, 0, dungeon::WIDTH, dungeon::HEIGHT, dungeon::DATA);
                }
                
                if player.hp <= 0 {
                    // game over screen + game over sound
                    // button check for restarting game?
                    game_state = GameState::GameOver;
                    current_state_initialized = false;
                    get_scheduler().yield_cpu();
                    pit::wait(1000);
                } else {
                    player.process(delta);
                }
                
                {
                    let mut i = 0;
                    while i < enemies.len() {
                        if enemies[i].hp <= 0 {
                            enemies.remove(i);
                            continue; // not incrementing i because we removed an element and it shifts in place
                        }

                        enemies[i].process(delta);
                        enemies[i].do_damage_check(player.get_attack_rect(), 1);
                        player.do_damage_check(enemies[i].get_attack_rect(), 1);
                        i += 1;
                    }

                    if enemies.len() == 0 {
                        game_state = GameState::BossStory; // next level
                        current_state_initialized = false;
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                        // victory sound?
                    }

                    if player.flame_fist_count > 0 {
                        game_state = GameState::GameOver;
                        current_state_initialized = false;
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                    }

                    if jingle_played == false {
                        play_no_thread(SoundEffect::SwitchSceneTheme);
                        jingle_played = true;
                    }
                }
            }
            GameState::BossStory => {
                story.play_boss_story();
                game_state = GameState::BossFight;
            }
            GameState::BossFight => {
                if !current_state_initialized {
                    current_state_initialized = true;
                    jingle_played = false;

                    player.x = 400;
                    player.y = 400;
                    
                    let mut slime = Slime::new();
                    slime.x = 100;
                    slime.y = 200;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 500;
                    slime.y = 370;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 700;
                    slime.y = 300;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 500;
                    slime.y = 400;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 300;
                    slime.y = 500;
                    enemies.push(slime);

                    slime = Slime::new();
                    slime.x = 200;
                    slime.y = 300;
                    enemies.push(slime);

                    dragtul.x = 300;
                    dragtul.y = 200;
                }

                if background_draw_timer <= 0 { // reduces flickering a little, implementing double/triple buffering would be better but I have no time for that
                    background_draw_timer = DRAW_COOLDOWN;
                    let mut lfb = get_lfb().lock();
                    lfb.draw_bitmap(0, 0, dungeon::WIDTH, dungeon::HEIGHT, dungeon::DATA);
                    lfb.draw_bitmap(400, 0, dungeon::WIDTH, dungeon::HEIGHT, dungeon::DATA);
                }
                
                if player.hp <= 0 {
                    // game over screen + game over sound
                    // button check for restarting game?
                    game_state = GameState::GameOver;
                    current_state_initialized = false;
                    get_scheduler().yield_cpu();
                    pit::wait(1000);
                } else {
                    player.process(delta);
                }
                
                {
                    let mut i = 0;
                    while i < enemies.len() {
                        if enemies[i].hp <= 0 {
                            enemies.remove(i);
                            continue; // not incrementing i because we removed an element and it shifts in place
                        }

                        enemies[i].process(delta);
                        enemies[i].do_damage_check(player.get_attack_rect(), 1);
                        player.do_damage_check(enemies[i].get_attack_rect(), 1);
                        i += 1;
                    }

                    {
                        dragtul.process(delta);
                        if dragtul.hp > 1 {
                            dragtul.do_damage_check(player.get_attack_rect(), 1);
                        }
                        else if player.flame_fist_count > 0 {
                            player.get_flame_attack_rects().iter().for_each(|rect| {
                                dragtul.do_damage_check(*rect, 1);
                            })
                        }

                        player.do_damage_check(dragtul.get_attack_rect(), 1);
                        i += 1;
                    }

                    if dragtul.hp <= 0 {
                        // win game
                        // continue so that player doesnt die from succeeding to hit flame fist
                        game_state = GameState::Victory;
                        sound_effects::play_no_thread(SoundEffect::Victory);
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                        continue
                    }

                    if player.flame_fist_count > 0 {
                        game_state = GameState::GameOver;
                        current_state_initialized = false;
                        get_scheduler().yield_cpu();
                        pit::wait(1000);
                    }

                    if jingle_played == false {
                        play_no_thread(SoundEffect::SwitchSceneTheme);
                        jingle_played = true;
                    }
                }
            }
            GameState::Victory => {
                story.play_victory();
                game_state = GameState::TitleScreen;
            }
            GameState::GameOver => {
                story.play_game_over();
                game_state = GameState::TitleScreen;
            }
            _ => {}
        }
        

        
    }
}
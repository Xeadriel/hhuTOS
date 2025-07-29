use crate::{devices::{pcspk::SPEAKER, pit}, kernel::threads::{scheduler::get_scheduler, thread::Thread}};

pub enum SoundEffect {
    SwordSound,
    SwordHit,
    PlayerHit,
    MainTheme,
    SwitchSceneTheme,
    GameOver,
    FlameFist,
    Victory
}

// queues the sound effect to the scheduler
pub fn play(effect: SoundEffect) {
    let sound_effect = match effect {
        SoundEffect::SwordSound => sword_sound,
        SoundEffect::SwordHit => sword_hit,
        SoundEffect::PlayerHit => player_hit,
        SoundEffect::MainTheme => main_theme,
        SoundEffect::SwitchSceneTheme => switch_scene_theme,
        SoundEffect::GameOver => game_over_theme,
        SoundEffect::FlameFist => flame_fist,
        SoundEffect::Victory => victory,
    };

    get_scheduler().ready(Thread::new(sound_effect))
}

pub fn play_no_thread(effect: SoundEffect) {
    let sound_effect = match effect {
        SoundEffect::SwordSound => sword_sound,
        SoundEffect::SwordHit => sword_hit,
        SoundEffect::PlayerHit => player_hit,
        SoundEffect::MainTheme => main_theme,
        SoundEffect::SwitchSceneTheme => switch_scene_theme,
        SoundEffect::GameOver => game_over_theme,
        SoundEffect::FlameFist => flame_fist,
        SoundEffect::Victory => victory,
    };

    sound_effect()
}

// grub code:
// 2000 700 1 3000 1 2000 1 3000 1
fn sword_sound() {
    SPEAKER.lock().play(700, 30);
    SPEAKER.lock().play(3000, 30);
    SPEAKER.lock().play(2000, 30);
    SPEAKER.lock().play(3000, 30);
}

// grub code:
// 2000 150 1 100 1
fn sword_hit() {
    SPEAKER.lock().play(150, 30);
    SPEAKER.lock().play(100, 30);
}

// grub code:
// 2000 50 1 30 1
fn player_hit() {
    SPEAKER.lock().play(50, 30);
    SPEAKER.lock().play(30, 30);
}

//2000 1 800 1 90 1 80 1 90 4 40 1
fn flame_fist() {
    SPEAKER.lock().play(2000, 125);
    SPEAKER.lock().play(800, 125);
    SPEAKER.lock().play(90, 125);
    SPEAKER.lock().play(80, 125);
    SPEAKER.lock().play(90, 500);
    SPEAKER.lock().play(40, 125);
}

// grub code:
// 480 391 4 246 2 196 2 
// 783 4 440 2 391 2 
// 587 2 369 2 293 4
// 440 2 391 2 587 2 369 2 293 4 
// 293 2 369 1 440 1 391 2 
// 493 4 100 1 150 1 100 1 150 1
// 4 = 500, 2 = 250, 1 = 125

// 480 391 4 246 2 196 2 783 4 440 2 391 2 587 2 369 2 293 4 440 2 391 2 587 2 369 2 293 4 293 2 369 1 440 1 391 2 493 4 100 1 150 1 100 1 150 1 4 = 500, 2 = 250, 1 = 125
fn main_theme() {
        SPEAKER.lock().play(391, 500);
        SPEAKER.lock().play(246, 250);
        SPEAKER.lock().play(196, 250);
    
        SPEAKER.lock().play(783, 500);
        SPEAKER.lock().play(440, 250);
        SPEAKER.lock().play(391, 250);
    
        SPEAKER.lock().play(587, 250);
        SPEAKER.lock().play(369, 250);
        SPEAKER.lock().play(293, 500);
    
        SPEAKER.lock().play(440, 250);
        SPEAKER.lock().play(391, 250);
        SPEAKER.lock().play(587, 250);
        SPEAKER.lock().play(369, 250);
        SPEAKER.lock().play(293, 500);
    
        SPEAKER.lock().play(293, 250);
        SPEAKER.lock().play(369, 125);
        SPEAKER.lock().play(440, 125);
        SPEAKER.lock().play(391, 250);
        
        SPEAKER.lock().play(493, 500);
        SPEAKER.lock().play(100, 125);
        SPEAKER.lock().play(150, 125);
        SPEAKER.lock().play(100, 125);
        SPEAKER.lock().play(150, 125);
        SPEAKER.lock().play(50, 250);
}


// 591 2 446 2 396 4
// 293 2 369 1 440 1 391 2
//493 4 100 1 150 1 100 1 150 1 50 2
pub fn switch_scene_theme(){
    SPEAKER.lock().play(591, 250);
    SPEAKER.lock().play(446, 250);
    SPEAKER.lock().play(396, 500);

    SPEAKER.lock().play(293, 250);
    SPEAKER.lock().play(369, 125);
    SPEAKER.lock().play(440, 125);
    SPEAKER.lock().play(391, 250);
    
    SPEAKER.lock().play(493, 500);
    SPEAKER.lock().play(100, 125);
    SPEAKER.lock().play(150, 125);
    SPEAKER.lock().play(100, 125);
    SPEAKER.lock().play(150, 125);
    SPEAKER.lock().play(50, 250);
}


// 146 6 123 6 98 8
pub fn game_over_theme(){
    SPEAKER.lock().play(146, 750);
    SPEAKER.lock().play(123, 750);
    SPEAKER.lock().play(98, 1000);
}

// 789 1 698 1 493 1 0 1 789 1 0 1 789 2 0 1 789 1 698 1 987 1 789 2 3157 1
pub fn victory(){
    SPEAKER.lock().play(789, 125);
    SPEAKER.lock().play(698, 125);
    SPEAKER.lock().play(493, 125);
    pit::wait(125);
    SPEAKER.lock().play(789, 125);
    pit::wait(125);
    SPEAKER.lock().play(789, 250);
    pit::wait(125);
    SPEAKER.lock().play(789, 125);
    SPEAKER.lock().play(698, 125);
    SPEAKER.lock().play(987, 125);
    SPEAKER.lock().play(789, 250);
    SPEAKER.lock().play(3157, 250);
}
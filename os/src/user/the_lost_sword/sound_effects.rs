use crate::{devices::pcspk::SPEAKER, kernel::threads::{scheduler::get_scheduler, thread::Thread}};

pub enum SoundEffect {
    SwordSound,
    SwordHit,
    PlayerHit,
}

// queues the sound effect to the scheduler
pub fn play(effect: SoundEffect) {
    let sound_effect = match effect {
        SoundEffect::SwordSound => sword_sound,
        SoundEffect::SwordHit => sword_hit,
        SoundEffect::PlayerHit => player_hit,
    };

    get_scheduler().ready(Thread::new(sound_effect))
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
use crate::{devices::lfb::get_lfb, library::input::{get_last_ch, getch}, user::the_lost_sword::{player_sprite, rng::RNG, slime, slime_sprite, sound_effects, sword_down, sword_left, sword_right, sword_up}};

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE
}
pub struct Slime {
    pub x: isize,
    pub y: isize,
    pub hp: usize,
    pub hit_timer: isize,
    pub move_timer: isize,
    pub attack_rect: (isize, isize, isize, isize),
}


const MOVEMENT_SPEED: isize = 10;
const DAMAGE_GRACE_PERIOD: isize = 500; // miliseconds
const ATTACK_SPEED: i32 = 500; // miliseconds
const MOVEMENT_COOLDOWN: isize = 1000; // miliseconds

impl Slime {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            hp: 3,
            hit_timer: 0,
            move_timer: MOVEMENT_COOLDOWN,
            attack_rect: (0, 0, 48, 48), // (x1, y1, x2, y2) absolute coordinates,upper left corner, lower right corner
        }
    }

    pub fn do_damage_check(&mut self, attack_rect : (isize, isize, isize, isize), damage : usize) -> bool{
        let (x1, y1, x2, y2) = attack_rect;
        let (x11, y11, x22, y22) = (self.x, self.y, self.x + slime_sprite::WIDTH as isize, self.y + slime_sprite::HEIGHT as isize);
        if x11 <= x2 && x22 >= x1 && y11 <= y2 && y22 >= y1 {
            self.take_damage(damage);
            return true;
        }

        false
    }

    pub fn take_damage(&mut self, amount: usize) {
        if self.hit_timer <= 0 {
            self.hp -= amount;
            self.hit_timer = DAMAGE_GRACE_PERIOD;
            sound_effects::play(sound_effects::SoundEffect::SwordHit);
        }
    }

    pub fn get_attack_rect(&self) -> (isize, isize, isize, isize) {
        (
            self.attack_rect.0 + self.x as isize,
            self.attack_rect.1 + self.y as isize,
            self.attack_rect.2 + self.x as isize,
            self.attack_rect.3 + self.y as isize,
        )
    }

    pub fn process(&mut self, delta: isize) {

        if self.hit_timer > 0 {
            self.hit_timer -= delta;
        }

        if self.move_timer > 0 {
            self.move_timer -= delta;
        }
        else {
            self.move_timer = RNG.lock().rand_range(0, 100) as isize;
            let random_direction = RNG.lock().rand_range(0, 4);
    
            if random_direction == 0 {
                self.y -= MOVEMENT_SPEED; // up
            } else if random_direction == 1 {
                self.y += MOVEMENT_SPEED; // down
            } else if random_direction == 2 {
                self.x -= MOVEMENT_SPEED; // left
            } else if random_direction == 3 {
                self.x += MOVEMENT_SPEED; // right
            }
            
            if self.x < MOVEMENT_SPEED {
                self.x = MOVEMENT_SPEED;
            }
            if self.x > 800 - 48 - MOVEMENT_SPEED{
                self.x = 800 - 48 - MOVEMENT_SPEED;
            }
            if self.y < MOVEMENT_SPEED {
                self.y = MOVEMENT_SPEED;
            }
            if self.y > 600 - 64 - MOVEMENT_SPEED {
                self.y = 600 - 64 - MOVEMENT_SPEED;
            }
        }


        

        let mut lfb = get_lfb().lock();
        lfb.draw_bitmap_rgba(self.x as u32, self.y as u32, slime_sprite::WIDTH, slime_sprite::HEIGHT, slime_sprite::DATA);
            
    }
}

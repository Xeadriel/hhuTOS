use crate::{devices::lfb::get_lfb, library::input::{get_last_ch, getch}, user::the_lost_sword::{heart_sprite, player_sprite, sound_effects, sword_down, sword_left, sword_right, sword_up}};

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE
}
pub struct Player {
    pub x: isize,
    pub y: isize,
    pub hp: usize,
    pub hit_timer: isize,
    pub attack_timer: isize,
    pub attack_direction: Direction,
    pub attack_rect: (isize, isize, isize, isize),
}

const MOVEMENT_SPEED: isize = 10;
const DAMAGE_GRACE_PERIOD: isize = 500; // miliseconds
const ATTACK_SPEED: isize = 500; // miliseconds

impl Player {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            hp: 3,
            hit_timer: 0,
            attack_timer: 0,
            attack_direction: Direction::NONE,
            attack_rect: (-10, -10, -15, -15) // (x1, y1, x2, y2) absolute coordinates,upper left corner, lower right corner
        }
    }

    pub fn do_damage_check(&mut self, attack_rect : (isize, isize, isize, isize), damage : usize) -> bool {
        let (x1, y1, x2, y2) = attack_rect;
        let (x11, y11, x22, y22) = (self.x, self.y, self.x + player_sprite::WIDTH as isize, self.y + player_sprite::HEIGHT as isize);
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
            sound_effects::play(sound_effects::SoundEffect::PlayerHit);
        }

        if self.hp <= 0 {
            // game over
            
        }
    }

    pub fn get_attack_rect(&self) -> (isize, isize, isize, isize) {
        self.attack_rect
    }

    pub fn attack(&self, direction: Direction) {
        match direction {
            Direction::UP => {
                
            }
            Direction::DOWN => {
                
            }
            Direction::LEFT => {
                
            }
            Direction::RIGHT => {
                
            }
            _ => {}
        }
    }

    pub fn process(&mut self, delta: isize) {

        if self.hit_timer > 0 {
            self.hit_timer -= delta;
        }

        // switch statement using getch()

        let char = get_last_ch();

        
        match char {
            'w' => {
                self.y -= MOVEMENT_SPEED;
            }
            's' => {
                self.y += MOVEMENT_SPEED;
            }
            _ => {}
        }
        
        match char {
            'a' => {
                self.x -= MOVEMENT_SPEED;
            }
            'd' => {
                self.x += MOVEMENT_SPEED;
            }
            _ => {}
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
        
        {
            let mut lfb = get_lfb().lock();
            lfb.draw_bitmap(self.x as u32, self.y as u32, player_sprite::WIDTH, player_sprite::HEIGHT, player_sprite::DATA);
            
            if 0 < self.attack_timer && self.attack_timer <= ATTACK_SPEED {
                
                let x = self.x;
                let y = self.y;
                
                match self.attack_direction {
                    Direction::UP => {
                        self.attack_rect = (x, y - 64, x + 48, y);
                        lfb.draw_bitmap((x + 18) as u32, (y - 64) as u32, sword_up::WIDTH, sword_up::HEIGHT, sword_up::DATA);
                    }
                    Direction::DOWN => {
                        self.attack_rect = (x , y + 64, x + 48, y + 128);
                        lfb.draw_bitmap((x + 18) as u32, (y + 64) as u32, sword_down::WIDTH, sword_down::HEIGHT, sword_down::DATA);
                    }
                    Direction::LEFT => {
                        self.attack_rect = (x - 64, y + 12, x, y + 48 + 12);
                        lfb.draw_bitmap((x - 64) as u32, (y + 24) as u32, sword_left::WIDTH, sword_left::HEIGHT, sword_left::DATA);
                    }
                    Direction::RIGHT => {
                        self.attack_rect = (x + 48, y + 12, x + 48 + 64, y + 12 + 48);
                        lfb.draw_bitmap((x + 48) as u32, (y + 24) as u32, sword_right::WIDTH, sword_right::HEIGHT, sword_right::DATA);
                    }
                    _ => {}
                }
                self.attack_timer -= delta;
                if self.attack_timer <= 0 {
                    self.attack_rect = (-10, -10, -15, -15);
                }
                
            }
            else {
                match  char{
                    'j' => {
                        self.attack_timer = ATTACK_SPEED;
                        self.attack_direction = Direction::LEFT;
                        self.attack(Direction::LEFT);
                        sound_effects::play(sound_effects::SoundEffect::SwordSound);
                    }
                    'l' => {
                        self.attack_timer = ATTACK_SPEED;
                        self.attack_direction = Direction::RIGHT;
                        self.attack(Direction::RIGHT);
                        sound_effects::play(sound_effects::SoundEffect::SwordSound);
                    }
                    'k' => {
                        self.attack_timer = ATTACK_SPEED;
                        self.attack_direction = Direction::DOWN;
                        self.attack(Direction::DOWN);
                        sound_effects::play(sound_effects::SoundEffect::SwordSound);
                    }
                    'i' => {
                        self.attack_timer = ATTACK_SPEED;
                        self.attack_direction = Direction::UP;
                        self.attack(Direction::UP);
                        sound_effects::play(sound_effects::SoundEffect::SwordSound);
                    }
                    _ => {}
                }
            }

        }
        
        self.draw_health();
        
    }
    
    fn draw_health(&self) {
        let mut lfb = get_lfb().lock();
        
        for i  in 0..self.hp {
            lfb.draw_bitmap(650 + i as u32 * 24, 10, heart_sprite::WIDTH, heart_sprite::HEIGHT, heart_sprite::DATA);
        }
    }
}

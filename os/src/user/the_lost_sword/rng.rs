use crate::library::mutex::Mutex;

pub static RNG : Mutex<RNG> = Mutex::new(RNG::new(111111111));

pub struct RNG {
    state: usize,
}

impl RNG {
    pub const fn new(seed: usize) -> Self {
        Self { state: seed }
    }

    pub fn next(&mut self) -> usize {

        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }

    pub fn rand_range(&mut self, min: usize, max: usize) -> usize {
        min + (self.next() % (max - min))
    }
}

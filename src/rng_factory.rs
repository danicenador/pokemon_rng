use rand::{rngs::ThreadRng, Rng};

pub struct RngFactory {
    rng: ThreadRng,
}

impl RngFactory {
    pub fn new() -> Self {
        RngFactory {
            rng: rand::thread_rng(),
        }
    }
    pub fn generate_personality_value(&mut self) -> u32 {
        self.rng.gen()
    }
}

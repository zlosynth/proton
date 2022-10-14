use daisy::hal::prelude::_stm32h7xx_hal_rng_RngCore;
use daisy::hal::rng::Rng;

use proton_instruments_interface::Rand;

pub struct Randomizer {
    pub rng: Rng,
}

impl Randomizer {
    pub fn new(rng: Rng) -> Self {
        Self { rng }
    }
}

impl Rand for Randomizer {
    fn generate(&mut self) -> u16 {
        self.rng.gen().unwrap()
    }
}

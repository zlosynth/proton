use daisy::hal::prelude::_stm32h7xx_hal_rng_RngCore;
use daisy::hal::rng::Rng;

#[cfg(feature = "karplus_strong")]
use proton_instruments_karplus_strong::Rand;

#[cfg(feature = "kaseta")]
use proton_instruments_kaseta::Rand;

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

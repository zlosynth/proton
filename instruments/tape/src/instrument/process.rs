use dasp::{signal, Signal};

use super::Instrument;
use crate::Rand;

impl Instrument {
    pub fn process(&mut self, block: &mut [f32; 32], _randomizer: &mut impl Rand) {
        let block_copy = *block;

        let input_signal = signal::from_iter(block_copy.into_iter());
        let mut pre_gained_signal = input_signal.mul_amp(self.pre_gain.by_ref());

        block.iter_mut().for_each(|f| {
            *f = pre_gained_signal.next();
        });
    }
}

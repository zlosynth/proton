use dasp::{signal, Signal};

use super::Instrument;
use crate::Rand;

impl Instrument {
    pub fn process(&mut self, block: &mut [f32; 32], _randomizer: &mut impl Rand) {
        let block_copy = *block;

        let input_signal = signal::from_iter(block_copy.into_iter());
        let pre_gained_signal = input_signal.mul_amp(self.pre_gain.by_ref());
        let mut hysteresis_signal = pre_gained_signal.map(|x| self.hysteresis.process(x));

        block.iter_mut().for_each(|f| {
            *f = hysteresis_signal.next();
        });
    }
}

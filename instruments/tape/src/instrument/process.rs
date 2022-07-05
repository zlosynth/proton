use dasp::{signal, Signal};

use super::Instrument;
use crate::Rand;

use proton_primitives::oversampling::downsampling::SignalDownsample;
use proton_primitives::oversampling::upsampling::SignalUpsample;

impl Instrument {
    pub fn process(&mut self, block: &mut [f32; 32], _randomizer: &mut impl Rand) {
        let block_copy = *block;

        let mut instrument = signal::from_iter(block_copy.into_iter())
            .mul_amp(self.pre_gain.by_ref())
            .clip_amp(10.0)
            .upsample(&mut self.upsampler)
            .map(|x| self.hysteresis.process(x))
            .downsample(&mut self.downsampler);

        block.iter_mut().for_each(|f| {
            *f = instrument.next();
        });
    }
}

use dasp::{signal, Signal};

use super::Instrument;
use crate::Rand;

use proton_primitives::oversampling::downsampling::SignalDownsample;
use proton_primitives::oversampling::upsampling::SignalUpsample;

use crate::hysteresis::SignalApplyHysteresis;

impl Instrument {
    pub fn process(&mut self, block: &mut [f32; 32], _randomizer: &mut impl Rand) {
        let block_copy = *block;

        let mut instrument = signal::from_iter(block_copy.into_iter())
            .mul_amp(self.pre_gain.by_ref())
            .clip_amp(10.0)
            .upsample(&mut self.upsampler)
            .apply_hysteresis(
                &mut self.hysteresis,
                self.drive.by_ref(),
                self.saturation.by_ref(),
                self.width.by_ref(),
            )
            .downsample(&mut self.downsampler);

        block.iter_mut().for_each(|f| {
            *f = instrument.next();
        });
    }
}

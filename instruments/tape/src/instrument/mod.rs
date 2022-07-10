mod attributes;
mod command;
mod control;
mod execute;
mod process;
mod state;

use dasp::Signal;

use crate::hysteresis::Hysteresis;

use proton_primitives::oversampling::downsampling::Downsampler16;
use proton_primitives::oversampling::upsampling::Upsampler16;

pub struct Instrument {
    pub(crate) pre_gain: SmoothedValue,

    pub(crate) hysteresis: Hysteresis,
    pub(crate) drive: SmoothedValue,
    pub(crate) saturation: SmoothedValue,
    pub(crate) width: SmoothedValue,

    pub(crate) upsampler: Upsampler16,
    pub(crate) downsampler: Downsampler16,
}

impl Instrument {
    pub fn new(sample_rate: u32) -> Self {
        let drive = SmoothedValue::new(1.0);
        let saturation = SmoothedValue::new(1.0);
        let width = SmoothedValue::new(1.0);
        let hysteresis = Hysteresis::new(
            sample_rate as f32,
            drive.value(),
            saturation.value(),
            width.value(),
        );

        Self {
            hysteresis,
            drive,
            saturation,
            width,
            pre_gain: SmoothedValue::new(1.0),
            upsampler: Upsampler16::new_16(),
            downsampler: Downsampler16::new_16(),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum SmoothedValue {
    Stable(f32),
    Converging(f32, f32, f32),
}

impl SmoothedValue {
    const STEP: f32 = 1.0 / 64.0;

    pub fn new(value: f32) -> Self {
        Self::Stable(value)
    }

    pub fn set(&mut self, value: f32) {
        let last_value = self.next();
        *self = Self::Converging(last_value, value, 0.0);
    }

    pub fn value(&self) -> f32 {
        match self {
            Self::Stable(value) => *value,
            Self::Converging(old, new, phase) => *old + (*new - *old) * *phase,
        }
    }
}

impl Signal for SmoothedValue {
    type Frame = f32;

    fn next(&mut self) -> Self::Frame {
        if let Self::Converging(old, new, mut phase) = *self {
            phase += Self::STEP;
            if phase > 1.0 {
                *self = Self::Stable(new);
            } else {
                *self = Self::Converging(old, new, phase);
            }
        }
        self.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_smooth_value_when_left_intact_it_returns_stable_value() {
        let value = SmoothedValue::new(1.0);
        for x in value.take(10) {
            assert_relative_eq!(x, 1.0);
        }
    }

    #[test]
    fn given_smooth_value_when_sets_a_new_value_it_linearly_progresses_to_it_and_remains_stable() {
        let mut value = SmoothedValue::new(1.0);
        value.set(0.0);
        for (i, x) in value.by_ref().take(64).enumerate() {
            assert_relative_eq!(x, 1.0 - (i as f32 + 1.0) / 64.0);
        }
        for x in value.take(100) {
            assert_relative_eq!(x, 0.0);
        }
    }
}

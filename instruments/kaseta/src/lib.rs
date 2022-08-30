#![no_std]

use core::convert::TryFrom;
use core::fmt;

use proton_control::input_snapshot::InputSnapshot;
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

const NAME: &str = "Kaseta";
const PRE_AMP_ATTRIBUTE: &str = "pre-amp";
const DRIVE_ATTRIBUTE: &str = "drive";
const SATURATION_ATTRIBUTE: &str = "saturation";
const BIAS_ATTRIBUTE: &str = "bias";
const WOW_FREQUENCY_ATTRIBUTE: &str = "wow frequency";
const WOW_DEPTH_ATTRIBUTE: &str = "wow depth";

pub trait Rand {
    fn generate(&mut self) -> u16;
}

pub struct Instrument {
    _sample_rate: u32,
}

fn writter(destination: &mut dyn fmt::Write, value: f32) {
    let value = (value * 100.0) as u32;
    write!(destination, "{}%", value).unwrap();
}

impl Instrument {
    pub fn state(&self) -> State {
        State::new(NAME)
            .with_attributes(&[
                Attribute::new(PRE_AMP_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(0.5).with_writter(writter)),
                Attribute::new(DRIVE_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(0.5).with_writter(writter)),
                Attribute::new(SATURATION_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(0.5).with_writter(writter)),
                Attribute::new(BIAS_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(0.5).with_writter(writter)),
                Attribute::new(WOW_FREQUENCY_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(0.5).with_writter(writter)),
                Attribute::new(WOW_DEPTH_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(0.5).with_writter(writter)),
            ])
            .unwrap()
    }

    pub fn new(sample_rate: u32) -> Self {
        Self {
            _sample_rate: sample_rate,
        }
    }

    pub fn process(&mut self, _buffer: &mut [(f32, f32)], _randomizer: &mut impl Rand) {}

    pub fn execute(&mut self, _command: Command) {}

    pub fn update_control(&mut self, _snapshot: InputSnapshot) {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Foo,
}

impl TryFrom<Reaction> for Command {
    type Error = &'static str;

    fn try_from(_other: Reaction) -> Result<Self, Self::Error> {
        Ok(Command::Foo)
    }
}

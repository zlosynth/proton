#![no_std]

use core::convert::TryFrom;
use core::fmt;

use proton_control::input_snapshot::InputSnapshot;
use proton_instruments_interface::{Instrument as InstrumentTrait, MemoryManager};
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

use kaseta_dsp::processor::{Attributes, Processor};

const NAME: &str = "Kaseta";
const PRE_AMP_ATTRIBUTE: &str = "pre-amp";
const DRIVE_ATTRIBUTE: &str = "drive";
const SATURATION_ATTRIBUTE: &str = "saturation";
const BIAS_ATTRIBUTE: &str = "bias";
const WOW_FREQUENCY_ATTRIBUTE: &str = "wow frequency";
const WOW_DEPTH_ATTRIBUTE: &str = "wow depth";

pub struct Instrument {
    processor: Processor,
}

fn writter(destination: &mut dyn fmt::Write, value: f32) {
    let value = (value * 100.0) as u32;
    write!(destination, "{}%", value).unwrap();
}

impl InstrumentTrait for Instrument {
    type Command = Command;

    fn new(sample_rate: u32, memory_manager: &mut MemoryManager) -> Self {
        let mut processor = Processor::new(sample_rate as f32, memory_manager);
        processor.set_attributes(Attributes {
            pre_amp: 0.1,
            drive: 1.0,
            saturation: 0.5,
            width: 0.5,
            wow_frequency: 0.0,
            wow_depth: 0.0,
        });
        Self { processor }
    }

    fn state(&self) -> State {
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

    fn process(&mut self, buffer: &mut [(f32, f32)]) {
        for chunk in buffer.chunks_exact_mut(32) {
            let mut buffer = [0.0; 32];
            for (i, x) in chunk.iter_mut().enumerate() {
                buffer[i] = x.0;
            }
            self.processor.process(&mut buffer);
            for (i, x) in buffer.iter_mut().enumerate() {
                chunk[i] = (*x, 0.0);
            }
        }
    }

    fn execute(&mut self, _command: Command) {}

    fn update_control(&mut self, _snapshot: InputSnapshot) {}
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

#![no_std]

use core::convert::TryFrom;
use core::fmt;

use sirena::memory_manager::MemoryManager;

use proton_control::input_snapshot::InputSnapshot;
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

use kaseta_control::{self, Cache, ControlAction, DSPReaction};
use kaseta_dsp::processor::Processor;

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
    processor: Processor,
    cache: Cache,
}

fn writter(destination: &mut dyn fmt::Write, value: f32) {
    // write!(destination, "{:.2}%", value).unwrap();
    write!(destination, "%").unwrap();
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

    pub fn new(sample_rate: u32, memory_manager: &mut MemoryManager) -> Self {
        let cache = Cache::default();
        let processor = {
            let mut processor = Processor::new(sample_rate as f32, memory_manager);
            let attributes = kaseta_control::cook_dsp_reaction_from_cache(&cache).into();
            processor.set_attributes(attributes);
            processor
        };
        Self { processor, cache }
    }

    pub fn process(&mut self, buffer: &mut [(f32, f32)], _randomizer: &mut impl Rand) {
        const BUFFER_LEN: usize = 32;
        assert!(buffer.len() % BUFFER_LEN == 0);

        let mut chunk_buffer = [0.0; BUFFER_LEN];
        for chunk_index in 0..buffer.len() / BUFFER_LEN {
            for (i, frame) in chunk_buffer.iter_mut().enumerate() {
                let index = chunk_index * BUFFER_LEN + i;
                *frame = buffer[index].0;
            }

            // self.processor.process(&mut chunk_buffer);

            for (i, frame) in chunk_buffer.iter().enumerate() {
                let index = chunk_index * BUFFER_LEN + i;
                buffer[index].0 = *frame;
            }
        }
    }

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

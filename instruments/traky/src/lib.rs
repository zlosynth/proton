#![no_std]

use core::convert::TryFrom;
use core::fmt;

use proton_control::input_snapshot::InputSnapshot;
use proton_instruments_interface::{
    Instrument as InstrumentTrait, MemoryManager, Rand as ProtonRandomizer,
};
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

const NAME: &str = "Traky";
const VOLUME_ATTRIBUTE: &str = "volume";

pub struct Instrument {}

fn writter(destination: &mut dyn fmt::Write, value: f32) {
    let value = (value * 100.0) as u32;
    write!(destination, "{}%", value).unwrap();
}

impl InstrumentTrait for Instrument {
    type Command = Command;

    fn new(_sample_rate: u32, _memory_manager: &mut MemoryManager) -> Self {
        defmt::info!("NEW");
        Self {}
    }

    fn state(&self) -> State {
        State::new(NAME)
            .with_attributes(&[Attribute::new(VOLUME_ATTRIBUTE)
                .with_value_f32(ValueF32::new(1.0).with_writter(writter))])
            .unwrap()
    }

    fn process(&mut self, buffer: &mut [(f32, f32)], _randomizer: &mut impl ProtonRandomizer) {
        for chunk in buffer.chunks_exact_mut(32) {
            let mut buffer = [(0.0, 0.0); 32];
            for (i, x) in chunk.iter_mut().enumerate() {
                buffer[i] = *x;
            }
            for (i, x) in buffer.iter_mut().enumerate() {
                chunk[i] = *x;
            }
        }
    }

    fn execute(&mut self, _command: Command) {}

    fn update_control(&mut self, _snapshot: InputSnapshot) {}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    SetVolume(f32),
}

impl TryFrom<Reaction> for Command {
    type Error = ();

    fn try_from(other: Reaction) -> Result<Self, Self::Error> {
        match other {
            Reaction::SetValue(VOLUME_ATTRIBUTE, value) => Ok(Command::SetVolume(value)),
            _ => Err(()),
        }
    }
}

#![no_std]

use core::convert::TryFrom;

use proton_primitives::state_variable_filter::{Bandform, StateVariableFilter};
use proton_primitives::white_noise;
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

const NAME: &str = "Karplus Strong";
const CUTOFF_FREQUENCY_ATTRIBUTE: &str = "cutoff";

pub struct Instrument {
    svf: StateVariableFilter,
}

impl Instrument {
    pub fn initial_state() -> State {
        State::new(NAME)
            .with_attributes(&[
                Attribute::new(CUTOFF_FREQUENCY_ATTRIBUTE).with_value_f32(ValueF32::new(0.3))
            ])
            .unwrap()
    }

    pub fn new(sample_rate: u32) -> Self {
        let mut svf = StateVariableFilter::new(sample_rate);
        svf.set_bandform(Bandform::LowPass).set_frequency(200.0);
        Self { svf }
    }

    pub fn populate(&mut self, buffer: &mut [f32]) {
        for x in buffer.iter_mut() {
            *x = self.svf.tick(white_noise::pop());
        }
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::SetCutoffFrequency(value) => {
                self.svf.set_frequency(value);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    SetCutoffFrequency(f32),
}

impl TryFrom<Reaction> for Command {
    type Error = &'static str;

    fn try_from(other: Reaction) -> Result<Self, Self::Error> {
        match other {
            Reaction::SetValue(attribute, value) => {
                if attribute == CUTOFF_FREQUENCY_ATTRIBUTE {
                    Ok(Command::SetCutoffFrequency(value * 1000.0))
                } else {
                    Err("cannot convert this reaction to a command")
                }
            }
            _ => Err("cannot convert this reaction to a command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(
        Reaction::SetValue("invalid", 0.0) =>
        matches Err(_)
    )]
    #[test_case(
        Reaction::SetValue(CUTOFF_FREQUENCY_ATTRIBUTE, 0.0) =>
        Ok(Command::SetCutoffFrequency(0.0))
    )]
    #[test_case(
        Reaction::SetValue(CUTOFF_FREQUENCY_ATTRIBUTE, 0.1) =>
        Ok(Command::SetCutoffFrequency(100.0))
    )]
    fn it_converts_reaction_to_command(reaction: Reaction) -> Result<Command, &'static str> {
        reaction.try_into()
    }
}

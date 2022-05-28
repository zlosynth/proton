#![no_std]

use core::convert::TryFrom;

use proton_primitives::ad_envelope::{Ad, Config as AdConfig};
use proton_primitives::ring_buffer::RingBuffer;
use proton_primitives::state_variable_filter::{Bandform, StateVariableFilter};
use proton_primitives::white_noise::WhiteNoise;
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

const NAME: &str = "Karplus Strong";
const CUTOFF_FREQUENCY_ATTRIBUTE: &str = "cutoff";

const MAX_SAMPLE_RATE: u32 = 48_000;
const MIN_FREQUENCY: f32 = 40.0;
const SAMPLES: usize = (MAX_SAMPLE_RATE as f32 / MIN_FREQUENCY) as usize;

pub struct Instrument {
    svf: StateVariableFilter,
    noise: WhiteNoise,
    envelope: Ad,
    ring_buffer: RingBuffer<SAMPLES>,
    sample_rate: u32,
    countdown: u32,
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
        assert!(
            sample_rate <= MAX_SAMPLE_RATE,
            "maximum supported sample rate is 48 kHz"
        );

        let svf = {
            let mut svf = StateVariableFilter::new(sample_rate);
            svf.set_bandform(Bandform::LowPass).set_frequency(1000.0);
            svf
        };
        let noise = WhiteNoise::new();
        let envelope = Ad::new(sample_rate as f32);
        let ring_buffer = RingBuffer::new();

        Self {
            svf,
            noise,
            envelope,
            ring_buffer,
            sample_rate,
            countdown: u32::MAX,
        }
    }

    pub fn populate(&mut self, buffer: &mut [f32]) {
        for x in buffer.iter_mut() {
            if self.countdown > self.sample_rate / 2 {
                self.envelope
                    .trigger(AdConfig::new().with_decay_time(0.005));
                self.countdown = 0;
            }
            self.countdown += 1;

            let new_sample = self.noise.pop() * self.envelope.pop();
            // TODO: Interpolate delayed samples
            let delayed_sample = self.ring_buffer.peek(self.sample_rate as i32 / -500);
            let mixed_sample = self.svf.tick(new_sample + delayed_sample * 0.99);
            self.ring_buffer.write(mixed_sample);

            *x = mixed_sample;
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
                    Ok(Command::SetCutoffFrequency(value * 5000.0))
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
        Ok(Command::SetCutoffFrequency(500.0))
    )]
    fn it_converts_reaction_to_command(reaction: Reaction) -> Result<Command, &'static str> {
        reaction.try_into()
    }
}
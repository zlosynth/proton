#![no_std]

use core::convert::TryFrom;
use core::fmt;

use proton_primitives::ad_envelope::{Ad, Config as AdConfig};
use proton_primitives::ring_buffer::RingBuffer;
use proton_primitives::state_variable_filter::{Bandform, StateVariableFilter};
use proton_primitives::white_noise::WhiteNoise;
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

const NAME: &str = "Karplus Strong";
const FREQUENCY_ATTRIBUTE: &str = "frequency";
const CUTOFF_ATTRIBUTE: &str = "cutoff";
const FEEDBACK_ATTRIBUTE: &str = "feedback";

const MAX_SAMPLE_RATE: u32 = 48_000;
const MIN_FREQUENCY: f32 = 40.0;
const SAMPLES: usize = (MAX_SAMPLE_RATE as f32 / MIN_FREQUENCY) as usize;

pub struct Instrument {
    svf: StateVariableFilter,
    noise: WhiteNoise,
    envelope: Ad,
    ring_buffer: RingBuffer<SAMPLES>,
    turing: Turing,
    frequency: f32,
    feedback: f32,
    sample_rate: u32,
}

fn feedback_writter(destination: &mut dyn fmt::Write, value: f32) {
    write!(destination, "{:.3}", value).unwrap();
}

impl Instrument {
    pub fn initial_state() -> State {
        State::new(NAME)
            .with_attributes(&[
                Attribute::new(FREQUENCY_ATTRIBUTE).with_value_f32(ValueF32::new(0.5)), // TODO: use default value
                Attribute::new(CUTOFF_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(0.3).with_step(0.005)),
                Attribute::new(FEEDBACK_ATTRIBUTE).with_value_f32(
                    ValueF32::new(0.9)
                        .with_min(0.6)
                        .with_max(1.0)
                        .with_step(0.005)
                        .with_writter(feedback_writter),
                ),
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
            turing: Turing::new(sample_rate),
            frequency: 100.0,
            feedback: 0.9,
            sample_rate,
        }
    }

    pub fn populate(&mut self, buffer: &mut [f32]) {
        let config = self.turing.tick(buffer.len() as u32);

        for x in buffer.iter_mut() {
            if config.triggered {
                self.envelope.trigger(
                    AdConfig::new().with_decay_time(self.frequency / self.sample_rate as f32),
                );
            }

            let new_sample = self.noise.pop() * self.envelope.pop();
            let delayed_sample = self
                .ring_buffer
                .peek_interpolated(-(self.sample_rate as f32) / self.frequency);
            let mixed_sample = self.svf.tick(new_sample + delayed_sample * self.feedback);
            self.ring_buffer.write(mixed_sample);

            *x = mixed_sample;
        }
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::SetCutoff(value) => {
                self.svf.set_frequency(value);
            }
            Command::SetFrequency(value) => {
                self.frequency = value;
            }
            Command::SetFeedback(value) => {
                self.feedback = value;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    SetFrequency(f32),
    SetCutoff(f32),
    SetFeedback(f32),
}

impl TryFrom<Reaction> for Command {
    type Error = &'static str;

    fn try_from(other: Reaction) -> Result<Self, Self::Error> {
        match other {
            Reaction::SetValue(attribute, value) => {
                if attribute == CUTOFF_ATTRIBUTE {
                    Ok(Command::SetCutoff(value * 5000.0))
                } else if attribute == FREQUENCY_ATTRIBUTE {
                    Ok(Command::SetFrequency(value * 1000.0))
                } else if attribute == FEEDBACK_ATTRIBUTE {
                    Ok(Command::SetFeedback(value))
                } else {
                    Err("cannot convert this reaction to a command")
                }
            }
            _ => Err("cannot convert this reaction to a command"),
        }
    }
}

struct Turing {
    sample_rate: u32,
    bpm: f32,
    triggers: [u32; 6],
    phase: u32,
}

impl Turing {
    const CELLS_IN_BEAT: u32 = 3 * 4;
    const CELLS: u32 = Self::CELLS_IN_BEAT * 16;

    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            bpm: 120.0,
            triggers: [
                0b1000_1000_0000_1000_1000_1000_1000_1000,
                0b1000_0000_0000_1000_1000_1000_1000_1000,
                0b1000_1000_0000_1000_1000_1000_1000_1000,
                0b1000_0000_0000_1000_1000_1000_1000_1000,
                0b1000_1000_0000_1000_1000_1000_1000_1000,
                0b1000_0000_0000_1000_1000_1000_1000_1000,
            ],
            phase: 0,
        }
    }

    // NOTE: In theory this tick may miss some triggers when bpm is too high.
    // However, in reality this can be safely ignored:
    //
    // With sample rate of 48 kHz, buffer length of 32 samples, tick would be
    // triggered every 1/1500 of a second.
    //
    // With BPM of 600 and beat resolution of 12 cells, each cell would last
    // 1/120 of a second.
    pub fn tick(&mut self, samples: u32) -> Config {
        let seconds_per_beat = 60.0 / self.bpm;
        let seconds_per_cell = seconds_per_beat / Self::CELLS_IN_BEAT as f32;
        let cell_in_samples = seconds_per_cell * self.sample_rate as f32;

        let old_tick = self.phase / cell_in_samples as u32;

        self.phase += samples;
        self.phase %= cell_in_samples as u32 * Self::CELLS;

        let new_tick = self.phase / cell_in_samples as u32;

        let triggered = if new_tick != old_tick {
            is_nth_tick_on(&self.triggers, new_tick as usize)
        } else {
            false
        };

        Config { triggered }
    }
}

fn is_nth_tick_on(triggers: &[u32; 6], tick_index: usize) -> bool {
    let (field_index, tick_index) = {
        let quotient = tick_index / 32;
        (quotient, tick_index - 32 * quotient)
    };
    triggers[field_index] << tick_index & (1 << 31) != 0
}

struct Config {
    triggered: bool,
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
        Reaction::SetValue(CUTOFF_ATTRIBUTE, 0.0) =>
        Ok(Command::SetCutoff(0.0))
    )]
    #[test_case(
        Reaction::SetValue(CUTOFF_ATTRIBUTE, 0.1) =>
        Ok(Command::SetCutoff(500.0))
    )]
    #[test_case(
        Reaction::SetValue(FREQUENCY_ATTRIBUTE, 0.1) =>
        Ok(Command::SetFrequency(100.0))
    )]
    #[test_case(
        Reaction::SetValue(FEEDBACK_ATTRIBUTE, 0.95) =>
        Ok(Command::SetFeedback(0.95))
    )]
    fn it_converts_reaction_to_command(reaction: Reaction) -> Result<Command, &'static str> {
        reaction.try_into()
    }

    #[test]
    fn is_nth_tick_on_returns_true_on_enabled_tick() {
        let triggers = [
            0b1000_0000_0000_0000_0000_0000_0000_0000,
            //^ 0
            0b0000_0000_0000_0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000_0000_1000_0001,
            //                              ^ 88    ^ 95
            0b0000_0000_0000_0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000_0000_0000_0000,
            0b0000_0000_0000_0000_0000_0000_0000_0000,
        ];

        assert!(is_nth_tick_on(&triggers, 0));
        assert!(is_nth_tick_on(&triggers, 88));
        assert!(is_nth_tick_on(&triggers, 95));
    }

    #[test]
    fn is_nth_tick_on_returns_false_on_disabled_tick() {
        let triggers = [
            0b1000_1000_1000_1000_1000_1000_1000_1000,
            // ^ 1
            0b1000_1000_1000_1000_1000_1000_1000_1000,
            0b1000_1000_1000_1000_1000_1000_1000_1000,
            //                               ^ 89
            0b1000_1000_1000_1000_1000_1000_1000_1000,
            0b1000_1000_1000_1000_1000_1000_1000_1000,
            0b1000_1000_1000_1000_1000_1000_1000_1000,
        ];

        assert!(!is_nth_tick_on(&triggers, 1));
        assert!(!is_nth_tick_on(&triggers, 89));
    }
}

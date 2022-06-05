#![no_std]

use core::convert::TryFrom;
use core::fmt;

use heapless::Vec;

use proton_primitives::ad_envelope::{Ad, Config as AdConfig};
use proton_primitives::ring_buffer::RingBuffer;
use proton_primitives::state_variable_filter::{Bandform, StateVariableFilter};
use proton_primitives::white_noise::WhiteNoise;
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

const NAME: &str = "Karplus Strong";
const FREQUENCY_ATTRIBUTE: &str = "frequency";
const FREQUENCY_DEFAULT: f32 = 100.0;
const CUTOFF_ATTRIBUTE: &str = "cutoff";
const CUTOFF_DEFAULT: f32 = 1000.0;
const FEEDBACK_ATTRIBUTE: &str = "feedback";
const FEEDBACK_DEFAULT: f32 = 0.95;

const MAX_SAMPLE_RATE: u32 = 48_000;
const MIN_FREQUENCY: f32 = 40.0;
const SAMPLES: usize = (MAX_SAMPLE_RATE as f32 / MIN_FREQUENCY) as usize;

pub trait Rand {
    fn generate(&mut self) -> u16;
}

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

fn frequency_writter(destination: &mut dyn fmt::Write, value: f32) {
    write!(destination, "{:.0}", value).unwrap();
}

fn feedback_writter(destination: &mut dyn fmt::Write, value: f32) {
    write!(destination, "{:.3}", value).unwrap();
}

impl Instrument {
    pub fn initial_state() -> State {
        State::new(NAME)
            .with_attributes(&[
                Attribute::new(FREQUENCY_ATTRIBUTE).with_value_f32(
                    ValueF32::new(FREQUENCY_DEFAULT)
                        .with_min(50.0)
                        .with_max(10000.0)
                        .with_step(10.0)
                        .with_writter(frequency_writter),
                ),
                Attribute::new(CUTOFF_ATTRIBUTE).with_value_f32(
                    ValueF32::new(CUTOFF_DEFAULT)
                        .with_min(50.0)
                        .with_max(10000.0)
                        .with_step(10.0)
                        .with_writter(frequency_writter),
                ),
                Attribute::new(FEEDBACK_ATTRIBUTE).with_value_f32(
                    ValueF32::new(FEEDBACK_DEFAULT)
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
            svf.set_bandform(Bandform::LowPass)
                .set_frequency(CUTOFF_DEFAULT);
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
            frequency: FREQUENCY_DEFAULT,
            feedback: FEEDBACK_DEFAULT,
            sample_rate,
        }
    }

    pub fn populate(&mut self, buffer: &mut [f32], randomizer: &mut impl Rand) {
        let config = self.turing.tick(buffer.len() as u32, randomizer);

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
                    Ok(Command::SetCutoff(value))
                } else if attribute == FREQUENCY_ATTRIBUTE {
                    Ok(Command::SetFrequency(value))
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
    triggers: [u32; 3],
    phase: u32,
    density: u32,
    rate_of_change: f32,
}

impl Turing {
    const CELLS_IN_BEAT: u32 = 2 * 3;
    const CELLS: u32 = Self::CELLS_IN_BEAT * 16;

    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            bpm: 360.0,
            triggers: [
                0b0000_0000_0000_0000_0000_0000_0000_0000,
                0b0000_0000_0000_0000_0000_0000_0000_0000,
                0b0000_0000_0000_0000_0000_0000_0000_0000,
            ],
            phase: 0,
            density: 16,
            rate_of_change: 4.0,
        }
    }

    // NOTE: In theory this tick may miss some triggers when BPM is too high.
    // However, in reality this can be safely ignored:
    //
    // With sample rate of 48 kHz, buffer length of 32 samples, tick would be
    // triggered every 1/1500 of a second.
    //
    // With BPM of 600 and beat resolution of 6 cells, each cell would last
    // 1/60 of a second.
    pub fn tick(&mut self, samples: u32, randomizer: &mut impl Rand) -> Config {
        let seconds_per_beat = 60.0 / self.bpm;
        let seconds_per_cell = seconds_per_beat / Self::CELLS_IN_BEAT as f32;
        let cell_in_samples = seconds_per_cell * self.sample_rate as f32;

        let old_tick = self.phase / cell_in_samples as u32;

        self.phase += samples;

        if self.phase >= cell_in_samples as u32 * Self::CELLS {
            self.randomize(randomizer);
            self.phase %= cell_in_samples as u32 * Self::CELLS;
        }

        let new_tick = self.phase / cell_in_samples as u32;

        let triggered = if new_tick != old_tick {
            is_nth_tick_on(&self.triggers, new_tick as usize)
        } else {
            false
        };

        Config { triggered }
    }

    fn randomize(&mut self, randomizer: &mut impl Rand) {
        use core::cmp::Ordering;

        let mut ticks_on = find_ticks_on(&self.triggers);
        let delta = ticks_on.len() as i32 - self.density as i32;

        let (add, remove) = match delta.cmp(&0) {
            Ordering::Less => (delta.abs().min(self.rate_of_change as i32), 0),
            Ordering::Equal => (self.rate_of_change as i32, self.rate_of_change as i32),
            Ordering::Greater => (0, delta.abs().min(self.rate_of_change as i32)),
        };

        for _ in 0..remove {
            let index = {
                let rand = randomizer.generate() as usize;
                ticks_on.swap_remove(rand % ticks_on.len())
            };
            set_nth_tick_off(&mut self.triggers, index);
        }

        for _ in 0..add {
            let length_in_cells = {
                let rand = randomizer.generate() as usize;
                let length = NoteLength::from_index(rand % NoteLength::LEN);
                length.in_cells()
            };
            let position = (randomizer.generate() as u32 % Self::CELLS) / length_in_cells;
            place_note(
                position as usize * length_in_cells as usize,
                length_in_cells,
                &mut self.triggers,
            );
        }
    }
}

#[derive(Clone, Copy)]
struct Config {
    triggered: bool,
}

#[derive(Clone, Copy)]
enum NoteLength {
    Whole,
    HalfTriplet,
    Half,
    QuarterTriplet,
    Quarter,
    EightTriplet,
    Eight,
}
use NoteLength::*;

impl NoteLength {
    const LEN: usize = 7;

    fn from_index(index: usize) -> Self {
        match index {
            0 => Whole,
            1 => HalfTriplet,
            2 => Half,
            3 => QuarterTriplet,
            4 => Quarter,
            5 => EightTriplet,
            6 => Eight,
            _ => panic!("no such length"),
        }
    }

    fn in_cells(&self) -> u32 {
        match self {
            Whole => Turing::CELLS_IN_BEAT * 4,
            HalfTriplet => Turing::CELLS_IN_BEAT * 3,
            Half => Turing::CELLS_IN_BEAT * 2,
            QuarterTriplet => (Turing::CELLS_IN_BEAT * 4) / 3,
            Quarter => Turing::CELLS_IN_BEAT,
            EightTriplet => (Turing::CELLS_IN_BEAT * 2) / 3,
            Eight => Turing::CELLS_IN_BEAT / 2,
        }
    }
}

fn place_note(start: usize, length: u32, triggers: &mut [u32; 3]) {
    reset_range(start, start + length as usize - 1, triggers);
    set_nth_tick_on(triggers, start);
}

fn reset_range(left: usize, right: usize, triggers: &mut [u32; 3]) {
    fn reset_range_in_u32(left: usize, right: usize, value: u32) -> u32 {
        let left = 31 - left;
        let right = 31 - right;
        let mask = ((1u64 << ((left - right) + 1)) - 1) << right;
        value & !(mask as u32)
    }

    let block_left = left / 32;
    let block_right = right / 32;

    let is_fully_within_block = block_left == block_right;

    let left_index_within_block = left - block_left * 32;
    let right_index_within_block = if is_fully_within_block {
        right - block_right * 32
    } else {
        31
    };
    triggers[block_left] = reset_range_in_u32(
        left_index_within_block,
        right_index_within_block,
        triggers[block_left],
    );

    let is_last = block_left == triggers.len() - 1;
    if is_fully_within_block || is_last {
        return;
    }

    reset_range((block_left + 1) * 32, right, triggers);
}

fn is_nth_tick_on(triggers: &[u32; 3], tick_index: usize) -> bool {
    let (field_index, tick_index) = {
        let quotient = tick_index / 32;
        (quotient, tick_index - 32 * quotient)
    };
    triggers[field_index] << tick_index & (1 << 31) != 0
}

fn set_nth_tick_on(triggers: &mut [u32; 3], tick_index: usize) {
    let tick_block = tick_index / 32;
    let tick_index_within_block = tick_index - tick_block * 32;
    triggers[tick_block] |= 1 << (31 - tick_index_within_block);
}

fn set_nth_tick_off(triggers: &mut [u32; 3], tick_index: usize) {
    let tick_block = tick_index / 32;
    let tick_index_within_block = tick_index - tick_block * 32;
    triggers[tick_block] &= !(1 << (31 - tick_index_within_block));
}

fn find_ticks_on(triggers: &[u32; 3]) -> Vec<usize, 32> {
    (0..96_usize)
        .filter(|i| is_nth_tick_on(triggers, *i))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    struct TestRand(u16);

    impl Rand for TestRand {
        fn generate(&mut self) -> u16 {
            self.0
        }
    }

    #[test_case(
        Reaction::SetValue("invalid", 0.0) =>
        matches Err(_)
    )]
    #[test_case(
        Reaction::SetValue(CUTOFF_ATTRIBUTE, 0.0) =>
        Ok(Command::SetCutoff(0.0))
    )]
    #[test_case(
        Reaction::SetValue(CUTOFF_ATTRIBUTE, 5.0) =>
        Ok(Command::SetCutoff(5.0))
    )]
    #[test_case(
        Reaction::SetValue(FREQUENCY_ATTRIBUTE, 300.0) =>
        Ok(Command::SetFrequency(300.0))
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
        ];

        assert!(!is_nth_tick_on(&triggers, 1));
        assert!(!is_nth_tick_on(&triggers, 89));
    }

    #[test]
    fn count_ticks_on_should_return_number_of_set_bits() {
        let triggers = [
            0b1000_0000_0000_0000_0000_0000_0000_0001,
            0b0000_0000_0000_0000_0000_0000_0001_0000,
            0b0000_0000_0000_0001_0000_0000_0000_0000,
        ];

        assert_eq!(find_ticks_on(&triggers), &[0, 31, 59, 79]);
    }

    #[test]
    fn place_whole_note_on_empty_triggers() {
        let mut triggers = [0; 3];
        place_note(4, 48, &mut triggers);
        assert_eq!(triggers[0], 0b0000_1000_0000_0000_0000_0000_0000_0000);
        for i in 1..3 {
            assert_eq!(triggers[i], 0);
        }
    }

    #[test]
    fn place_whole_note_on_populated_triggers() {
        let mut triggers = [u32::MAX; 3];
        place_note(4, 48, &mut triggers);
        assert_eq!(triggers[0], 0b1111_1000_0000_0000_0000_0000_0000_0000);
        assert_eq!(triggers[1], 0b0000_0000_0000_0000_0000_1111_1111_1111);
        for i in 2..3 {
            assert_eq!(triggers[i], u32::MAX);
        }
    }

    #[test]
    fn reset_range_at_beginning() {
        let mut triggers = [u32::MAX; 3];
        reset_range(0, 3, &mut triggers);
        assert_eq!(triggers[0], 0b0000_1111_1111_1111_1111_1111_1111_1111);
        for i in 1..3 {
            assert_eq!(triggers[i], u32::MAX);
        }
    }

    #[test]
    fn reset_range_at_end_of_block() {
        let mut triggers = [u32::MAX; 3];
        reset_range(28, 31, &mut triggers);
        assert_eq!(triggers[0], 0b1111_1111_1111_1111_1111_1111_1111_0000);
        for i in 1..3 {
            assert_eq!(triggers[i], u32::MAX);
        }
    }

    #[test]
    fn reset_range_in_the_middle_of_block() {
        let mut triggers = [u32::MAX; 3];
        reset_range(8, 27, &mut triggers);
        assert_eq!(triggers[0], 0b1111_1111_0000_0000_0000_0000_0000_1111);
        for i in 1..3 {
            assert_eq!(triggers[i], u32::MAX);
        }
    }

    #[test]
    fn reset_range_crossing_two_blocks() {
        let mut triggers = [u32::MAX; 3];
        reset_range(28, 35, &mut triggers);
        assert_eq!(triggers[0], 0b1111_1111_1111_1111_1111_1111_1111_0000);
        assert_eq!(triggers[1], 0b0000_1111_1111_1111_1111_1111_1111_1111);
        for i in 2..3 {
            assert_eq!(triggers[i], u32::MAX);
        }
    }

    #[test]
    fn reset_range_end_to_end() {
        let mut triggers = [u32::MAX; 3];
        reset_range(0, 191, &mut triggers);
        for i in 0..3 {
            assert_eq!(triggers[i], 0);
        }
    }

    #[test]
    fn set_nth_tick_on_in_middle() {
        let mut triggers = [0; 3];
        set_nth_tick_on(&mut triggers, 84);
        for i in 0..2 {
            assert_eq!(triggers[i], 0);
        }
        assert_eq!(triggers[2], 0b0000_0000_0000_0000_0000_1000_0000_0000);
    }

    #[test]
    fn set_nth_tick_off_in_middle() {
        let mut triggers = [u32::MAX; 3];
        set_nth_tick_off(&mut triggers, 84);
        for i in 0..2 {
            assert_eq!(triggers[i], u32::MAX);
        }
        assert_eq!(triggers[2], 0b1111_1111_1111_1111_1111_0111_1111_1111);
    }

    #[test]
    fn turing_should_work_without_panic() {
        let mut turing = Turing::new(48_000);
        for i in 0..48_000 * 10 {
            turing.tick(64, &mut TestRand(i as u16));
        }
    }

    #[test]
    fn note_lengths_are_proportional() {
        assert_eq!(
            NoteLength::Half.in_cells() * 2,
            NoteLength::Whole.in_cells()
        );
        assert_eq!(
            NoteLength::QuarterTriplet.in_cells() * 3,
            NoteLength::Whole.in_cells()
        );
        assert_eq!(
            NoteLength::Quarter.in_cells() * 4,
            NoteLength::Whole.in_cells()
        );
        assert_eq!(
            NoteLength::EightTriplet.in_cells() * 6,
            NoteLength::Whole.in_cells()
        );
        assert_eq!(
            NoteLength::Eight.in_cells() * 8,
            NoteLength::Whole.in_cells()
        );
    }

    #[test]
    fn confirm_that_note_length_len_is_correct_with_valid_indices() {
        for i in 0..NoteLength::LEN {
            let _ = NoteLength::from_index(i);
        }
    }

    #[test]
    #[should_panic]
    fn confirm_that_note_length_len_is_correct_with_invalid_index() {
        let _ = NoteLength::from_index(NoteLength::LEN);
    }
}

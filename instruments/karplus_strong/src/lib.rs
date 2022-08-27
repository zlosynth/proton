#![no_std]

#[allow(unused_imports)]
use micromath::F32Ext;

use core::convert::TryFrom;
use core::fmt;

use heapless::FnvIndexMap as IndexMap;
use heapless::Vec;

use proton_control::input_snapshot::InputSnapshot;
use proton_primitives::ad_envelope::{Ad, Config as AdConfig};
use proton_primitives::ring_buffer::RingBuffer;
use proton_primitives::state_variable_filter::{Bandform, StateVariableFilter};
use proton_primitives::white_noise::WhiteNoise;
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

const NAME: &str = "Karplus Strong";

const CUTOFF_ATTRIBUTE: &str = "cutoff";
const CUTOFF_DEFAULT: f32 = 1000.0;

const FEEDBACK_ATTRIBUTE: &str = "feedback";
const FEEDBACK_DEFAULT: f32 = 0.95;

const DENSITY_ATTRIBUTE: &str = "density";
const DENSITY_DEFAULT: f32 = 4.0;

const CHANGE_ATTRIBUTE: &str = "change";
const CHANGE_DEFAULT: f32 = 1.0;

const BEATS_ATTRIBUTE: &str = "beats";
const BEATS_DEFAULT: f32 = 4.0;

const OFF_ON: [&str; 2] = ["off", "on"];
const WHOLE_ATTRIBUTE: &str = "whole";
const WHOLE_DEFAULT: usize = 0;
const HALF_TRIPLET_ATTRIBUTE: &str = "half triplet";
const HALF_TRIPLET_DEFAULT: usize = 0;
const HALF_ATTRIBUTE: &str = "half";
const HALF_DEFAULT: usize = 0;
const QUARTER_TRIPLET_ATTRIBUTE: &str = "quarter triplet";
const QUARTER_TRIPLET_DEFAULT: usize = 0;
const QUARTER_ATTRIBUTE: &str = "quarter";
const QUARTER_DEFAULT: usize = 1;
const EIGHT_TRIPLET_ATTRIBUTE: &str = "eight triplet";
const EIGHT_TRIPLET_DEFAULT: usize = 0;
const EIGHT_ATTRIBUTE: &str = "eight";
const EIGHT_DEFAULT: usize = 0;

const MAX_SAMPLE_RATE: u32 = 48_000;
const MIN_FREQUENCY: f32 = 10.0;
const SAMPLES: usize = (MAX_SAMPLE_RATE as f32 / MIN_FREQUENCY) as usize;

const A: f32 = 12.978_271;

pub trait Rand {
    fn generate(&mut self) -> u16;
}

pub struct Instrument {
    svf: StateVariableFilter,
    noise: WhiteNoise,
    envelope: Ad,
    ring_buffer: RingBuffer<SAMPLES>,
    turing: Turing,
    cutoff_ui: f32,
    cutoff_cv: f32,
    feedback_ui: f32,
    feedback_cv: f32,
    frequency: f32,
    sample_rate: u32,
}

fn int_writter(destination: &mut dyn fmt::Write, value: f32) {
    write!(destination, "{:.0}", value).unwrap();
}

fn f3_writter(destination: &mut dyn fmt::Write, value: f32) {
    write!(destination, "{:.3}", value).unwrap();
}

impl Instrument {
    pub fn state(&self) -> State {
        State::new(NAME)
            .with_attributes(&[
                Attribute::new(CUTOFF_ATTRIBUTE).with_value_f32(
                    ValueF32::new(CUTOFF_DEFAULT)
                        .with_min(50.0)
                        .with_max(10000.0)
                        .with_step(10.0)
                        .with_writter(int_writter),
                ),
                Attribute::new(FEEDBACK_ATTRIBUTE).with_value_f32(
                    ValueF32::new(FEEDBACK_DEFAULT)
                        .with_min(0.6)
                        .with_max(1.0)
                        .with_step(0.005)
                        .with_writter(f3_writter),
                ),
                Attribute::new(DENSITY_ATTRIBUTE).with_value_f32(
                    ValueF32::new(DENSITY_DEFAULT)
                        .with_min(0.0)
                        .with_max(16.0)
                        .with_step(1.0)
                        .with_writter(int_writter),
                ),
                Attribute::new(CHANGE_ATTRIBUTE).with_value_f32(
                    ValueF32::new(CHANGE_DEFAULT)
                        .with_min(0.0)
                        .with_max(4.0)
                        .with_step(1.0)
                        .with_writter(int_writter),
                ),
                Attribute::new(BEATS_ATTRIBUTE).with_value_f32(
                    ValueF32::new(BEATS_DEFAULT)
                        .with_min(1.0)
                        .with_max(16.0)
                        .with_step(1.0)
                        .with_writter(int_writter),
                ),
                Attribute::new(WHOLE_ATTRIBUTE).with_value_select(
                    ValueSelect::new(&OFF_ON)
                        .unwrap()
                        .with_selected(WHOLE_DEFAULT),
                ),
                Attribute::new(HALF_TRIPLET_ATTRIBUTE).with_value_select(
                    ValueSelect::new(&OFF_ON)
                        .unwrap()
                        .with_selected(HALF_TRIPLET_DEFAULT),
                ),
                Attribute::new(HALF_ATTRIBUTE).with_value_select(
                    ValueSelect::new(&OFF_ON)
                        .unwrap()
                        .with_selected(HALF_DEFAULT),
                ),
                Attribute::new(QUARTER_TRIPLET_ATTRIBUTE).with_value_select(
                    ValueSelect::new(&OFF_ON)
                        .unwrap()
                        .with_selected(QUARTER_TRIPLET_DEFAULT),
                ),
                Attribute::new(QUARTER_ATTRIBUTE).with_value_select(
                    ValueSelect::new(&OFF_ON)
                        .unwrap()
                        .with_selected(QUARTER_DEFAULT),
                ),
                Attribute::new(EIGHT_TRIPLET_ATTRIBUTE).with_value_select(
                    ValueSelect::new(&OFF_ON)
                        .unwrap()
                        .with_selected(EIGHT_TRIPLET_DEFAULT),
                ),
                Attribute::new(EIGHT_ATTRIBUTE).with_value_select(
                    ValueSelect::new(&OFF_ON)
                        .unwrap()
                        .with_selected(EIGHT_DEFAULT),
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
        let turing = {
            let mut turing = Turing::new(sample_rate);
            turing.density = DENSITY_DEFAULT as u32;
            turing.rate_of_change = CHANGE_DEFAULT;
            turing.beats = BEATS_DEFAULT as u32;
            turing.lengths.push(NoteLength::Quarter).unwrap();
            turing
        };

        Self {
            svf,
            noise,
            envelope,
            ring_buffer,
            turing,
            feedback_ui: FEEDBACK_DEFAULT,
            feedback_cv: 0.0,
            cutoff_ui: CUTOFF_DEFAULT,
            cutoff_cv: 0.0,
            frequency: 1000.0,
            sample_rate,
        }
    }

    pub fn process(&mut self, buffer: &mut [(f32, f32)], randomizer: &mut impl Rand) {
        let config = self.turing.tick(buffer.len() as u32, randomizer);
        if config.frequency > 0.1 {
            self.frequency = config.frequency;
        }

        for (x, _) in buffer.iter_mut() {
            if config.triggered {
                self.envelope.trigger(
                    AdConfig::new().with_decay_time(self.frequency / self.sample_rate as f32),
                );
            }

            let new_sample = self.noise.pop() * self.envelope.pop();
            let delayed_sample = self
                .ring_buffer
                .peek_interpolated(-(self.sample_rate as f32) / self.frequency);
            let mixed_sample = self.svf.tick(new_sample + delayed_sample * self.feedback());
            self.ring_buffer.write(mixed_sample);

            *x = mixed_sample;
        }
    }

    pub fn execute(&mut self, command: Command) {
        fn add_length(lengths: &mut Vec<NoteLength, { NoteLength::LEN }>, length: NoteLength) {
            if !lengths.iter().enumerate().any(|(_, l)| *l == length) {
                lengths.push(length).unwrap();
            }
        }
        fn remove_length(lengths: &mut Vec<NoteLength, { NoteLength::LEN }>, length: NoteLength) {
            if let Some((index, _)) = lengths.iter().enumerate().find(|(_, l)| **l == length) {
                lengths.swap_remove(index);
            }
        }

        match command {
            Command::SetCutoff(value) => {
                self.cutoff_ui = value;
                self.svf.set_frequency(self.cutoff());
            }
            Command::SetFeedback(value) => {
                self.feedback_ui = value;
            }
            Command::SetDensity(value) => {
                self.turing.density = value as u32;
            }
            Command::SetChange(value) => {
                self.turing.rate_of_change = value;
            }
            Command::SetBeats(value) => {
                self.turing.beats = value;
            }
            Command::EnableLength(length) => add_length(&mut self.turing.lengths, length),
            Command::DisableLength(length) => remove_length(&mut self.turing.lengths, length),
        }
    }

    pub fn update_control(&mut self, snapshot: InputSnapshot) {
        self.cutoff_cv = snapshot.cv[0].value * 5000.0;
        self.svf.set_frequency(self.cutoff());

        self.feedback_cv = snapshot.cv[1].value * 2.0 - 1.0;
    }

    fn cutoff(&self) -> f32 {
        self.cutoff_ui + self.cutoff_cv
    }

    fn feedback(&self) -> f32 {
        (self.feedback_ui + self.feedback_cv).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    SetCutoff(f32),
    SetFeedback(f32),
    SetDensity(f32),
    SetChange(f32),
    SetBeats(u32),
    EnableLength(NoteLength),
    DisableLength(NoteLength),
}

impl TryFrom<Reaction> for Command {
    type Error = &'static str;

    fn try_from(other: Reaction) -> Result<Self, Self::Error> {
        match other {
            Reaction::SetValue(attribute, value) => {
                if attribute == CUTOFF_ATTRIBUTE {
                    Ok(Command::SetCutoff(value))
                } else if attribute == FEEDBACK_ATTRIBUTE {
                    Ok(Command::SetFeedback(value))
                } else if attribute == DENSITY_ATTRIBUTE {
                    Ok(Command::SetDensity(value))
                } else if attribute == CHANGE_ATTRIBUTE {
                    Ok(Command::SetChange(value))
                } else if attribute == BEATS_ATTRIBUTE {
                    Ok(Command::SetBeats(value as u32))
                } else {
                    Err("cannot convert this reaction to a command")
                }
            }
            Reaction::SelectValue(attribute, value) => {
                let length = NoteLength::from_attribute(attribute);
                if value == OFF_ON[1] {
                    Ok(Command::EnableLength(length))
                } else {
                    Ok(Command::DisableLength(length))
                }
            }
        }
    }
}

struct Turing {
    sample_rate: u32,
    triggers: [u32; 3],
    tones: IndexMap<usize, f32, 64>,
    phase: u32,
    pub lengths: Vec<NoteLength, { NoteLength::LEN }>,
    pub bpm: f32,
    pub density: u32,
    pub rate_of_change: f32,
    pub beats: u32,
}

impl Turing {
    const CELLS_IN_BEAT: u32 = 2 * 3;

    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            bpm: 360.0,
            triggers: [
                0b0000_0000_0000_0000_0000_0000_0000_0000,
                0b0000_0000_0000_0000_0000_0000_0000_0000,
                0b0000_0000_0000_0000_0000_0000_0000_0000,
            ],
            tones: IndexMap::new(),
            phase: 0,
            density: 16,
            rate_of_change: 4.0,
            lengths: Vec::new(),
            beats: 4,
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

        if self.phase >= cell_in_samples as u32 * self.enabled_cells() {
            self.randomize(randomizer);
            self.phase %= cell_in_samples as u32 * self.enabled_cells();
        }

        let new_tick = self.phase / cell_in_samples as u32;

        let triggered = if new_tick != old_tick {
            is_nth_tick_on(&self.triggers, new_tick as usize)
        } else {
            false
        };

        let frequency = if triggered {
            let voct = *self.tones.get(&(new_tick as usize)).unwrap();

            let oct = voct.trunc();
            let pentatonic = {
                let fract = voct.fract();
                if fract < 1.0 / 5.0 {
                    0.0
                } else if fract < 2.0 / 5.0 {
                    2.0 / 12.0
                } else if fract < 3.0 / 5.0 {
                    4.0 / 12.0
                } else if fract < 4.0 / 5.0 {
                    7.0 / 12.0
                } else {
                    9.0 / 12.0
                }
            };
            let quantized_voct = oct + pentatonic;
            A * 2.0_f32.powf(quantized_voct)
        } else {
            0.0
        };

        Config {
            triggered,
            frequency,
        }
    }

    fn randomize(&mut self, randomizer: &mut impl Rand) {
        use core::cmp::Ordering;

        let mut ticks_on = find_ticks_on(&self.triggers);
        let delta = ticks_on.len() as i32 - self.density as i32;

        let (add, remove) = match delta.cmp(&0) {
            Ordering::Less => (delta.abs().min(self.rate_of_change as i32), 0),
            Ordering::Equal => {
                let change = self.density.min(self.rate_of_change as u32) as i32;
                (change, change)
            }
            Ordering::Greater => (0, delta.min(self.rate_of_change as i32)),
        };

        for _ in 0..remove {
            let index = {
                let rand = randomizer.generate() as usize;
                ticks_on.swap_remove(rand % ticks_on.len())
            };
            set_nth_tick_off(&mut self.triggers, index);
            self.tones.remove(&index);
        }

        if !self.lengths.is_empty() {
            for _ in 0..add {
                let (length, tone) = {
                    let rand = randomizer.generate();
                    let length = self
                        .lengths
                        .get(rand as usize % self.lengths.len())
                        .unwrap();
                    const TONE_MIN: f32 = 1.0;
                    const TONE_MAX: f32 = 3.0;
                    let tone = TONE_MIN + (TONE_MAX - TONE_MIN) * (rand as f32 / u16::MAX as f32);
                    (length, tone)
                };
                let length_in_cells = length.in_cells();
                let position =
                    (randomizer.generate() as u32 % self.enabled_cells()) / length_in_cells;
                let index = position as usize * length_in_cells as usize;
                place_note(index, length_in_cells, &mut self.triggers);
                self.tones.insert(index, tone).unwrap();
            }
        }
    }

    fn enabled_cells(&self) -> u32 {
        self.beats * Self::CELLS_IN_BEAT
    }
}

#[derive(Clone, Copy)]
struct Config {
    triggered: bool,
    frequency: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NoteLength {
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

    fn from_attribute(name: &str) -> Self {
        match name {
            WHOLE_ATTRIBUTE => Whole,
            HALF_TRIPLET_ATTRIBUTE => HalfTriplet,
            HALF_ATTRIBUTE => Half,
            QUARTER_TRIPLET_ATTRIBUTE => QuarterTriplet,
            QUARTER_ATTRIBUTE => Quarter,
            EIGHT_TRIPLET_ATTRIBUTE => EightTriplet,
            EIGHT_ATTRIBUTE => Eight,
            _ => unreachable!(),
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
        Reaction::SetValue(FEEDBACK_ATTRIBUTE, 0.95) =>
        Ok(Command::SetFeedback(0.95))
    )]
    #[test_case(
        Reaction::SetValue(DENSITY_ATTRIBUTE, 5.0) =>
        Ok(Command::SetDensity(5.0))
    )]
    #[test_case(
        Reaction::SetValue(CHANGE_ATTRIBUTE, 5.0) =>
        Ok(Command::SetChange(5.0))
    )]
    #[test_case(
        Reaction::SetValue(BEATS_ATTRIBUTE, 5.0) =>
        Ok(Command::SetBeats(5))
    )]
    #[test_case(
        Reaction::SelectValue(WHOLE_ATTRIBUTE, OFF_ON[0]) =>
        Ok(Command::DisableLength(NoteLength::Whole))
    )]
    #[test_case(
        Reaction::SelectValue(WHOLE_ATTRIBUTE, OFF_ON[1]) =>
        Ok(Command::EnableLength(NoteLength::Whole))
    )]
    #[test_case(
        Reaction::SelectValue(HALF_ATTRIBUTE, OFF_ON[0]) =>
        Ok(Command::DisableLength(NoteLength::Half))
    )]
    #[test_case(
        Reaction::SelectValue(HALF_ATTRIBUTE, OFF_ON[1]) =>
        Ok(Command::EnableLength(NoteLength::Half))
    )]
    #[test_case(
        Reaction::SelectValue(QUARTER_ATTRIBUTE, OFF_ON[0]) =>
        Ok(Command::DisableLength(NoteLength::Quarter))
    )]
    #[test_case(
        Reaction::SelectValue(QUARTER_ATTRIBUTE, OFF_ON[1]) =>
        Ok(Command::EnableLength(NoteLength::Quarter))
    )]
    #[test_case(
        Reaction::SelectValue(EIGHT_ATTRIBUTE, OFF_ON[0]) =>
        Ok(Command::DisableLength(NoteLength::Eight))
    )]
    #[test_case(
        Reaction::SelectValue(EIGHT_ATTRIBUTE, OFF_ON[1]) =>
        Ok(Command::EnableLength(NoteLength::Eight))
    )]
    #[test_case(
        Reaction::SelectValue(HALF_TRIPLET_ATTRIBUTE, OFF_ON[0]) =>
        Ok(Command::DisableLength(NoteLength::HalfTriplet))
    )]
    #[test_case(
        Reaction::SelectValue(HALF_TRIPLET_ATTRIBUTE, OFF_ON[1]) =>
        Ok(Command::EnableLength(NoteLength::HalfTriplet))
    )]
    #[test_case(
        Reaction::SelectValue(QUARTER_TRIPLET_ATTRIBUTE, OFF_ON[0]) =>
        Ok(Command::DisableLength(NoteLength::QuarterTriplet))
    )]
    #[test_case(
        Reaction::SelectValue(QUARTER_TRIPLET_ATTRIBUTE, OFF_ON[1]) =>
        Ok(Command::EnableLength(NoteLength::QuarterTriplet))
    )]
    #[test_case(
        Reaction::SelectValue(EIGHT_TRIPLET_ATTRIBUTE, OFF_ON[0]) =>
        Ok(Command::DisableLength(NoteLength::EightTriplet))
    )]
    #[test_case(
        Reaction::SelectValue(EIGHT_TRIPLET_ATTRIBUTE, OFF_ON[1]) =>
        Ok(Command::EnableLength(NoteLength::EightTriplet))
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
}

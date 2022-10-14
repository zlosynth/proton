#![no_std]

use core::convert::TryFrom;
use core::fmt;

use proton_control::input_snapshot::InputSnapshot;
use proton_instruments_interface::{
    Instrument as InstrumentTrait, MemoryManager, Rand as ProtonRandomizer,
};
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

use kaseta_control::{self as control, Cache, ControlAction};
use kaseta_dsp::processor::Processor;
use kaseta_dsp::random::Random as KasetaRandomizer;

const NAME: &str = "Kaseta";
const PRE_AMP_ATTRIBUTE: &str = "pre-amp";
const DRIVE_ATTRIBUTE: &str = "drive";
const BIAS_ATTRIBUTE: &str = "bias";
const WOW_FREQUENCY_ATTRIBUTE: &str = "wow frequency";
const WOW_DEPTH_ATTRIBUTE: &str = "wow depth";
const WOW_FILTER_ATTRIBUTE: &str = "wow filter";
const DELAY_ATTRIBUTE: &str = "delay";
const DELAY_1_POSITION_ATTRIBUTE: &str = "position 1";
const DELAY_2_POSITION_ATTRIBUTE: &str = "position 2";
const DELAY_3_POSITION_ATTRIBUTE: &str = "position 3";
const DELAY_4_POSITION_ATTRIBUTE: &str = "position 4";
const DELAY_1_VOLUME_ATTRIBUTE: &str = "volume 1";
const DELAY_2_VOLUME_ATTRIBUTE: &str = "volume 2";
const DELAY_3_VOLUME_ATTRIBUTE: &str = "volume 3";
const DELAY_4_VOLUME_ATTRIBUTE: &str = "volume 4";
const DELAY_1_FEEDBACK_ATTRIBUTE: &str = "feedback 1";
const DELAY_2_FEEDBACK_ATTRIBUTE: &str = "feedback 2";
const DELAY_3_FEEDBACK_ATTRIBUTE: &str = "feedback 3";
const DELAY_4_FEEDBACK_ATTRIBUTE: &str = "feedback 4";

pub struct Instrument {
    processor: Processor,
    cache: Cache,
}

fn writter(destination: &mut dyn fmt::Write, value: f32) {
    let value = (value * 100.0) as u32;
    write!(destination, "{}%", value).unwrap();
}

impl InstrumentTrait for Instrument {
    type Command = Command;

    fn new(sample_rate: u32, memory_manager: &mut MemoryManager) -> Self {
        let mut processor = Processor::new(sample_rate as f32, memory_manager);
        let cache = Cache::default();
        let attributes = control::cook_dsp_reaction_from_cache(&cache).into();
        processor.set_attributes(attributes);
        Self { processor, cache }
    }

    fn state(&self) -> State {
        let attributes = control::cook_dsp_reaction_from_cache(&self.cache);
        State::new(NAME)
            .with_attributes(&[
                Attribute::new(PRE_AMP_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.pre_amp).with_writter(writter)),
                Attribute::new(DRIVE_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.drive).with_writter(writter)),
                Attribute::new(BIAS_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.bias).with_writter(writter)),
                Attribute::new(""),
                Attribute::new(WOW_FREQUENCY_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.wow_frequency).with_writter(writter)),
                Attribute::new(WOW_DEPTH_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.wow_depth).with_writter(writter)),
                Attribute::new(WOW_FILTER_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.wow_filter).with_writter(writter)),
                Attribute::new(DELAY_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.delay_length).with_writter(writter)),
                Attribute::new(DELAY_1_POSITION_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_position[0]).with_writter(writter),
                ),
                Attribute::new(DELAY_2_POSITION_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_position[1]).with_writter(writter),
                ),
                Attribute::new(DELAY_3_POSITION_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_position[2]).with_writter(writter),
                ),
                Attribute::new(DELAY_4_POSITION_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_position[3]).with_writter(writter),
                ),
                Attribute::new(DELAY_1_VOLUME_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_volume[0]).with_writter(writter),
                ),
                Attribute::new(DELAY_2_VOLUME_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_volume[1]).with_writter(writter),
                ),
                Attribute::new(DELAY_3_VOLUME_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_volume[2]).with_writter(writter),
                ),
                Attribute::new(DELAY_4_VOLUME_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_volume[3]).with_writter(writter),
                ),
                Attribute::new(DELAY_1_FEEDBACK_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_feedback_amount[0]).with_writter(writter),
                ),
                Attribute::new(DELAY_2_FEEDBACK_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_feedback_amount[1]).with_writter(writter),
                ),
                Attribute::new(DELAY_3_FEEDBACK_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_feedback_amount[2]).with_writter(writter),
                ),
                Attribute::new(DELAY_4_FEEDBACK_ATTRIBUTE).with_value_f32(
                    ValueF32::new(attributes.delay_head_feedback_amount[3]).with_writter(writter),
                ),
            ])
            .unwrap()
    }

    fn process(&mut self, buffer: &mut [(f32, f32)], randomizer: &mut impl ProtonRandomizer) {
        let mut randomizer = LocalRandomizer::from(randomizer);
        for chunk in buffer.chunks_exact_mut(32) {
            let mut buffer = [0.0; 32];
            for (i, x) in chunk.iter_mut().enumerate() {
                buffer[i] = x.0;
            }
            self.processor.process(&mut buffer, &mut randomizer);
            for (i, x) in buffer.iter_mut().enumerate() {
                chunk[i] = (*x, 0.0);
            }
        }
    }

    fn execute(&mut self, command: Command) {
        let dsp_reaction = control::reduce_control_action(command.into(), &mut self.cache);
        self.processor.set_attributes(dsp_reaction.into());
    }

    fn update_control(&mut self, snapshot: InputSnapshot) {
        self.cache.hysteresis.drive_cv = snapshot.cv[0].value;
        self.cache.hysteresis.bias_cv = snapshot.pot.value;
        let attributes = control::cook_dsp_reaction_from_cache(&self.cache).into();
        self.processor.set_attributes(attributes);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    SetPreAmp(f32),
    SetDrive(f32),
    SetBias(f32),
    SetWowFrequency(f32),
    SetWowDepth(f32),
    SetWowFilter(f32),
    SetDelay(f32),
    SetDelay1Position(f32),
    SetDelay2Position(f32),
    SetDelay3Position(f32),
    SetDelay4Position(f32),
    SetDelay1Volume(f32),
    SetDelay2Volume(f32),
    SetDelay3Volume(f32),
    SetDelay4Volume(f32),
    SetDelay1Feedback(f32),
    SetDelay2Feedback(f32),
    SetDelay3Feedback(f32),
    SetDelay4Feedback(f32),
}

impl TryFrom<Reaction> for Command {
    type Error = ();

    fn try_from(other: Reaction) -> Result<Self, Self::Error> {
        match other {
            Reaction::SetValue(PRE_AMP_ATTRIBUTE, value) => Ok(Command::SetPreAmp(value)),
            Reaction::SetValue(DRIVE_ATTRIBUTE, value) => Ok(Command::SetDrive(value)),
            Reaction::SetValue(BIAS_ATTRIBUTE, value) => Ok(Command::SetBias(value)),
            Reaction::SetValue(WOW_FREQUENCY_ATTRIBUTE, value) => {
                Ok(Command::SetWowFrequency(value))
            }
            Reaction::SetValue(WOW_DEPTH_ATTRIBUTE, value) => Ok(Command::SetWowDepth(value)),
            Reaction::SetValue(WOW_FILTER_ATTRIBUTE, value) => Ok(Command::SetWowFilter(value)),
            Reaction::SetValue(DELAY_ATTRIBUTE, value) => Ok(Command::SetDelay(value)),
            Reaction::SetValue(DELAY_1_POSITION_ATTRIBUTE, value) => {
                Ok(Command::SetDelay1Position(value))
            }
            Reaction::SetValue(DELAY_2_POSITION_ATTRIBUTE, value) => {
                Ok(Command::SetDelay2Position(value))
            }
            Reaction::SetValue(DELAY_3_POSITION_ATTRIBUTE, value) => {
                Ok(Command::SetDelay3Position(value))
            }
            Reaction::SetValue(DELAY_4_POSITION_ATTRIBUTE, value) => {
                Ok(Command::SetDelay4Position(value))
            }
            Reaction::SetValue(DELAY_1_VOLUME_ATTRIBUTE, value) => {
                Ok(Command::SetDelay1Volume(value))
            }
            Reaction::SetValue(DELAY_2_VOLUME_ATTRIBUTE, value) => {
                Ok(Command::SetDelay2Volume(value))
            }
            Reaction::SetValue(DELAY_3_VOLUME_ATTRIBUTE, value) => {
                Ok(Command::SetDelay3Volume(value))
            }
            Reaction::SetValue(DELAY_4_VOLUME_ATTRIBUTE, value) => {
                Ok(Command::SetDelay4Volume(value))
            }
            Reaction::SetValue(DELAY_1_FEEDBACK_ATTRIBUTE, value) => {
                Ok(Command::SetDelay1Feedback(value))
            }
            Reaction::SetValue(DELAY_2_FEEDBACK_ATTRIBUTE, value) => {
                Ok(Command::SetDelay2Feedback(value))
            }
            Reaction::SetValue(DELAY_3_FEEDBACK_ATTRIBUTE, value) => {
                Ok(Command::SetDelay3Feedback(value))
            }
            Reaction::SetValue(DELAY_4_FEEDBACK_ATTRIBUTE, value) => {
                Ok(Command::SetDelay4Feedback(value))
            }
            _ => Err(()),
        }
    }
}

impl From<Command> for ControlAction {
    fn from(other: Command) -> ControlAction {
        match other {
            Command::SetPreAmp(value) => ControlAction::SetPreAmpPot(value),
            Command::SetDrive(value) => ControlAction::SetDrivePot(value),
            Command::SetBias(value) => ControlAction::SetBiasPot(value),
            Command::SetWowFrequency(value) => ControlAction::SetWowFrequencyPot(value),
            Command::SetWowDepth(value) => ControlAction::SetWowDepthPot(value),
            Command::SetWowFilter(value) => ControlAction::SetWowFilterPot(value),
            Command::SetDelay(value) => ControlAction::SetDelayLengthPot(value),
            Command::SetDelay1Position(value) => ControlAction::SetDelayHeadPositionPot(0, value),
            Command::SetDelay2Position(value) => ControlAction::SetDelayHeadPositionPot(1, value),
            Command::SetDelay3Position(value) => ControlAction::SetDelayHeadPositionPot(2, value),
            Command::SetDelay4Position(value) => ControlAction::SetDelayHeadPositionPot(3, value),
            Command::SetDelay1Volume(value) => ControlAction::SetDelayHeadVolume(0, value),
            Command::SetDelay2Volume(value) => ControlAction::SetDelayHeadVolume(1, value),
            Command::SetDelay3Volume(value) => ControlAction::SetDelayHeadVolume(2, value),
            Command::SetDelay4Volume(value) => ControlAction::SetDelayHeadVolume(3, value),
            Command::SetDelay1Feedback(value) => {
                ControlAction::SetDelayHeadFeedbackAmount(0, value)
            }
            Command::SetDelay2Feedback(value) => {
                ControlAction::SetDelayHeadFeedbackAmount(1, value)
            }
            Command::SetDelay3Feedback(value) => {
                ControlAction::SetDelayHeadFeedbackAmount(2, value)
            }
            Command::SetDelay4Feedback(value) => {
                ControlAction::SetDelayHeadFeedbackAmount(3, value)
            }
        }
    }
}

struct LocalRandomizer<'a, R> {
    rand: &'a mut R,
}

impl<'a, R> From<&'a mut R> for LocalRandomizer<'a, R> {
    fn from(rand: &'a mut R) -> Self {
        Self { rand }
    }
}

impl<'a, R: ProtonRandomizer> KasetaRandomizer for LocalRandomizer<'a, R> {
    fn normal(&mut self) -> f32 {
        self.rand.generate() as f32 / (2 << 14) as f32 - 1.0
    }
}

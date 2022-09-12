#![no_std]

use core::convert::TryFrom;
use core::fmt;

use proton_control::input_snapshot::InputSnapshot;
use proton_instruments_interface::{Instrument as InstrumentTrait, MemoryManager};
use proton_ui::reaction::Reaction;
use proton_ui::state::*;

use kaseta_control::{self as control, Cache, ControlAction};
use kaseta_dsp::processor::Processor;

const NAME: &str = "Kaseta";
const PRE_AMP_ATTRIBUTE: &str = "pre-amp";
const DRIVE_ATTRIBUTE: &str = "drive";
const SATURATION_ATTRIBUTE: &str = "saturation";
const BIAS_ATTRIBUTE: &str = "bias";
const WOW_FREQUENCY_ATTRIBUTE: &str = "wow frequency";
const WOW_DEPTH_ATTRIBUTE: &str = "wow depth";

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
                Attribute::new(SATURATION_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.saturation).with_writter(writter)),
                Attribute::new(BIAS_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.bias).with_writter(writter)),
                Attribute::new(WOW_FREQUENCY_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.wow_frequency).with_writter(writter)),
                Attribute::new(WOW_DEPTH_ATTRIBUTE)
                    .with_value_f32(ValueF32::new(attributes.wow_depth).with_writter(writter)),
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

    fn execute(&mut self, command: Command) {
        let dsp_reaction = control::reduce_control_action(command.into(), &mut self.cache);
        self.processor.set_attributes(dsp_reaction.into());
    }

    fn update_control(&mut self, snapshot: InputSnapshot) {
        self.cache.drive_cv = snapshot.cv[0].value;
        self.cache.saturation_cv = snapshot.cv[1].value;
        self.cache.bias_cv = snapshot.pot.value;
        let attributes = control::cook_dsp_reaction_from_cache(&self.cache).into();
        self.processor.set_attributes(attributes);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    SetPreAmp(f32),
    SetDrive(f32),
    SetSaturation(f32),
    SetBias(f32),
    SetWowFrequency(f32),
    SetWowDepth(f32),
}

impl TryFrom<Reaction> for Command {
    type Error = ();

    fn try_from(other: Reaction) -> Result<Self, Self::Error> {
        match other {
            Reaction::SetValue(PRE_AMP_ATTRIBUTE, value) => Ok(Command::SetPreAmp(value)),
            Reaction::SetValue(DRIVE_ATTRIBUTE, value) => Ok(Command::SetDrive(value)),
            Reaction::SetValue(SATURATION_ATTRIBUTE, value) => Ok(Command::SetSaturation(value)),
            Reaction::SetValue(BIAS_ATTRIBUTE, value) => Ok(Command::SetBias(value)),
            Reaction::SetValue(WOW_FREQUENCY_ATTRIBUTE, value) => Ok(Command::SetWowFrequency(value)),
            Reaction::SetValue(WOW_DEPTH_ATTRIBUTE, value) => Ok(Command::SetWowDepth(value)),
            _ => Err(()),
        }
    }
}

impl From<Command> for ControlAction {
    fn from(other: Command) -> ControlAction {
        match other {
            Command::SetPreAmp(value) => ControlAction::SetPreAmpPot(value),
            Command::SetDrive(value) => ControlAction::SetDrivePot(value),
            Command::SetSaturation(value) => ControlAction::SetSaturationPot(value),
            Command::SetBias(value) => ControlAction::SetBiasPot(value),
            Command::SetWowFrequency(value) => ControlAction::SetWowFrequencyPot(value),
            Command::SetWowDepth(value) => ControlAction::SetWowDepthPot(value),
        }
    }
}

use core::convert::TryFrom;

use proton_ui::reaction::Reaction;

use super::attributes;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    SetPreGain(f32),
    SetDrive(f32),
    SetSaturation(f32),
    SetWidth(f32),
    Flush,
}

use Command::*;

impl TryFrom<Reaction> for Command {
    type Error = &'static str;

    fn try_from(other: Reaction) -> Result<Self, Self::Error> {
        match other {
            Reaction::SetValue(name, value) => match name {
                attributes::PRE_GAIN => Ok(SetPreGain(value)),
                attributes::DRIVE => Ok(SetDrive(value)),
                attributes::SATURATION => Ok(SetSaturation(value)),
                attributes::WIDTH => Ok(SetWidth(value)),
                _ => Err("invalid attribute name"),
            },
            Reaction::SelectValue(name, _) => match name {
                attributes::FLUSH => Ok(Flush),
                _ => Err("invalid attribute name"),
            },
        }
    }
}

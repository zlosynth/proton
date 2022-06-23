use core::convert::TryFrom;

use proton_ui::reaction::Reaction;

use super::attributes;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    SetPreGain(f32),
}

use Command::*;

impl TryFrom<Reaction> for Command {
    type Error = &'static str;

    fn try_from(other: Reaction) -> Result<Self, Self::Error> {
        match other {
            Reaction::SetValue(name, value) => match name {
                attributes::PRE_GAIN => Ok(SetPreGain(value)),
                _ => Err("invalid attribute name"),
            },
            Reaction::SelectValue(_, _) => Err("invalid attribute type"),
        }
    }
}

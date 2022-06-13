use core::convert::TryFrom;

use proton_ui::reaction::Reaction;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    Noop,
}

impl TryFrom<Reaction> for Command {
    type Error = &'static str;

    fn try_from(_other: Reaction) -> Result<Self, Self::Error> {
        Ok(Command::Noop)
    }
}

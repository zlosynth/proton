use proton_ui::state::*;

use crate::instrument::Instrument;

const NAME: &str = "Tape";

impl Instrument {
    pub fn initial_state() -> State {
        State::new(NAME)
    }
}

#![no_std]

use proton_primitives::white_noise;
use proton_ui::state::*;

const NAME: &str = "Karplus Strong";
const FREQUENCY_ATTRIBUTE: &str = "frequency";

pub struct Instrument;

// TODO: Just filtered noise

impl Instrument {
    pub fn initial_state() -> State {
        State::new(NAME)
            .with_attributes(&[
                Attribute::new(FREQUENCY_ATTRIBUTE).with_value_f32(ValueF32::new(0.3))
            ])
            .unwrap()
    }

    pub fn populate(&mut self, buffer: &mut [f32]) {
        for x in buffer.iter_mut() {
            *x = white_noise::pop();
        }
    }
}

// type Reaction = (); // TODO Import from UI

// pub struct Instrument;

// pub enum Command{
//     // TODO
// }

// impl From<Reaction> for Command {
//     // TODO: Match the string, convert to something known
// }

// // TODO: generate initial state

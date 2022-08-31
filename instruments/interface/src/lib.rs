#![no_std]

use proton_control::input_snapshot::InputSnapshot;
use proton_ui::reaction::Reaction;
use proton_ui::state::State;

pub trait Instrument {
    type Command: TryFrom<Reaction>;
    fn new(sample_rate: u32) -> Self;
    fn state(&self) -> State;
    fn process(&mut self, buffer: &mut [(f32, f32)]);
    fn execute(&mut self, command: Self::Command);
    fn update_control(&mut self, snapshot: InputSnapshot);
}

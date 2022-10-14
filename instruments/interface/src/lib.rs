#![no_std]

use proton_control::input_snapshot::InputSnapshot;
use proton_ui::reaction::Reaction;
use proton_ui::state::State;

pub trait Instrument {
    type Command: TryFrom<Reaction>;
    fn new(sample_rate: u32, memory_manager: &mut MemoryManager) -> Self;
    fn state(&self) -> State;
    fn process(&mut self, buffer: &mut [(f32, f32)], randomizer: &mut impl Rand);
    fn execute(&mut self, command: Self::Command);
    fn update_control(&mut self, snapshot: InputSnapshot);
}

pub use sirena::memory_manager::MemoryManager;

pub trait Rand {
    fn generate(&mut self) -> u16;
}

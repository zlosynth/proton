use super::command::Command;
use super::Instrument;

impl Instrument {
    pub fn execute(&mut self, command: Command) {
        match command {
            Command::SetPreGain(value) => self.pre_gain.set(value),
        }
    }
}

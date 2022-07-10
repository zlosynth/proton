use super::command::Command;
use super::Instrument;

impl Instrument {
    pub fn execute(&mut self, command: Command) {
        match command {
            Command::SetPreGain(value) => self.pre_gain.set(value),
            Command::SetDrive(value) => self.drive.set(value),
            Command::SetSaturation(value) => self.saturation.set(value),
            Command::SetWidth(value) => self.width.set(value),
        }
    }
}

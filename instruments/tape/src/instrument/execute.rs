use super::command::Command;
use super::Instrument;

impl Instrument {
    pub fn execute(&mut self, command: Command) {
        match command {
            Command::SetPreGain(value) => self.pre_gain.set(value),
            Command::SetDrive(value) => self.hysteresis.set_drive(value),
            Command::SetSaturation(value) => self.hysteresis.set_saturation(value),
            Command::SetWidth(value) => self.hysteresis.set_width(value),
        }
    }
}

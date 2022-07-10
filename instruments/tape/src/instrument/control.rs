use proton_control::input_snapshot::InputSnapshot;

use super::Instrument;

impl Instrument {
    pub fn update_control(&mut self, snapshot: InputSnapshot) {
        self.drive.set(snapshot.cv[0].value);
        self.saturation.set(snapshot.cv[1].value);
        self.width.set(snapshot.cv[2].value);
    }
}

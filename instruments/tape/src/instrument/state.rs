use core::fmt;

use proton_ui::state::*;

use super::attributes;
use super::Instrument;

const NAME: &str = "Tape";

impl Instrument {
    pub fn state(&self) -> State {
        State::new(NAME)
            .with_attributes(&[Attribute::new(attributes::POST_GAIN).with_value_f32(
                ValueF32::new(self.post_gain)
                    .with_min(0.0)
                    .with_max(1.0)
                    .with_step(0.05)
                    .with_writter(percentage_writter),
            )])
            .unwrap()
    }
}

fn percentage_writter(destination: &mut dyn fmt::Write, value: f32) {
    write!(destination, "{:.0}%", value * 100.0).unwrap();
}

use core::fmt;

use proton_ui::state::*;

use super::attributes;
use super::Instrument;

const NAME: &str = "Tape";

impl Instrument {
    pub fn state(&self) -> State {
        State::new(NAME)
            .with_attributes(&[
                Attribute::new(attributes::PRE_GAIN).with_value_f32(
                    ValueF32::new(self.pre_gain.value())
                        .with_min(0.0)
                        .with_max(5.0)
                        .with_step(0.05)
                        .with_writter(percentage_writter),
                ),
                Attribute::new(attributes::DRIVE).with_value_f32(
                    ValueF32::new(self.hysteresis.drive())
                        .with_min(-5.0)
                        .with_max(5.0)
                        .with_step(0.1)
                        .with_writter(percentage_writter),
                ),
                Attribute::new(attributes::SATURATION).with_value_f32(
                    ValueF32::new(self.hysteresis.saturation())
                        .with_min(-10.0)
                        .with_max(2.0)
                        .with_step(0.05)
                        .with_writter(percentage_writter),
                ),
                Attribute::new(attributes::WIDTH).with_value_f32(
                    ValueF32::new(self.hysteresis.width())
                        .with_min(-10.0)
                        .with_max(1.0)
                        .with_step(0.02)
                        .with_writter(percentage_writter),
                ),
                Attribute::new(attributes::FLUSH)
                    .with_value_select(ValueSelect::new(&["Trigger", "Boom"]).unwrap()),
            ])
            .unwrap()
    }
}

fn percentage_writter(destination: &mut dyn fmt::Write, value: f32) {
    write!(destination, "{:.0}%", value * 100.0).unwrap();
}

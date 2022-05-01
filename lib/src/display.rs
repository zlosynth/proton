use alloc::vec;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_graphics_core::draw_target::DrawTarget;

use crate::store::{Module, Store};

const PADDING_LEFT: i32 = 5;
const FONT_HEIGHT: i32 = 12;

pub struct Display<D> {
    pub display: D,
}

impl<D> Display<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    pub fn new(display: D) -> Self {
        Self { display }
    }

    pub fn update(&mut self) {
        let store = Store {
            modules: vec![
                Module { name: "ENV" },
                Module { name: "MIX" },
                Module { name: "OSC" },
                Module { name: "CV" },
                Module { name: "AUD" },
            ],
        };

        for (i, module) in store.modules.iter().enumerate() {
            draw_module(module, i, &mut self.display);
        }
    }
}

fn draw_module<D: DrawTarget<Color = BinaryColor>>(module: &Module, index: usize, display: &mut D) {
    Text::new(
        module.name,
        Point::new(PADDING_LEFT, FONT_HEIGHT * (index + 1) as i32 - 1),
        MonoTextStyle::new(&FONT_6X12, BinaryColor::On),
    )
    .draw(display)
    .ok()
    .unwrap();
}

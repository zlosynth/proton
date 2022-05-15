#![no_std]
#![no_main]

use proton_eurorack as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {
    #[test]
    fn display_works() {
        use embedded_graphics::{
            mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
            pixelcolor::BinaryColor,
            prelude::*,
            text::Text,
        };
        use proton_eurorack::system::System;

        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = daisy::pac::Peripherals::take().unwrap();

        let system = System::init(cp, dp);
        let mut display = system.display;

        let style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build();
        let position = Point::new(15, 45);

        Text::new("Test 3", position, style)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
        cortex_m::asm::delay(480_000_000);

        Text::new("Test 2", position, style)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
        cortex_m::asm::delay(480_000_000);

        Text::new("Test 1", position, style)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
        cortex_m::asm::delay(480_000_000);
    }
}

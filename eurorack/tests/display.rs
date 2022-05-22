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

        let mut system = System::init(cp, dp);
        let mut display = system.display;

        let style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build();
        let position = Point::new(15, 45);
        Text::new("TEST", position, style)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();

        defmt::info!("ACTION REQUIRED: Click Alpha if display displays");
        while !system.alpha_button.clicked() {
            system.alpha_button.sample();
            cortex_m::asm::nop();
        }
    }
}

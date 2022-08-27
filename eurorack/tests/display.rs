#![no_std]
#![no_main]

use proton_eurorack as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {
    use super::wait_for_click;
    use proton_eurorack::system::System;

    #[init]
    fn init() -> System {
        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = daisy::pac::Peripherals::take().unwrap();

        System::init(cp, dp)
    }

    #[test]
    fn display_works(system: &mut System) {
        use embedded_graphics::{
            mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
            pixelcolor::BinaryColor,
            prelude::*,
            text::Text,
        };

        let display = &mut system.display;

        let style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build();
        let position = Point::new(15, 45);

        Text::new("TEST 1", position, style).draw(display).unwrap();
        display.flush().unwrap();
        defmt::info!("ACTION REQUIRED: Click encoder if display displays TEST 1");
        wait_for_click(&mut system.button);

        Text::new("TEST 2", position, style).draw(display).unwrap();
        display.flush().unwrap();
        defmt::info!("ACTION REQUIRED: Click encoder if display displays TEST 2");
        wait_for_click(&mut system.button);
    }
}

fn wait_for_click<const N: usize, P>(button: &mut proton_peripherals::button::Button<N, P>)
where
    P: daisy::embedded_hal::digital::v2::InputPin,
{
    loop {
        button.sample();
        if button.clicked() {
            return;
        }
        cortex_m::asm::delay(480_000_000 / 1000);
    }
}

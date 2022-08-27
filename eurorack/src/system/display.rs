use daisy::hal;
use fugit::RateExtU32;
use hal::gpio;
use hal::hal::blocking::delay::DelayMs;
use hal::pac;
use hal::prelude::_stm32h7xx_hal_spi_SpiExt;
use hal::spi::{self, Spi};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, Ssd1306};

pub type Display = Ssd1306<
    SPIInterface<
        Spi<pac::SPI1, spi::Enabled>,
        gpio::gpiob::PB4<gpio::Output<gpio::PushPull>>,
        gpio::gpiog::PG10<gpio::Output<gpio::PushPull>>,
    >,
    DisplaySize128x64,
    BufferedGraphicsMode<DisplaySize128x64>,
>;

pub fn init(
    mut pins: DisplayPins,
    delay: &mut impl DelayMs<u8>,
    dp_spi1: pac::SPI1,
    ccdr_spi1: hal::rcc::rec::Spi1,
    clocks: &hal::rcc::CoreClocks,
) -> Display {
    let spi = dp_spi1.spi(
        (pins.SCK, spi::NoMiso, pins.MOSI),
        spi::MODE_0,
        3.MHz(),
        ccdr_spi1,
        clocks,
    );

    let interface = display_interface_spi::SPIInterface::new(spi, pins.DC, pins.CS);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.reset(&mut pins.RST, delay).unwrap();
    display.init().unwrap();

    display
}

#[allow(non_snake_case)]
pub struct DisplayPins {
    pub CS: gpio::gpiog::PG10<gpio::Output<gpio::PushPull>>, // SEED PIN 8
    pub SCK: gpio::gpiog::PG11<gpio::Alternate<5>>,          // SEED PIN 9
    pub DC: gpio::gpiob::PB4<gpio::Output<gpio::PushPull>>,  // SEED PIN 10
    pub MOSI: gpio::gpiob::PB5<gpio::Alternate<5>>,          // SEED PIN 11
    pub RST: gpio::gpiob::PB15<gpio::Output<gpio::PushPull>>, // SEED PIN 37
}

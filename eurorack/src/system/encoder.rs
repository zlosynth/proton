use daisy::hal;
use proton_peripherals::button::Button;
use proton_peripherals::detent_rotary::DetentRotary;

pub type AlphaButton = Button<10, hal::gpio::gpioc::PC1<hal::gpio::Input>>;
pub type BetaButton = Button<10, hal::gpio::gpiob::PB9<hal::gpio::Input>>;

pub type AlphaRotary =
    DetentRotary<hal::gpio::gpioc::PC4<hal::gpio::Input>, hal::gpio::gpioa::PA1<hal::gpio::Input>>;
pub type BetaRotary =
    DetentRotary<hal::gpio::gpiob::PB7<hal::gpio::Input>, hal::gpio::gpiob::PB6<hal::gpio::Input>>;

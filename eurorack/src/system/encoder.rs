use daisy::hal;
use proton_peripherals::button::Button;
use proton_peripherals::rotary::Rotary;

pub type AlphaButtonPin = hal::gpio::gpioc::PC1<hal::gpio::Input>;
pub type AlphaButton = Button<10, AlphaButtonPin>;
pub type BetaButtonPin = hal::gpio::gpiob::PB9<hal::gpio::Input>;
pub type BetaButton = Button<10, BetaButtonPin>;

pub type AlphaRotaryPinA = hal::gpio::gpioc::PC4<hal::gpio::Input>;
pub type AlphaRotaryPinB = hal::gpio::gpioa::PA1<hal::gpio::Input>;
pub type AlphaRotary = Rotary<AlphaRotaryPinA, AlphaRotaryPinB>;

pub type BetaRotaryPinA = hal::gpio::gpiob::PB7<hal::gpio::Input>;
pub type BetaRotaryPinB = hal::gpio::gpiob::PB6<hal::gpio::Input>;
pub type BetaRotary = Rotary<BetaRotaryPinA, BetaRotaryPinB>;

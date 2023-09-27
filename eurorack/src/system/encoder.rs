use proton_peripherals::button::Button;
use proton_peripherals::rotary::Rotary;
use stm32h7xx_hal as hal;

pub type EncoderButtonPin = hal::gpio::gpiod::PD11<hal::gpio::Input>;
pub type EncoderButton = Button<10, EncoderButtonPin>;

pub type EncoderRotaryPinA = hal::gpio::gpioc::PC4<hal::gpio::Input>;
pub type EncoderRotaryPinB = hal::gpio::gpioa::PA1<hal::gpio::Input>;
pub type EncoderRotary = Rotary<EncoderRotaryPinA, EncoderRotaryPinB>;

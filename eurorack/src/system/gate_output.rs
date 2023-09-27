use proton_peripherals::gate_output::GateOutput as PeripheralGateOutput;
use stm32h7xx_hal as hal;

pub type GateOutput1 = PeripheralGateOutput<hal::gpio::gpiog::PG9<hal::gpio::Output>>;
pub type GateOutput2 = PeripheralGateOutput<hal::gpio::gpioa::PA2<hal::gpio::Output>>;
pub type GateOutput3 = PeripheralGateOutput<hal::gpio::gpiob::PB14<hal::gpio::Output>>;

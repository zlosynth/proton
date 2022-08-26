use daisy::hal;
use proton_peripherals::gate_output::GateOutput as PeripheralGateOutput;

pub type GateOutput1 = PeripheralGateOutput<hal::gpio::gpiog::PG9<hal::gpio::Output>>;
pub type GateOutput2 = PeripheralGateOutput<hal::gpio::gpioa::PA2<hal::gpio::Output>>;

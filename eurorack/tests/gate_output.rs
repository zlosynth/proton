#![no_std]
#![no_main]

use proton_eurorack as _; // memory layout + panic handler
use proton_peripherals::button::Button;

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
    fn gate_output_up_and_down(system: &mut System) {
        defmt::info!("ACTION REQUIRED: Confirm that gate 1 is up and click encoder to continue");
        system.gate_1.set();
        defmt::info!("ACTION REQUIRED: Confirm that gate 1 is down and click encoder to continue");
        system.gate_1.reset();

        defmt::info!("ACTION REQUIRED: Confirm that gate 2 is up and click encoder to continue");
        system.gate_2.set();
        defmt::info!("ACTION REQUIRED: Confirm that gate 2 is down and click encoder to continue");
        system.gate_2.reset();

        defmt::info!("ACTION REQUIRED: Click encoder to continue");
        wait_for_click(&mut system.button);
    }
}

fn wait_for_click<const N: usize, P>(button: &mut Button<N, P>)
where
    P: daisy::embedded_hal::digital::v2::InputPin,
{
    while !button.clicked() {
        button.sample();
        cortex_m::asm::nop();
    }
}

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
    fn gate_output_up_and_down(system: &mut System) {
        macro_rules! test_gate {
            ($i:expr, $gate:ident) => {
                defmt::info!(
                    "ACTION REQUIRED: Confirm that gate {} is up and click encoder to continue",
                    $i
                );
                system.$gate.set();
                wait_for_click(&mut system.button);

                defmt::info!(
                    "ACTION REQUIRED: Confirm that gate {} is down and click encoder to continue",
                    $i
                );
                system.$gate.reset();
                wait_for_click(&mut system.button);
            };
        }

        test_gate!(1, gate_1);
        test_gate!(2, gate_2);
        test_gate!(3, gate_3);

        defmt::info!("ACTION REQUIRED: Click encoder to continue");
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

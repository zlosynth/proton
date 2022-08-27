#![no_std]
#![no_main]

use proton_eurorack as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {
    use super::wait_for_click;
    use proton_eurorack::system::System;
    use proton_peripherals::cv_output::CvOutput;

    #[init]
    fn init() -> System {
        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = daisy::pac::Peripherals::take().unwrap();

        System::init(cp, dp)
    }

    #[test]
    fn cv_output_span(system: &mut System) {
        macro_rules! test_cv_output {
            ($i:expr, $cv_output:ident) => {
                defmt::info!(
                    "ACTION REQUIRED: Confirm that CV output {} is on its minimum and click encoder to continue",
                    $i
                );
                system.$cv_output.set_value(0.0);
                wait_for_click(&mut system.button);

                defmt::info!(
                    "ACTION REQUIRED: Confirm that CV output {} is on its center and click encoder to continue",
                    $i
                );
                system.$cv_output.set_value(0.5);
                wait_for_click(&mut system.button);

                defmt::info!(
                    "ACTION REQUIRED: Confirm that CV output {} is on its maximum and click encoder to continue",
                    $i
                );
                system.$cv_output.set_value(1.0);
                wait_for_click(&mut system.button);
            };
        }

        test_cv_output!(1, cv_output_1);
        test_cv_output!(2, cv_output_2);

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

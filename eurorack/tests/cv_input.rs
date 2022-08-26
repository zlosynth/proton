#![no_std]
#![no_main]

use proton_eurorack as _; // memory layout + panic handler
use proton_peripherals::button::Button;

#[defmt_test::tests]
mod tests {
    use super::wait_for_click;
    use proton_eurorack::system::System;
    use proton_peripherals::cv_input::CvInput;

    #[init]
    fn init() -> System {
        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = daisy::pac::Peripherals::take().unwrap();

        System::init(cp, dp)
    }

    #[test]
    fn pot_spans_expected_range(system: &mut System) {
        let pot = &mut system.pot;

        macro_rules! measure_pot {
            () => {
                wait_for_click(&mut system.button);
                pot.start_sampling(&mut system.adc_1);
                pot.finish_sampling(&mut system.adc_1);
                defmt::info!("Value: {:?}", pot.value());
            };
        }

        defmt::info!("ACTION REQUIRED: Turn pot to min and click encoder");
        measure_pot!();

        defmt::info!("ACTION REQUIRED: Turn pot to max and click encoder");
        measure_pot!();

        defmt::info!("ACTION REQUIRED: Click encoder to continue");
        wait_for_click(&mut system.button);
    }

    #[test]
    fn cv_inputs_span_expected_range(system: &mut System) {
        macro_rules! measure_cv {
            ($cv:ident, $adc:ident) => {
                wait_for_click(&mut system.button);
                system.$cv.start_sampling(&mut system.$adc);
                system.$cv.finish_sampling(&mut system.$adc);
                defmt::info!("Value: {:?}", system.$cv.value());
            };
        }

        macro_rules! test_cv {
            ($i:expr, $cv:ident, $adc:ident) => {
                defmt::info!(
                    "ACTION REQUIRED: Turn CV {:?} to min and click encoder",
                    $i + 1
                );
                measure_cv!($cv, $adc);

                defmt::info!(
                    "ACTION REQUIRED: Turn CV {:?} to max and click encoder",
                    $i + 1
                );
                measure_cv!($cv, $adc);
            };
        }

        test_cv!(1, cv_input_1, adc_2);
        test_cv!(2, cv_input_2, adc_1);
        test_cv!(3, cv_input_3, adc_2);
        test_cv!(4, cv_input_4, adc_1);
        test_cv!(5, cv_input_5, adc_2);

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

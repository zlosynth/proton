#![no_std]
#![no_main]

use proton_eurorack as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {
    use proton_eurorack::system::System;

    #[init]
    fn init() -> System {
        let cp = cortex_m::Peripherals::take().unwrap();
        let dp = daisy::pac::Peripherals::take().unwrap();

        System::init(cp, dp)
    }

    #[test]
    fn encoder_can_be_clicked(system: &mut System) {
        defmt::info!("ACTION REQUIRED: Click encoder");
        loop {
            system.button.sample();
            if system.button.clicked() {
                return;
            }
            cortex_m::asm::delay(480_000_000 / 1000);
        }
    }

    #[test]
    fn encoder_can_be_turned_clockwise(system: &mut System) {
        use proton_peripherals::rotary::Direction;

        'repeats: for i in (1..=10).rev() {
            defmt::info!("ACTION REQUIRED: Turn encoder {}x clockwise", i);
            loop {
                system.rotary.sample().unwrap();
                cortex_m::asm::delay(480_000_000 / 1000);
                match system.rotary.direction() {
                    Direction::Clockwise => {
                        continue 'repeats;
                    }
                    Direction::None => (),
                    Direction::CounterClockwise => panic!("Reverse direction was detected"),
                }
            }
        }
    }

    #[test]
    fn encoder_can_be_turned_counter_clockwise(system: &mut System) {
        use proton_peripherals::rotary::Direction;

        'repeats: for i in (1..=10).rev() {
            defmt::info!("ACTION REQUIRED: Turn encoder {}x counter clockwise", i);
            loop {
                system.rotary.sample().unwrap();
                cortex_m::asm::delay(480_000_000 / 1000);
                match system.rotary.direction() {
                    Direction::CounterClockwise => {
                        continue 'repeats;
                    }
                    Direction::None => (),
                    Direction::Clockwise => panic!("Reverse direction was detected"),
                }
            }
        }
    }
}

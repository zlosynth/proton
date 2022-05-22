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
    fn encoder_alpha_can_be_clicked(system: &mut System) {
        defmt::info!("ACTION REQUIRED: Click Alpha");
        while !system.alpha_button.clicked() {
            system.alpha_button.sample();
            cortex_m::asm::nop();
        }
    }

    #[test]
    fn encoder_alpha_can_be_turned_clockwise(system: &mut System) {
        use proton_peripherals::detent_rotary::Direction;
        defmt::info!("ACTION REQUIRED: Turn Alpha clockwise");
        loop {
            system.alpha_rotary.sample().unwrap();
            match system.alpha_rotary.direction() {
                Direction::Clockwise => {
                    return;
                }
                Direction::None => (),
                Direction::CounterClockwise => panic!("Reverse direction was detected"),
            }
        }
    }

    #[test]
    fn encoder_alpha_can_be_turned_counter_clockwise(system: &mut System) {
        use proton_peripherals::detent_rotary::Direction;
        defmt::info!("ACTION REQUIRED: Turn Alpha counter clockwise");
        loop {
            system.alpha_rotary.sample().unwrap();
            match system.alpha_rotary.direction() {
                Direction::CounterClockwise => {
                    return;
                }
                Direction::None => (),
                Direction::Clockwise => panic!("Reverse direction was detected"),
            }
        }
    }

    #[test]
    fn encoder_beta_can_be_clicked(system: &mut System) {
        defmt::info!("ACTION REQUIRED: Click Beta");
        while !system.beta_button.clicked() {
            system.beta_button.sample();
            cortex_m::asm::nop();
        }
    }

    #[test]
    fn encoder_beta_can_be_turned_clockwise(system: &mut System) {
        use proton_peripherals::detent_rotary::Direction;
        defmt::info!("ACTION REQUIRED: Turn Beta clockwise");
        loop {
            system.beta_rotary.sample().unwrap();
            match system.beta_rotary.direction() {
                Direction::Clockwise => {
                    return;
                }
                Direction::None => (),
                Direction::CounterClockwise => panic!("Reverse direction was detected"),
            }
        }
    }

    #[test]
    fn encoder_beta_can_be_turned_counter_clockwise(system: &mut System) {
        use proton_peripherals::detent_rotary::Direction;
        defmt::info!("ACTION REQUIRED: Turn Beta counter clockwise");
        loop {
            system.beta_rotary.sample().unwrap();
            match system.beta_rotary.direction() {
                Direction::CounterClockwise => {
                    return;
                }
                Direction::None => (),
                Direction::Clockwise => panic!("Reverse direction was detected"),
            }
        }
    }
}

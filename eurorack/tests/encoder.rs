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
        while !system.button.clicked() {
            system.button.sample();
            cortex_m::asm::nop();
        }
    }

    #[test]
    fn encoder_can_be_turned_clockwise(system: &mut System) {
        use proton_peripherals::rotary::Direction;
        defmt::info!("ACTION REQUIRED: Turn encoder clockwise");
        loop {
            system.rotary.sample().unwrap();
            match system.rotary.direction() {
                Direction::Clockwise => {
                    return;
                }
                Direction::None => (),
                Direction::CounterClockwise => panic!("Reverse direction was detected"),
            }
        }
    }

    #[test]
    fn encoder_can_be_turned_counter_clockwise(system: &mut System) {
        use proton_peripherals::rotary::Direction;
        defmt::info!("ACTION REQUIRED: Turn encoder counter clockwise");
        loop {
            system.rotary.sample().unwrap();
            match system.rotary.direction() {
                Direction::CounterClockwise => {
                    return;
                }
                Direction::None => (),
                Direction::Clockwise => panic!("Reverse direction was detected"),
            }
        }
    }
}

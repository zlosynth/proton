pub mod display;
pub mod encoder;
pub mod led;

use daisy::hal;
use hal::pac::CorePeripherals;
use hal::pac::Peripherals as DevicePeripherals;
use systick_monotonic::Systick;

use display::{Display, DisplayPins};
use encoder::{AlphaButton, AlphaRotary, BetaButton, BetaRotary};
use led::Led;

pub struct System {
    pub display: Display,
    pub led: Led,
    pub mono: Systick<1000>,
    pub alpha_button: AlphaButton,
    pub alpha_rotary: AlphaRotary,
    pub beta_button: BetaButton,
    pub beta_rotary: BetaRotary,
}

impl System {
    pub fn init(mut cp: CorePeripherals, dp: DevicePeripherals) -> Self {
        enable_cache(&mut cp);

        let board = daisy::Board::take().unwrap();
        let ccdr = daisy::board_freeze_clocks!(board, dp);
        let pins = daisy::board_split_gpios!(board, ccdr, dp);
        let led = daisy::board_split_leds!(pins).USER;

        let display = display::init(
            DisplayPins {
                SCK: pins.GPIO.PIN_8.into_alternate(),
                CS: pins.GPIO.PIN_7.into_push_pull_output(),
                MOSI: pins.GPIO.PIN_10.into_alternate(),
                RST: pins.GPIO.PIN_30.into_push_pull_output(),
                DC: pins.GPIO.PIN_9.into_push_pull_output(),
            },
            dp.TIM2,
            ccdr.peripheral.TIM2,
            dp.SPI1,
            ccdr.peripheral.SPI1,
            &ccdr.clocks,
        );

        let mono = Systick::new(cp.SYST, 480_000_000);

        let alpha_button = AlphaButton::new(pins.GPIO.PIN_20.into_pull_up_input());
        let alpha_rotary = AlphaRotary::new(
            pins.GPIO.PIN_21.into_pull_up_input(),
            pins.GPIO.PIN_24.into_pull_up_input(),
        );
        let beta_button = BetaButton::new(pins.GPIO.PIN_12.into_pull_up_input());
        let beta_rotary = BetaRotary::new(
            pins.GPIO.PIN_14.into_pull_up_input(),
            pins.GPIO.PIN_13.into_pull_up_input(),
        );

        Self {
            display,
            led,
            mono,
            alpha_button,
            alpha_rotary,
            beta_button,
            beta_rotary,
        }
    }
}

/// AN5212: Improve application performance when fetching instruction and
/// data, from both internal andexternal memories.
fn enable_cache(cp: &mut CorePeripherals) {
    cp.SCB.enable_icache();
}

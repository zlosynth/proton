pub mod audio;
pub mod cv_input;
pub mod display;
pub mod encoder;
pub mod led;
pub mod randomizer;

use daisy::hal;
use hal::adc::{Adc, AdcSampleTime, Enabled, Resolution};
use hal::delay::DelayFromCountDownTimer;
use hal::pac::CorePeripherals;
use hal::pac::Peripherals as DevicePeripherals;
use hal::pac::{ADC1, ADC2};
use hal::prelude::*;
use systick_monotonic::Systick;

use audio::Audio;
use cv_input::{CvInput1, CvInput2, CvInput3, CvInput4, CvInput5};
use display::{Display, DisplayPins};
use encoder::{EncoderButton, EncoderRotary};
use led::Led;
use randomizer::Randomizer;

pub struct System {
    pub audio: Audio,
    pub display: Display,
    pub led: Led,
    pub mono: Systick<1000>,
    pub button: EncoderButton,
    pub rotary: EncoderRotary,
    pub cv_input_1: CvInput1,
    pub cv_input_2: CvInput2,
    pub cv_input_3: CvInput3,
    pub cv_input_4: CvInput4,
    pub cv_input_5: CvInput5,
    pub adc_1: Adc<ADC1, Enabled>,
    pub adc_2: Adc<ADC2, Enabled>,
    pub randomizer: Randomizer,
}

impl System {
    pub fn init(mut cp: CorePeripherals, dp: DevicePeripherals) -> Self {
        enable_cache(&mut cp);

        let board = daisy::Board::take().unwrap();
        let ccdr = daisy::board_freeze_clocks!(board, dp);
        let pins = daisy::board_split_gpios!(board, ccdr, dp);
        let led = daisy::board_split_leds!(pins).USER;
        let audio = Audio::init(daisy::board_split_audio!(ccdr, pins));

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

        let button = EncoderButton::new(pins.GPIO.PIN_20.into_pull_up_input());
        let rotary = EncoderRotary::new(
            pins.GPIO.PIN_21.into_pull_up_input(),
            pins.GPIO.PIN_24.into_pull_up_input(),
        );

        let cv_input_1 = CvInput1::new(pins.GPIO.PIN_15);
        let cv_input_2 = CvInput2::new(pins.GPIO.PIN_16);
        let cv_input_3 = CvInput3::new(pins.GPIO.PIN_17);
        let cv_input_4 = CvInput4::new(pins.GPIO.PIN_18);
        let cv_input_5 = CvInput5::new(pins.GPIO.PIN_19);

        let (adc_1, adc_2) = {
            let mut delay = DelayFromCountDownTimer::new(dp.TIM3.timer(
                100.Hz(),
                ccdr.peripheral.TIM3,
                &ccdr.clocks,
            ));
            let (mut adc_1, mut adc_2) = hal::adc::adc12(
                dp.ADC1,
                dp.ADC2,
                &mut delay,
                ccdr.peripheral.ADC12,
                &ccdr.clocks,
            );
            adc_1.set_resolution(Resolution::SIXTEENBIT);
            adc_1.set_sample_time(AdcSampleTime::T_16);
            adc_2.set_resolution(Resolution::SIXTEENBIT);
            adc_2.set_sample_time(AdcSampleTime::T_16);
            (adc_1.enable(), adc_2.enable())
        };

        let randomizer = Randomizer::new(dp.RNG.constrain(ccdr.peripheral.RNG, &ccdr.clocks));

        Self {
            audio,
            display,
            led,
            mono,
            button,
            rotary,
            cv_input_1,
            cv_input_2,
            cv_input_3,
            cv_input_4,
            cv_input_5,
            adc_1,
            adc_2,
            randomizer,
        }
    }
}

/// AN5212: Improve application performance when fetching instruction and
/// data, from both internal andexternal memories.
fn enable_cache(cp: &mut CorePeripherals) {
    cp.SCB.enable_icache();
}

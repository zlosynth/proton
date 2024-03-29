pub mod audio;
pub mod cv_input;
pub mod cv_output;
pub mod display;
pub mod encoder;
pub mod gate_output;
pub mod led;
pub mod randomizer;
pub mod sdmmc;

use daisy::sdram::SDRAM;
use hal::adc::{Adc, AdcSampleTime, Enabled, Resolution};
use hal::delay::DelayFromCountDownTimer;
use hal::gpio::Speed;
use hal::pac::CorePeripherals;
use hal::pac::Peripherals as DevicePeripherals;
use hal::pac::{ADC1, ADC2};
use hal::prelude::*;
use stm32h7xx_hal as hal;
use systick_monotonic::Systick;

use audio::Audio;
use cv_input::{CvInput1, CvInput2, CvInput3, CvInput4, CvInput5, Pot};
use cv_output::{CvOutput1, CvOutput2};
use display::{Display, DisplayPins};
use encoder::{EncoderButton, EncoderRotary};
use gate_output::{GateOutput1, GateOutput2, GateOutput3};
use led::Led;
use randomizer::Randomizer;
use sdmmc::SDMMC;

pub struct System {
    pub audio: Audio,
    pub display: Display,
    pub led: Led,
    pub mono: Systick<1000>,
    pub button: EncoderButton,
    pub rotary: EncoderRotary,
    pub pot: Pot,
    pub cv_input_1: CvInput1,
    pub cv_input_2: CvInput2,
    pub cv_input_3: CvInput3,
    pub cv_input_4: CvInput4,
    pub cv_input_5: CvInput5,
    pub gate_1: GateOutput1,
    pub gate_2: GateOutput2,
    pub gate_3: GateOutput3,
    pub cv_output_1: CvOutput1,
    pub cv_output_2: CvOutput2,
    pub adc_1: Adc<ADC1, Enabled>,
    pub adc_2: Adc<ADC2, Enabled>,
    pub sdram: SDRAM,
    pub sdmmc: SDMMC,
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
        let sdram = daisy::board_split_sdram!(cp, dp, ccdr, pins);

        let mut delay = DelayFromCountDownTimer::new(dp.TIM2.timer(
            100.Hz(),
            ccdr.peripheral.TIM2,
            &ccdr.clocks,
        ));

        let display = display::init(
            DisplayPins {
                SCK: pins.GPIO.PIN_8.into_alternate(),
                CS: pins.GPIO.PIN_7.into_push_pull_output(),
                MOSI: pins.GPIO.PIN_10.into_alternate(),
                RST: pins.GPIO.PIN_30.into_push_pull_output(),
                DC: pins.GPIO.PIN_9.into_push_pull_output(),
            },
            &mut delay,
            dp.SPI1,
            ccdr.peripheral.SPI1,
            &ccdr.clocks,
        );

        let mono = Systick::new(cp.SYST, 480_000_000);

        let button = EncoderButton::new(pins.GPIO.PIN_26.into_pull_up_input());
        let rotary = EncoderRotary::new(
            pins.GPIO.PIN_21.into_pull_up_input(),
            pins.GPIO.PIN_24.into_pull_up_input(),
        );

        let pot = Pot::new(pins.GPIO.PIN_20);
        let cv_input_1 = CvInput1::new(pins.GPIO.PIN_15);
        let cv_input_2 = CvInput2::new(pins.GPIO.PIN_17);
        let cv_input_3 = CvInput3::new(pins.GPIO.PIN_18);
        let cv_input_4 = CvInput4::new(pins.GPIO.PIN_19);
        let cv_input_5 = CvInput5::new(pins.GPIO.PIN_16);

        let gate_1 = GateOutput1::new(pins.GPIO.PIN_27.into_push_pull_output());
        let gate_2 = GateOutput2::new(pins.GPIO.PIN_28.into_push_pull_output());
        let gate_3 = GateOutput3::new(pins.GPIO.PIN_29.into_push_pull_output());

        let (cv_output_1, cv_output_2) = cv_output::init(
            (pins.GPIO.PIN_23, pins.GPIO.PIN_22),
            dp.DAC,
            ccdr.peripheral.DAC12,
            &mut delay,
        );

        let (adc_1, adc_2) = {
            let (mut adc_1, mut adc_2) = hal::adc::adc12(
                dp.ADC1,
                dp.ADC2,
                4.MHz(),
                &mut delay,
                ccdr.peripheral.ADC12,
                &ccdr.clocks,
            );
            adc_1.set_resolution(Resolution::SixteenBit);
            adc_1.set_sample_time(AdcSampleTime::T_16);
            adc_2.set_resolution(Resolution::SixteenBit);
            adc_2.set_sample_time(AdcSampleTime::T_16);
            (adc_1.enable(), adc_2.enable())
        };

        let sdmmc = {
            let (clk, cmd, d0, d1, d2, d3) = (
                pins.GPIO.PIN_6,
                pins.GPIO.PIN_5,
                pins.GPIO.PIN_4,
                pins.GPIO.PIN_3,
                pins.GPIO.PIN_2,
                pins.GPIO.PIN_1,
            );

            let clk = clk
                .into_alternate::<12>()
                .internal_pull_up(false)
                .speed(Speed::VeryHigh);
            let clk = clk
                .into_alternate()
                .internal_pull_up(false)
                .speed(Speed::VeryHigh);
            let cmd = cmd
                .into_alternate()
                .internal_pull_up(true)
                .speed(Speed::VeryHigh);
            let d0 = d0
                .into_alternate()
                .internal_pull_up(true)
                .speed(Speed::VeryHigh);
            let d1 = d1
                .into_alternate()
                .internal_pull_up(true)
                .speed(Speed::VeryHigh);
            let d2 = d2
                .into_alternate()
                .internal_pull_up(true)
                .speed(Speed::VeryHigh);
            let d3 = d3
                .into_alternate()
                .internal_pull_up(true)
                .speed(Speed::VeryHigh);

            let mut sdmmc: SDMMC = dp.SDMMC1.sdmmc(
                (clk, cmd, d0, d1, d2, d3),
                ccdr.peripheral.SDMMC1,
                &ccdr.clocks,
            );

            let bus_frequency = 50.MHz();

            for _ in 0..10 {
                if sdmmc.init(bus_frequency).is_ok() {
                    break;
                }
                cortex_m::asm::delay(480_000_000);
            }

            sdmmc
        };

        let randomizer = Randomizer::new(dp.RNG.constrain(ccdr.peripheral.RNG, &ccdr.clocks));

        Self {
            audio,
            display,
            led,
            mono,
            button,
            rotary,
            pot,
            cv_input_1,
            cv_input_2,
            cv_input_3,
            cv_input_4,
            cv_input_5,
            gate_1,
            gate_2,
            gate_3,
            cv_output_1,
            cv_output_2,
            adc_1,
            adc_2,
            sdram,
            sdmmc,
            randomizer,
        }
    }
}

/// AN5212: Improve application performance when fetching instruction and
/// data, from both internal andexternal memories.
fn enable_cache(cp: &mut CorePeripherals) {
    cp.SCB.enable_icache();
    // NOTE: This requires cache management around all use of DMA.
    cp.SCB.enable_dcache(&mut cp.CPUID);
}

use proton_peripherals::cv_output::CvOutput as PeripheralCvOutput;

use crate::system::hal;
use hal::dac::{DacExt, Enabled, C1, C2};
use hal::gpio;
use hal::hal::blocking::delay::DelayUs;
use hal::traits::DacOut;

type C1Pin = gpio::gpioa::PA4<hal::gpio::Analog>;
type C2Pin = gpio::gpioa::PA5<hal::gpio::Analog>;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CvOutput<D> {
    dac: D,
}

pub fn init(
    pins: (C1Pin, C2Pin),
    dp_dac: hal::pac::DAC,
    ccdr_dac: hal::rcc::rec::Dac12,
    delay: &mut impl DelayUs<u32>,
) -> (CvOutput1, CvOutput2) {
    let (dac1, dac2) = dp_dac.dac((pins.0, pins.1), ccdr_dac);
    let dac1 = dac1.calibrate_buffer(delay).enable();
    let dac2 = dac2.calibrate_buffer(delay).enable();
    (CvOutput1 { dac: dac2 }, CvOutput2 { dac: dac1 })
}

impl<D> PeripheralCvOutput for CvOutput<D>
where
    D: DacOut<u16>,
{
    fn set_value(&mut self, value: f32) {
        self.dac.set_value(transpose_dac(value));
    }
}

fn transpose_dac(value: f32) -> u16 {
    u16::min((value * (2 << 11) as f32) as u16, 0xfff)
}

pub type CvOutput1 = CvOutput<C2<hal::pac::DAC, Enabled>>;
pub type CvOutput2 = CvOutput<C1<hal::pac::DAC, Enabled>>;

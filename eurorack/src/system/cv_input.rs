use core::marker::PhantomData;

use nb::block;

use proton_peripherals::cv_input::CvInput as PeripheralCvInput;

use crate::system::hal::adc::{Adc, Enabled};
use crate::system::hal::gpio;
use crate::system::hal::hal::adc::Channel;
use crate::system::hal::pac::{ADC1, ADC2};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CvInput<A, P> {
    pin: P,
    value: f32,
    _adc: PhantomData<A>,
}

macro_rules! cv_input {
    ($adc:ident) => {
        impl<P: Channel<$adc, ID = u8>> CvInput<$adc, P> {
            pub fn new(pin: P) -> Self {
                Self {
                    pin,
                    value: 0.0,
                    _adc: PhantomData,
                }
            }
        }

        impl<P: Channel<$adc, ID = u8>> PeripheralCvInput for CvInput<$adc, P> {
            type Adc = Adc<$adc, Enabled>;

            fn start_sampling(&mut self, adc: &mut Self::Adc) {
                adc.start_conversion(&mut self.pin);
            }

            fn finish_sampling(&mut self, adc: &mut Self::Adc) {
                let sample: u32 = block!(adc.read_sample()).unwrap();
                self.value = transpose_adc(sample as f32, adc.slope());
            }

            fn value(&self) -> f32 {
                self.value
            }
        }
    };
}

fn transpose_adc(sample: f32, slope: u32) -> f32 {
    (slope as f32 - sample) / slope as f32
}

cv_input!(ADC1);
cv_input!(ADC2);

pub type CvInput1 = CvInput<ADC1, gpio::gpioc::PC0<gpio::Analog>>;
pub type CvInput2 = CvInput<ADC2, gpio::gpioa::PA3<gpio::Analog>>;
pub type CvInput3 = CvInput<ADC1, gpio::gpiob::PB1<gpio::Analog>>;
pub type CvInput4 = CvInput<ADC2, gpio::gpioa::PA7<gpio::Analog>>;
pub type CvInput5 = CvInput<ADC1, gpio::gpioa::PA6<gpio::Analog>>;

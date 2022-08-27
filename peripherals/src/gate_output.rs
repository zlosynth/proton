use embedded_hal::digital::v2::OutputPin;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GateOutput<P> {
    pin: P,
}

impl<P: OutputPin> GateOutput<P> {
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    pub fn pin(&mut self) -> &mut P {
        &mut self.pin
    }
}

pub trait GateOutputExt {
    fn set(&mut self);
    fn reset(&mut self);
    fn set_value(&mut self, on: bool) {
        if on {
            self.set();
        } else {
            self.reset();
        }
    }
}

impl<P: OutputPin> GateOutputExt for GateOutput<P> {
    fn set(&mut self) {
        self.pin.set_high().ok().unwrap();
    }

    fn reset(&mut self) {
        self.pin.set_low().ok().unwrap();
    }
}

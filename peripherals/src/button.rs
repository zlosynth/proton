use embedded_hal::digital::v2::InputPin;

use super::debounce_buffer::DebounceBuffer;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Button<const N: usize, P> {
    pin: P,
    debounce_filter: DebounceBuffer<N>,
    active: bool,
    clicked: bool,
}

impl<const N: usize, P: InputPin> Button<N, P> {
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            debounce_filter: DebounceBuffer::new(),
            active: false,
            clicked: false,
        }
    }

    pub fn pin(&mut self) -> &mut P {
        &mut self.pin
    }

    pub fn sample(&mut self) {
        let was_active = self.active;
        self.debounce_filter.write(self.pin.is_low().ok().unwrap());
        self.active = self.debounce_filter.read();
        self.clicked = !was_active && self.active;
    }

    pub fn active(&self) -> bool {
        self.debounce_filter.read()
    }

    pub fn active_no_filter(&self) -> bool {
        self.pin.is_low().ok().unwrap()
    }

    pub fn clicked(&self) -> bool {
        self.clicked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPin {
        pub high: bool,
    }

    impl TestPin {
        fn new() -> Self {
            Self { high: false }
        }
    }

    impl embedded_hal::digital::v2::InputPin for TestPin {
        type Error = ();

        fn is_high(&self) -> Result<bool, Self::Error> {
            Ok(self.high)
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.high)
        }
    }

    #[test]
    fn when_held_down_it_reports_clicked_for_one_cycle() {
        let a = TestPin::new();
        let mut button = Button::<3, _>::new(a);

        button.pin().high = true;
        for _ in 0..3 {
            button.sample();
        }
        assert!(!button.clicked());

        button.pin().high = false;
        for _ in 0..2 {
            button.sample();
        }
        assert!(button.clicked());

        button.sample();
        assert!(!button.clicked());
    }

    #[test]
    fn when_held_down_it_reports_as_active() {
        let a = TestPin::new();
        let mut button = Button::<3, _>::new(a);

        button.pin().high = false;
        for _ in 0..3 {
            button.sample();
        }
        assert!(button.active());

        button.pin().high = true;
        for _ in 0..3 {
            button.sample();
        }
        assert!(!button.active());
    }

    #[test]
    fn when_read_without_filter_it_responds_immediately() {
        let a = TestPin::new();
        let mut button = Button::<3, _>::new(a);

        button.pin().high = false;
        assert!(button.active_no_filter());

        button.pin().high = true;
        assert!(!button.active_no_filter());
    }
}

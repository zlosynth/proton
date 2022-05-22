// Kudos to https://github.com/leshow/rotary-encoder-hal

use either::Either;
use embedded_hal::digital::v2::InputPin;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Rotary<A, B> {
    pin_a: A,
    pin_b: B,
    state: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    Clockwise,
    CounterClockwise,
    None,
}

impl From<u8> for Direction {
    fn from(transition: u8) -> Self {
        match transition {
            0b0001 | 0b0111 | 0b1000 | 0b1110 => Direction::Clockwise,
            0b0010 | 0b0100 | 0b1011 | 0b1101 => Direction::CounterClockwise,
            _ => Direction::None,
        }
    }
}

impl<A, B> Rotary<A, B>
where
    A: InputPin,
    B: InputPin,
{
    pub fn new(pin_a: A, pin_b: B) -> Self {
        Self {
            pin_a,
            pin_b,
            state: 0u8,
        }
    }

    pub fn pin_a(&mut self) -> &mut A {
        &mut self.pin_a
    }

    pub fn pin_b(&mut self) -> &mut B {
        &mut self.pin_b
    }

    pub fn update(&mut self) -> Result<Direction, Either<A::Error, B::Error>> {
        // use mask to get previous state value
        let mut transition = self.state & 0b11;

        let (a_is_high, b_is_high) = (self.pin_a.is_high(), self.pin_b.is_high());

        // move in the new state
        if a_is_high.map_err(Either::Left)? {
            transition |= 0b1000;
        }
        if b_is_high.map_err(Either::Right)? {
            transition |= 0b100;
        }

        // move new state in
        self.state = transition >> 2;

        Ok(transition.into())
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
    fn it_should_work() {
        use Direction::*;

        let states = [
            // B A Direction
            (false, false, None),
            (false, true, Clockwise),
            (true, true, Clockwise),
            (true, false, Clockwise),
            (false, false, Clockwise),
            (true, false, CounterClockwise),
            (true, true, CounterClockwise),
            (true, true, None),
            (false, true, CounterClockwise),
            (false, false, CounterClockwise),
        ];

        let a = TestPin::new();
        let b = TestPin::new();
        let mut rotary = Rotary::new(a, b);

        for (b_high, a_high, direction) in states {
            rotary.pin_a().high = a_high;
            rotary.pin_b().high = b_high;
            assert_eq!(rotary.update().unwrap(), direction);
        }
    }
}

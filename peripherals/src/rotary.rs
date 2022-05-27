// Kudos to https://github.com/leshow/rotary-encoder-hal

use either::Either;
use embedded_hal::digital::v2::InputPin;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Rotary<A, B> {
    pin_a: A,
    pin_b: B,
    traveled: i8,
    transition: u8,
    direction: Direction,
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
            traveled: 0i8,
            transition: 0u8,
            direction: Direction::None,
        }
    }

    pub fn pin_a(&mut self) -> &mut A {
        &mut self.pin_a
    }

    pub fn pin_b(&mut self) -> &mut B {
        &mut self.pin_b
    }

    pub fn sample(&mut self) -> Result<(), Either<A::Error, B::Error>> {
        let mut transition = self.transition;

        // discard the pre-previous state
        transition >>= 2;

        let (a_is_low, b_is_low) = (self.pin_a.is_low(), self.pin_b.is_low());

        // record the new state
        if a_is_low.map_err(Either::Left)? {
            transition |= 0b1000;
        }
        if b_is_low.map_err(Either::Right)? {
            transition |= 0b100;
        }

        self.transition = transition;

        self.traveled = match self.transition.into() {
            Direction::Clockwise => self.traveled + 1,
            Direction::CounterClockwise => self.traveled - 1,
            _ => self.traveled,
        };

        // in case some samples were missed and corrupted the traveled tracker,
        // reset to resting point
        if self.transition & 0b1100 == 0b0000 {
            self.traveled = 0;
        }

        // if traveled over a detent, record movement
        if self.traveled == 3 {
            self.direction = self.transition.into();
            self.traveled = -1;
        } else if self.traveled == -3 {
            self.direction = self.transition.into();
            self.traveled = 1;
        } else {
            self.direction = Direction::None;
        }

        Ok(())
    }

    pub fn direction(&self) -> Direction {
        self.direction
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
    fn when_measuring_each_step_it_should_work() {
        use Direction::*;

        let states = [
            // B A Direction
            (false, false, None),            // 0
            (false, true, None),             // >
            (true, true, None),              // >
            (true, false, Clockwise),        // >
            (false, false, None),            // >, 0
            (true, false, None),             // <
            (true, true, None),              // <
            (true, true, None),              // _
            (false, true, CounterClockwise), // <
            (false, false, None),            // <, 0
        ];

        let a = TestPin::new();
        let b = TestPin::new();
        let mut rotary = Rotary::new(a, b);

        for (b_high, a_high, direction) in states {
            rotary.pin_a().high = a_high;
            rotary.pin_b().high = b_high;
            rotary.sample().unwrap();
            assert_eq!(rotary.direction(), direction);
        }
    }
}

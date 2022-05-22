// Kudos to https://github.com/leshow/rotary-encoder-hal

use either::Either;
use embedded_hal::digital::v2::InputPin;

use super::rotary::{Direction as RotaryDirection, Rotary};

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DetentRotary<A, B> {
    rotary: Rotary<A, B>,
    detent: u8,
    traveled: i16,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    Clockwise,
    CounterClockwise,
    None,
}

impl<A, B> DetentRotary<A, B>
where
    A: InputPin,
    B: InputPin,
{
    pub fn new(pin_a: A, pin_b: B, detent: u8) -> Self {
        Self {
            rotary: Rotary::new(pin_a, pin_b),
            detent,
            traveled: 0,
            direction: Direction::None,
        }
    }

    pub fn pin_a(&mut self) -> &mut A {
        self.rotary.pin_a()
    }

    pub fn pin_b(&mut self) -> &mut B {
        self.rotary.pin_b()
    }

    pub fn sample(&mut self) -> Result<(), Either<A::Error, B::Error>> {
        self.rotary.sample()?;

        match self.rotary.direction() {
            RotaryDirection::Clockwise => {
                self.traveled += 1;
                self.direction = if self.traveled == self.detent as i16 {
                    self.traveled = 0;
                    Direction::Clockwise
                } else {
                    Direction::None
                };
            }
            RotaryDirection::CounterClockwise => {
                self.traveled -= 1;
                self.direction = if self.traveled == -(self.detent as i16) {
                    self.traveled = 0;
                    Direction::CounterClockwise
                } else {
                    Direction::None
                };
            }
            _ => self.direction = Direction::None,
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
    fn when_measuring_jumps_between_detents_it_should_work() {
        use Direction::*;

        let states = [
            // B A Direction
            (false, false, None),
            (false, true, None),
            (true, true, None),
            (true, false, None),
            (false, false, Clockwise),
            (true, false, None),
            (true, true, None),
            (false, true, None),
            (false, false, CounterClockwise),
        ];

        let a = TestPin::new();
        let b = TestPin::new();
        let mut rotary = DetentRotary::new(a, b, 4);

        for (b_high, a_high, direction) in states {
            rotary.pin_a().high = a_high;
            rotary.pin_b().high = b_high;
            rotary.sample().unwrap();
            assert_eq!(rotary.direction(), direction);
        }
    }

    #[test]
    fn when_jumping_around_edge_of_detent_it_does_not_record_movement() {
        use Direction::*;

        let states = [
            // B A Direction
            (false, false, None),
            (false, true, None),
            (true, true, None),
            (true, false, None),
            (false, false, Clockwise),
            (true, false, None),  // back
            (false, false, None), // forth
        ];

        let a = TestPin::new();
        let b = TestPin::new();
        let mut rotary = DetentRotary::new(a, b, 4);

        for (b_high, a_high, direction) in states {
            rotary.pin_a().high = a_high;
            rotary.pin_b().high = b_high;
            rotary.sample().unwrap();
            assert_eq!(rotary.direction(), direction);
        }
    }
}

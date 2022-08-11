use embedded_hal::digital::v2::InputPin;
use heapless::Vec;
use proton_peripherals::button::Button;
use proton_peripherals::rotary::{Direction, Rotary};

use crate::action::Action;

pub struct Input<A, B, C> {
    button: Button<10, C>,
    rotary: Rotary<A, B>,
}

impl<A, B, C> Input<A, B, C>
where
    A: InputPin,
    B: InputPin,
    C: InputPin,
{
    pub fn new(button: Button<10, C>, rotary: Rotary<A, B>) -> Self {
        Self { button, rotary }
    }

    pub fn process(&mut self) -> Vec<Action, 6> {
        self.button.sample();
        self.rotary.sample().ok().unwrap();

        let mut actions = Vec::new();

        if self.button.clicked() {
            actions.push(Action::EncoderClick).unwrap();
        }

        match self.rotary.direction() {
            Direction::Clockwise => actions.push(Action::EncoderDown).unwrap(),
            Direction::CounterClockwise => actions.push(Action::EncoderUp).unwrap(),
            _ => (),
        }

        actions
    }
}

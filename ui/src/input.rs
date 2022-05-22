use embedded_hal::digital::v2::InputPin;
use heapless::Vec;
use proton_peripherals::button::Button;
use proton_peripherals::detent_rotary::{DetentRotary, Direction};

use crate::action::Action;

pub struct Input<A1, B1, C1, A2, B2, C2> {
    alpha_button: Button<10, C1>,
    alpha_rotary: DetentRotary<A1, B1>,
    beta_button: Button<10, C2>,
    beta_rotary: DetentRotary<A2, B2>,
}

impl<A1, B1, C1, A2, B2, C2> Input<A1, B1, C1, A2, B2, C2>
where
    A1: InputPin,
    B1: InputPin,
    C1: InputPin,
    A2: InputPin,
    B2: InputPin,
    C2: InputPin,
{
    pub fn new(
        alpha_button: Button<10, C1>,
        alpha_rotary: DetentRotary<A1, B1>,
        beta_button: Button<10, C2>,
        beta_rotary: DetentRotary<A2, B2>,
    ) -> Self {
        Self {
            alpha_button,
            alpha_rotary,
            beta_button,
            beta_rotary,
        }
    }

    pub fn process(&mut self) -> Vec<Action, 6> {
        self.alpha_button.sample();
        self.alpha_rotary.sample().ok().unwrap();
        self.beta_button.sample();
        self.beta_rotary.sample().ok().unwrap();

        let mut actions = Vec::new();

        if self.alpha_button.clicked() {
            actions.push(Action::AlphaClick).unwrap();
        }

        match self.alpha_rotary.direction() {
            Direction::Clockwise => actions.push(Action::AlphaDown).unwrap(),
            Direction::CounterClockwise => actions.push(Action::AlphaUp).unwrap(),
            _ => (),
        }

        if self.beta_button.clicked() {
            actions.push(Action::BetaClick).unwrap();
        }

        match self.beta_rotary.direction() {
            Direction::Clockwise => actions.push(Action::BetaDown).unwrap(),
            Direction::CounterClockwise => actions.push(Action::BetaUp).unwrap(),
            _ => (),
        }

        actions
    }
}

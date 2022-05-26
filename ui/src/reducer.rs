use super::action::Action;
use super::state::State;

pub fn reduce(action: Action, state: &mut State) {
    match action {
        Action::AlphaUp => {
            move_to_previous_attribute(state);
        }
        Action::AlphaDown => {
            move_to_next_attribute(state);
        }
        Action::AlphaClick => (),
        Action::BetaUp => (),
        Action::BetaDown => (),
        Action::BetaClick => (),
    }
}

fn move_to_previous_attribute(state: &mut State) {
    if state.selected_attribute == 0 {
        state.selected_attribute = state.attributes.len() - 1;
    } else {
        state.selected_attribute -= 1;
    }
}

fn move_to_next_attribute(state: &mut State) {
    state.selected_attribute = (state.selected_attribute + 1) % state.attributes.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_alpha_turns_up_on_middle_attribute_it_scrolls_to_previous_attribute() {
        use crate::state::*;
        use heapless::Vec;

        let mut state = State {
            title: "Proton",
            attributes: Vec::from_slice(&[
                Attribute {
                    name: "a1",
                    value: Value::F32(1.0),
                },
                Attribute {
                    name: "a2",
                    value: Value::F32(1.0),
                },
                Attribute {
                    name: "a3",
                    value: Value::F32(1.0),
                },
            ])
            .unwrap(),
            selected_attribute: 1,
        };
        reduce(Action::AlphaUp, &mut state);

        assert_eq!(state.selected_attribute, 0);
    }

    #[test]
    fn when_alpha_turns_up_on_first_attribute_it_scrolls_to_last_attribute() {
        use crate::state::*;
        use heapless::Vec;

        let mut state = State {
            title: "Proton",
            attributes: Vec::from_slice(&[
                Attribute {
                    name: "a1",
                    value: Value::F32(1.0),
                },
                Attribute {
                    name: "a2",
                    value: Value::F32(1.0),
                },
                Attribute {
                    name: "a3",
                    value: Value::F32(1.0),
                },
            ])
            .unwrap(),
            selected_attribute: 0,
        };
        reduce(Action::AlphaUp, &mut state);

        assert_eq!(state.selected_attribute, 2);
    }

    #[test]
    fn when_alpha_turns_down_on_middle_attribute_it_scrolls_to_next_attribute() {
        use crate::state::*;
        use heapless::Vec;

        let mut state = State {
            title: "Proton",
            attributes: Vec::from_slice(&[
                Attribute {
                    name: "a1",
                    value: Value::F32(1.0),
                },
                Attribute {
                    name: "a2",
                    value: Value::F32(1.0),
                },
                Attribute {
                    name: "a3",
                    value: Value::F32(1.0),
                },
            ])
            .unwrap(),
            selected_attribute: 1,
        };
        reduce(Action::AlphaDown, &mut state);

        assert_eq!(state.selected_attribute, 2);
    }

    #[test]
    fn when_alpha_turns_down_on_last_attribute_it_scrolls_to_first_attribute() {
        use crate::state::*;
        use heapless::Vec;

        let mut state = State {
            title: "Proton",
            attributes: Vec::from_slice(&[
                Attribute {
                    name: "a1",
                    value: Value::F32(1.0),
                },
                Attribute {
                    name: "a2",
                    value: Value::F32(1.0),
                },
                Attribute {
                    name: "a3",
                    value: Value::F32(1.0),
                },
            ])
            .unwrap(),
            selected_attribute: 2,
        };
        reduce(Action::AlphaDown, &mut state);

        assert_eq!(state.selected_attribute, 0);
    }
}

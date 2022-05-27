use super::action::Action;
use super::state::{State, Value};

pub fn reduce(action: Action, state: &mut State) {
    match action {
        Action::AlphaUp => move_to_previous_attribute(state),
        Action::AlphaDown => move_to_next_attribute(state),
        Action::AlphaClick => (),
        Action::BetaUp => decrease_attribute_value(state),
        Action::BetaDown => increase_attribute_value(state),
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

fn decrease_attribute_value(state: &mut State) {
    let attribute = &mut state.attributes[state.selected_attribute];

    match &mut attribute.value {
        Value::F32(value_f32) => {
            value_f32.value = (value_f32.value - value_f32.step).max(0.0);
        }
        Value::Select(value_select) => {
            if value_select.selected == 0 {
                value_select.selected = value_select.available.len() - 1;
            } else {
                value_select.selected -= 1;
            }
        }
    }
}

fn increase_attribute_value(state: &mut State) {
    let attribute = &mut state.attributes[state.selected_attribute];

    match &mut attribute.value {
        Value::F32(value_f32) => {
            value_f32.value = (value_f32.value + value_f32.step).min(1.0);
        }
        Value::Select(value_select) => {
            value_select.selected = (value_select.selected + 1) % value_select.available.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_alpha_turns_up_on_middle_attribute_it_scrolls_to_previous_attribute() {
        assert_attribute_transition(1, Action::AlphaUp, 0);
    }

    #[test]
    fn when_alpha_turns_up_on_first_attribute_it_scrolls_to_last_attribute() {
        assert_attribute_transition(0, Action::AlphaUp, 2);
    }

    #[test]
    fn when_alpha_turns_down_on_middle_attribute_it_scrolls_to_next_attribute() {
        assert_attribute_transition(1, Action::AlphaDown, 2);
    }

    #[test]
    fn when_alpha_turns_down_on_last_attribute_it_scrolls_to_first_attribute() {
        assert_attribute_transition(2, Action::AlphaDown, 0);
    }

    fn assert_attribute_transition(old: usize, action: Action, new: usize) {
        use crate::state::*;

        let mut state = State::new("Proton")
            .with_attributes(&[
                Attribute::new("a1"),
                Attribute::new("a2"),
                Attribute::new("a3"),
            ])
            .unwrap()
            .with_selected_attribute(old);
        reduce(action, &mut state);

        assert_eq!(state.selected_attribute, new);
    }

    #[test]
    fn given_f32_attribute_in_middle_when_beta_turns_up_it_decreases_the_value_by_set_step() {
        assert_value_f32_transition(0.5, 0.1, Action::BetaUp, 0.4);
    }

    #[test]
    fn given_f32_attribute_on_bottom_when_beta_turns_up_it_does_not_go_below_zero() {
        assert_value_f32_transition(0.05, 0.1, Action::BetaUp, 0.0);
    }

    #[test]
    fn given_f32_attribute_in_middle_when_beta_turns_down_it_increases_the_value_by_set_step() {
        assert_value_f32_transition(0.5, 0.1, Action::BetaDown, 0.6);
    }

    #[test]
    fn given_f32_attribute_on_top_when_beta_turns_down_it_does_not_go_above_one() {
        assert_value_f32_transition(0.95, 0.1, Action::BetaDown, 1.0);
    }

    fn assert_value_f32_transition(old: f32, step: f32, action: Action, new: f32) {
        use crate::state::*;

        let mut state = State::new("Proton")
            .with_attributes(&[
                Attribute::new("a1").with_value_f32(ValueF32::new(old).with_step(step))
            ])
            .unwrap();
        reduce(action, &mut state);

        if let Value::F32(value_f32) = state.attributes[0].value {
            assert_relative_eq!(value_f32.value, new);
        } else {
            unreachable!();
        };
    }

    #[test]
    fn given_select_attribute_on_middle_when_beta_turns_up_it_scrolls_to_previous_value() {
        assert_value_select_transition(1, Action::BetaUp, 0);
    }

    #[test]
    fn given_select_attribute_on_first_when_beta_turns_up_it_scrolls_to_last_value() {
        assert_value_select_transition(0, Action::BetaUp, 2);
    }

    #[test]
    fn given_select_attribute_on_middle_when_beta_turns_down_it_scrolls_to_previous_value() {
        assert_value_select_transition(1, Action::BetaDown, 2);
    }

    #[test]
    fn given_select_attribute_on_first_when_beta_turns_down_it_scrolls_to_last_value() {
        assert_value_select_transition(2, Action::BetaDown, 0);
    }

    fn assert_value_select_transition(old: usize, action: Action, new: usize) {
        use crate::state::*;

        let mut state = State::new("Proton")
            .with_attributes(&[Attribute::new("a1").with_value_select(
                ValueSelect::new(&["v1", "v2", "v3"])
                    .unwrap()
                    .with_selected(old),
            )])
            .unwrap();
        reduce(action, &mut state);

        if let Value::Select(value_select) = &state.attributes[0].value {
            assert_eq!(value_select.selected, new);
        } else {
            unreachable!();
        };
    }
}

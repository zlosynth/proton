use super::action::Action;
use super::reaction::Reaction;
use super::state::{Menu, State, Value, ValueF32, ValueSelect};

pub fn reduce(action: Action, state: &mut State) -> Option<Reaction> {
    match action {
        Action::EncoderClick => {
            switch_menu(state);
            None
        }
        Action::EncoderUp => match state.menu {
            Menu::Main => {
                move_to_previous_attribute(state);
                None
            }
            Menu::Sub => decrease_attribute_value(state),
        },
        Action::EncoderDown => match state.menu {
            Menu::Main => {
                move_to_next_attribute(state);
                None
            }
            Menu::Sub => increase_attribute_value(state),
        },
    }
}

fn switch_menu(state: &mut State) {
    state.menu = match state.menu {
        Menu::Sub => Menu::Main,
        Menu::Main => Menu::Sub,
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

fn decrease_attribute_value(state: &mut State) -> Option<Reaction> {
    let attribute = &mut state.attributes[state.selected_attribute];

    match &mut attribute.value {
        Value::F32(value) => {
            decrease_f32_attribute_value(value).map(|v| Reaction::SetValue(attribute.name, v))
        }
        Value::Select(value) => {
            decrease_select_attribute_value(value).map(|v| Reaction::SelectValue(attribute.name, v))
        }
    }
}

fn increase_attribute_value(state: &mut State) -> Option<Reaction> {
    let attribute = &mut state.attributes[state.selected_attribute];

    match &mut attribute.value {
        Value::F32(value) => {
            increase_f32_attribute_value(value).map(|v| Reaction::SetValue(attribute.name, v))
        }
        Value::Select(value) => {
            increase_select_attribute_value(value).map(|v| Reaction::SelectValue(attribute.name, v))
        }
    }
}

fn decrease_f32_attribute_value(value_f32: &mut ValueF32) -> Option<f32> {
    let old_value = value_f32.value;

    value_f32.value = (value_f32.value - value_f32.step).max(value_f32.min);

    let epsilon = (value_f32.max - value_f32.min) * 0.0001;
    let new_value = value_f32.value;
    if old_value - new_value > epsilon {
        Some(new_value)
    } else {
        None
    }
}

fn increase_f32_attribute_value(value_f32: &mut ValueF32) -> Option<f32> {
    let old_value = value_f32.value;

    value_f32.value = (value_f32.value + value_f32.step).min(value_f32.max);

    let epsilon = (value_f32.max - value_f32.min) * 0.0001;
    let new_value = value_f32.value;
    if new_value - old_value > epsilon {
        Some(new_value)
    } else {
        None
    }
}

fn decrease_select_attribute_value(value_select: &mut ValueSelect) -> Option<&'static str> {
    let old_value = value_select.available[value_select.selected];

    if value_select.selected == 0 {
        value_select.selected = value_select.available.len() - 1;
    } else {
        value_select.selected -= 1;
    }

    let new_value = value_select.available[value_select.selected];
    if old_value != new_value {
        Some(new_value)
    } else {
        None
    }
}

fn increase_select_attribute_value(value_select: &mut ValueSelect) -> Option<&'static str> {
    let old_value = value_select.available[value_select.selected];

    value_select.selected = (value_select.selected + 1) % value_select.available.len();

    let new_value = value_select.available[value_select.selected];
    if old_value != new_value {
        Some(new_value)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_turns_up_on_middle_attribute_it_scrolls_to_previous_attribute() {
        assert_attribute_transition(1, Action::EncoderUp, 0);
    }

    #[test]
    fn when_turns_up_on_first_attribute_it_scrolls_to_last_attribute() {
        assert_attribute_transition(0, Action::EncoderUp, 2);
    }

    #[test]
    fn when_turns_down_on_middle_attribute_it_scrolls_to_next_attribute() {
        assert_attribute_transition(1, Action::EncoderDown, 2);
    }

    #[test]
    fn when_turns_down_on_last_attribute_it_scrolls_to_first_attribute() {
        assert_attribute_transition(2, Action::EncoderDown, 0);
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
        let reaction = reduce(action, &mut state);

        assert_eq!(state.selected_attribute, new);
        assert!(reaction.is_none());
    }

    #[test]
    fn when_clicks_on_selected_attribute_it_enters_submenu() {
        use crate::state::*;

        let mut state = State::new("Proton");
        assert!(matches!(state.menu, Menu::Main));

        let reaction = reduce(Action::EncoderClick, &mut state);
        assert!(reaction.is_none());
        assert!(matches!(state.menu, Menu::Sub));

        let reaction = reduce(Action::EncoderClick, &mut state);
        assert!(reaction.is_none());
        assert!(matches!(state.menu, Menu::Main));
    }

    #[test]
    fn given_f32_attribute_in_middle_when_turns_up_in_submenu_it_decreases_the_value_by_set_step() {
        assert_value_f32_transition_with_reaction_in_submenu(
            ValueF32::new(2.0)
                .with_min(0.0)
                .with_max(10.0)
                .with_step(1.0),
            Action::EncoderUp,
            1.0,
            true,
        );
    }

    #[test]
    fn given_f32_attribute_almost_on_bottom_when_turns_up_in_submenu_it_does_not_go_below_min() {
        assert_value_f32_transition_with_reaction_in_submenu(
            ValueF32::new(-9.5)
                .with_min(-10.0)
                .with_max(0.0)
                .with_step(1.0),
            Action::EncoderUp,
            -10.0,
            true,
        );
    }

    #[test]
    fn given_f32_attribute_on_bottom_when_turns_up_in_submenu_it_does_not_go_below_min() {
        assert_value_f32_transition_with_reaction_in_submenu(
            ValueF32::new(-10.0)
                .with_min(-10.0)
                .with_max(0.0)
                .with_step(1.0),
            Action::EncoderUp,
            -10.0,
            false,
        );
    }

    #[test]
    fn given_f32_attribute_in_middle_when_turns_down_in_submenu_it_increases_the_value_by_set_step()
    {
        assert_value_f32_transition_with_reaction_in_submenu(
            ValueF32::new(1.0)
                .with_min(0.0)
                .with_max(10.0)
                .with_step(1.0),
            Action::EncoderDown,
            2.0,
            true,
        );
    }

    #[test]
    fn given_f32_attribute_almost_on_top_when_turns_down_in_submenu_it_does_not_go_above_max() {
        assert_value_f32_transition_with_reaction_in_submenu(
            ValueF32::new(9.5)
                .with_min(0.0)
                .with_max(10.0)
                .with_step(1.0),
            Action::EncoderDown,
            10.0,
            true,
        );
    }

    #[test]
    fn given_f32_attribute_on_top_when_turns_down_in_submenu_it_does_not_go_above_max() {
        assert_value_f32_transition_with_reaction_in_submenu(
            ValueF32::new(10.0)
                .with_min(0.0)
                .with_max(10.0)
                .with_step(1.0),
            Action::EncoderDown,
            10.0,
            false,
        );
    }

    fn assert_value_f32_transition_with_reaction_in_submenu(
        value_f32: ValueF32,
        action: Action,
        new: f32,
        expect_reaction: bool,
    ) {
        use crate::state::*;

        let mut state = State::new("Proton")
            .with_attributes(&[Attribute::new("a1").with_value_f32(value_f32)])
            .unwrap();
        reduce(Action::EncoderClick, &mut state);
        let reaction = reduce(action, &mut state);

        if let Value::F32(value_f32) = state.attributes[0].value {
            assert_relative_eq!(value_f32.value, new);

            if expect_reaction {
                if let Some(Reaction::SetValue(attribute, value)) = reaction {
                    assert_eq!(attribute, "a1");
                    assert_relative_eq!(value, new);
                } else {
                    panic!("incorrect reaction");
                }
            } else {
                assert!(reaction.is_none());
            }
        } else {
            unreachable!();
        };
    }

    #[test]
    fn given_select_attribute_on_middle_when_turns_up_in_submenu_it_scrolls_to_previous_value() {
        assert_value_select_transition_in_submenu(1, Action::EncoderUp, 0);
    }

    #[test]
    fn given_select_attribute_on_first_when_turns_up_in_submenu_it_scrolls_to_last_value() {
        assert_value_select_transition_in_submenu(0, Action::EncoderUp, 2);
    }

    #[test]
    fn given_select_attribute_on_middle_when_turns_down_in_submenu_it_scrolls_to_previous_value() {
        assert_value_select_transition_in_submenu(1, Action::EncoderDown, 2);
    }

    #[test]
    fn given_select_attribute_on_first_when_turns_down_in_submenu_it_scrolls_to_last_value() {
        assert_value_select_transition_in_submenu(2, Action::EncoderDown, 0);
    }

    fn assert_value_select_transition_in_submenu(old: usize, action: Action, new: usize) {
        use crate::state::*;

        let mut state = State::new("Proton")
            .with_attributes(&[Attribute::new("a1").with_value_select(
                ValueSelect::new(&["v1", "v2", "v3"])
                    .unwrap()
                    .with_selected(old),
            )])
            .unwrap();
        reduce(Action::EncoderClick, &mut state);
        let reaction = reduce(action, &mut state);

        if let Value::Select(value_select) = &state.attributes[0].value {
            assert_eq!(value_select.selected, new);

            if let Some(Reaction::SelectValue(attribute, value)) = reaction {
                assert_eq!(attribute, "a1");
                assert_eq!(value, value_select.available[value_select.selected]);
            } else {
                panic!("incorrect reaction");
            }
        } else {
            unreachable!();
        };
    }
}

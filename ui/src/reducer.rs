use super::action::Action;
use super::state::State;

pub fn reduce(action: Action, state: &mut State) {
    match action {
        Action::AlphaUp => {
            state.selected_attribute -= 1;
        }
        Action::AlphaDown => {
            state.selected_attribute += 1;
        }
        Action::AlphaClick => (),
        Action::BetaUp => (),
        Action::BetaDown => (),
        Action::BetaClick => (),
    }
}

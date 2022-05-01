use super::action::Action;
use super::state::*;

pub fn reduce<NI, CI, PI>(state: &mut State<NI, CI, PI>, action: Action) {
    match action {
        Action::AlphaUp => reduce_alpha_up(state),
        Action::AlphaDown => reduce_alpha_down(state),
        Action::AlphaClick => reduce_alpha_click(state),
    }
}

fn reduce_alpha_up<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    if state.modules.is_empty() {
        return;
    }

    state.selected_module =
        ((state.selected_module as i32 - 1).rem_euclid(state.modules.len() as i32)) as usize;
}

fn reduce_alpha_down<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    if state.modules.is_empty() {
        return;
    }

    state.selected_module += 1;
    state.selected_module %= state.modules.len();
}

fn reduce_alpha_click<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    state.view = match state.view {
        View::Modules => View::Patches,
        View::Patches => View::Modules,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::vec;

    use graphity::Node;

    pub struct TestNode;

    #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
    pub struct TestConsumer;

    #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
    pub struct TestProducer;

    impl Node<bool> for TestNode {
        type Consumer = TestConsumer;
        type Producer = TestProducer;
    }

    graphity!(
        TestGraph<bool>;
        Node = {TestNode, TestConsumer, TestProducer},
    );

    #[test]
    fn when_modules_are_empty_alpha_up_does_nothing() {
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        let original_state = state.clone();

        reduce(&mut state, Action::AlphaUp);
        assert!(state == original_state);
    }

    #[test]
    fn when_at_the_top_of_modules_alpha_up_moves_to_last() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();

        let node1_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node1_handle,
            name: "",
            index: 1,
            attributes: vec![],
            selected_attribute: 0,
        });

        let node2_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node2_handle,
            name: "",
            index: 2,
            attributes: vec![],
            selected_attribute: 0,
        });

        assert_eq!(state.selected_module, 0);
        reduce(&mut state, Action::AlphaUp);
        assert_eq!(state.selected_module, 1);
    }

    #[test]
    fn when_at_the_bottom_of_modules_alpha_up_goes_to_previous() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        state.selected_module = 1;

        let node1_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node1_handle,
            name: "",
            index: 1,
            attributes: vec![],
            selected_attribute: 0,
        });

        let node2_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node2_handle,
            name: "",
            index: 2,
            attributes: vec![],
            selected_attribute: 0,
        });

        reduce(&mut state, Action::AlphaUp);
        assert_eq!(state.selected_module, 0);
    }

    #[test]
    fn when_modules_are_empty_alpha_down_does_nothing() {
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        let original_state = state.clone();

        reduce(&mut state, Action::AlphaDown);
        assert!(state == original_state);
    }

    #[test]
    fn when_at_the_top_of_modules_alpha_down_moves_to_next() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();

        let node1_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node1_handle,
            name: "",
            index: 1,
            attributes: vec![],
            selected_attribute: 0,
        });

        let node2_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node2_handle,
            name: "",
            index: 2,
            attributes: vec![],
            selected_attribute: 0,
        });

        assert_eq!(state.selected_module, 0);
        reduce(&mut state, Action::AlphaDown);
        assert_eq!(state.selected_module, 1);
    }

    #[test]
    fn when_at_the_bottom_of_modules_alpha_down_goes_to_start() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        state.selected_module = 1;

        let node1_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node1_handle,
            name: "",
            index: 1,
            attributes: vec![],
            selected_attribute: 0,
        });

        let node2_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node2_handle,
            name: "",
            index: 2,
            attributes: vec![],
            selected_attribute: 0,
        });

        reduce(&mut state, Action::AlphaDown);
        assert_eq!(state.selected_module, 0);
    }

    #[test]
    fn alpha_click_toggles_between_modules_and_patches() {
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        assert!(matches!(state.view, View::Modules));
        reduce(&mut state, Action::AlphaClick);
        assert!(matches!(state.view, View::Patches));
        reduce(&mut state, Action::AlphaClick);
        assert!(matches!(state.view, View::Modules));
    }
}

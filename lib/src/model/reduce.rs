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
    match state.view {
        View::Modules => {
            if state.modules.is_empty() {
                return;
            }
            state.selected_module = ((state.selected_module as i32 - 1)
                .rem_euclid(state.modules.len() as i32))
                as usize;
        }
        View::Patches => {
            if state.patches.is_empty() {
                return;
            }
            state.selected_patch =
                ((state.selected_patch as i32 - 1).rem_euclid(state.patches.len() as i32)) as usize;
        }
    }
}

fn reduce_alpha_down<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    match state.view {
        View::Modules => {
            if state.modules.is_empty() {
                return;
            }
            state.selected_module += 1;
            state.selected_module %= state.modules.len();
        }
        View::Patches => {
            if state.patches.is_empty() {
                return;
            }
            state.selected_patch += 1;
            state.selected_patch %= state.patches.len();
        }
    }
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

    use crate::graphity::NodeIndex;
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

    fn add_empty_module(
        graph: &mut TestGraph,
        state: &mut State<__NodeIndex, __ConsumerIndex, __ProducerIndex>,
    ) {
        let node_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node_handle,
            name: "",
            index: 1,
            attributes: vec![],
            selected_attribute: 0,
        });
    }

    fn add_two_modules_and_patch(
        graph: &mut TestGraph,
        state: &mut State<__NodeIndex, __ConsumerIndex, __ProducerIndex>,
    ) {
        let node1_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node1_handle,
            name: "",
            index: 1,
            attributes: vec![Attribute {
                socket: Socket::Producer(node1_handle.producer(TestProducer)),
                name: "",
                connected: false,
            }],
            selected_attribute: 0,
        });

        let node2_handle = graph.add_node(TestNode);
        state.modules.push(Module {
            handle: node2_handle,
            name: "",
            index: 1,
            attributes: vec![Attribute {
                socket: Socket::Consumer(node2_handle.consumer(TestConsumer)),
                name: "",
                connected: false,
            }],
            selected_attribute: 0,
        });

        graph.must_add_edge(
            node1_handle.producer(TestProducer),
            node2_handle.consumer(TestConsumer),
        );
        state.patches.push(Patch {
            source: Some(Source {
                producer: node1_handle.producer(TestProducer),
                module_name: "",
                module_index: 0,
                attribute_name: "",
            }),
            destination: Some(Destination {
                consumer: node2_handle.consumer(TestConsumer),
                module_name: "",
                module_index: 0,
                attribute_name: "",
            }),
        });
    }

    #[test]
    fn when_clicked_alpha_toggles_between_modules_and_patches() {
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        assert!(matches!(state.view, View::Modules));
        reduce(&mut state, Action::AlphaClick);
        assert!(matches!(state.view, View::Patches));
        reduce(&mut state, Action::AlphaClick);
        assert!(matches!(state.view, View::Modules));
    }

    #[test]
    fn when_modules_are_empty_alpha_up_does_nothing() {
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        let original_selected_module = state.selected_module;

        reduce(&mut state, Action::AlphaUp);
        assert!(state.selected_module == original_selected_module);
    }

    #[test]
    fn when_at_the_top_of_modules_alpha_up_moves_to_last() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        add_empty_module(&mut graph, &mut state);
        add_empty_module(&mut graph, &mut state);

        assert_eq!(state.selected_module, 0);
        reduce(&mut state, Action::AlphaUp);
        assert_eq!(state.selected_module, 1);
    }

    #[test]
    fn when_at_the_bottom_of_modules_alpha_up_goes_to_previous() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        add_empty_module(&mut graph, &mut state);
        add_empty_module(&mut graph, &mut state);
        state.selected_module = 1;

        reduce(&mut state, Action::AlphaUp);
        assert_eq!(state.selected_module, 0);
    }

    #[test]
    fn when_modules_are_empty_alpha_down_does_nothing() {
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        let original_selected_module = state.selected_module;

        reduce(&mut state, Action::AlphaDown);
        assert!(state.selected_module == original_selected_module);
    }

    #[test]
    fn when_at_the_top_of_modules_alpha_down_moves_to_next() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        add_empty_module(&mut graph, &mut state);
        add_empty_module(&mut graph, &mut state);

        assert_eq!(state.selected_module, 0);
        reduce(&mut state, Action::AlphaDown);
        assert_eq!(state.selected_module, 1);
    }

    #[test]
    fn when_at_the_bottom_of_modules_alpha_down_goes_to_start() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        add_empty_module(&mut graph, &mut state);
        add_empty_module(&mut graph, &mut state);
        state.selected_module = 1;

        reduce(&mut state, Action::AlphaDown);
        assert_eq!(state.selected_module, 0);
    }

    #[test]
    fn when_patches_are_empty_alpha_up_does_nothing() {
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        state.view = View::Patches;
        let original_selected_patch = state.selected_patch;

        reduce(&mut state, Action::AlphaUp);
        assert!(state.selected_patch == original_selected_patch);
    }

    #[test]
    fn when_at_the_top_of_patches_alpha_up_moves_to_last() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        state.view = View::Patches;
        add_two_modules_and_patch(&mut graph, &mut state);
        add_two_modules_and_patch(&mut graph, &mut state);

        assert_eq!(state.selected_patch, 0);
        reduce(&mut state, Action::AlphaUp);
        assert_eq!(state.selected_patch, 1);
    }

    #[test]
    fn when_at_the_bottom_of_patches_alpha_up_goes_to_previous() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        state.view = View::Patches;
        add_two_modules_and_patch(&mut graph, &mut state);
        add_two_modules_and_patch(&mut graph, &mut state);
        state.selected_patch = 1;

        reduce(&mut state, Action::AlphaUp);
        assert_eq!(state.selected_patch, 0);
    }

    #[test]
    fn when_patches_are_empty_alpha_down_does_nothing() {
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        state.view = View::Patches;
        let original_selected_patch = state.selected_patch;

        reduce(&mut state, Action::AlphaDown);
        assert!(state.selected_patch == original_selected_patch);
    }

    #[test]
    fn when_at_the_top_of_patches_alpha_down_moves_to_next() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        state.view = View::Patches;
        add_two_modules_and_patch(&mut graph, &mut state);
        add_two_modules_and_patch(&mut graph, &mut state);

        assert_eq!(state.selected_patch, 0);
        reduce(&mut state, Action::AlphaDown);
        assert_eq!(state.selected_patch, 1);
    }

    #[test]
    fn when_at_the_bottom_of_patches_alpha_down_goes_to_start() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        state.view = View::Patches;
        add_two_modules_and_patch(&mut graph, &mut state);
        add_two_modules_and_patch(&mut graph, &mut state);
        state.selected_patch = 1;

        reduce(&mut state, Action::AlphaDown);
        assert_eq!(state.selected_patch, 0);
    }

    #[test]
    fn when_turned_up_alpha_scrolls_only_selected_view() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        add_two_modules_and_patch(&mut graph, &mut state);
        add_two_modules_and_patch(&mut graph, &mut state);

        state.view = View::Modules;
        let original_selected_patch = state.selected_patch;
        reduce(&mut state, Action::AlphaUp);
        assert_eq!(state.selected_patch, original_selected_patch);

        state.view = View::Patches;
        let original_selected_module = state.selected_module;
        reduce(&mut state, Action::AlphaUp);
        assert_eq!(state.selected_module, original_selected_module);
    }

    #[test]
    fn when_turned_down_alpha_scrolls_only_selected_view() {
        let mut graph = TestGraph::new();
        let mut state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
        add_two_modules_and_patch(&mut graph, &mut state);
        add_two_modules_and_patch(&mut graph, &mut state);

        state.view = View::Modules;
        let original_selected_patch = state.selected_patch;
        reduce(&mut state, Action::AlphaDown);
        assert_eq!(state.selected_patch, original_selected_patch);

        state.view = View::Patches;
        let original_selected_module = state.selected_module;
        reduce(&mut state, Action::AlphaDown);
        assert_eq!(state.selected_module, original_selected_module);
    }
}
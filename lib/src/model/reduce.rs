use core::cmp::PartialEq;

use super::action::Action;
use super::reaction::Reaction;
use super::state::*;

pub fn reduce<NI, CI, PI>(state: &mut State<NI, CI, PI>, action: Action) -> Option<Reaction<PI, CI>>
where
    CI: PartialEq + Copy,
    PI: PartialEq + Copy,
{
    match action {
        Action::AlphaUp => reduce_alpha_up(state),
        Action::AlphaDown => reduce_alpha_down(state),
        Action::AlphaClick => reduce_alpha_click(state),
        Action::AlphaHold => reduce_alpha_hold(state),
    }
}

fn reduce_alpha_up<NI, CI, PI>(state: &mut State<NI, CI, PI>) -> Option<Reaction<PI, CI>> {
    match state.view {
        View::Modules => {
            if state.modules.is_empty() {
                return None;
            }
            state.selected_module = ((state.selected_module as i32 - 1)
                .rem_euclid(state.modules.len() as i32))
                as usize;
        }
        View::Patches => {
            if state.patches.is_empty() {
                return None;
            }
            state.selected_patch =
                ((state.selected_patch as i32 - 1).rem_euclid(state.patches.len() as i32)) as usize;
        }
    }

    None
}

fn reduce_alpha_down<NI, CI, PI>(state: &mut State<NI, CI, PI>) -> Option<Reaction<PI, CI>> {
    match state.view {
        View::Modules => {
            if state.modules.is_empty() {
                return None;
            }
            state.selected_module += 1;
            state.selected_module %= state.modules.len();
        }
        View::Patches => {
            if state.patches.is_empty() {
                return None;
            }
            state.selected_patch += 1;
            state.selected_patch %= state.patches.len();
        }
    }

    None
}

fn reduce_alpha_click<NI, CI, PI>(state: &mut State<NI, CI, PI>) -> Option<Reaction<PI, CI>> {
    state.view = match state.view {
        View::Modules => View::Patches,
        View::Patches => View::Modules,
    };

    None
}

// TODO: Return graph action
fn reduce_alpha_hold<NI, CI, PI>(state: &mut State<NI, CI, PI>) -> Option<Reaction<PI, CI>>
where
    CI: PartialEq + Copy,
    PI: PartialEq + Copy,
{
    match state.view {
        View::Modules => {
            todo!();
        }
        View::Patches => {
            if state.patches.is_empty() {
                return None;
            }

            let patch = &mut state.patches[state.selected_patch];
            #[allow(clippy::question_mark)]
            if patch.source.is_none() {
                return None;
            }

            let patch_producer = patch.source.as_ref().unwrap().producer;
            let patch_consumer = patch.destination.consumer;

            patch.source = None;

            if find_connected_patch_with_producer(&state.patches, patch_producer).is_none() {
                find_attribute_with_producer(&mut state.modules, patch_producer)
                    .unwrap()
                    .connected = false;
            }

            find_attribute_with_consumer(&mut state.modules, patch_consumer)
                .unwrap()
                .connected = false;

            Some(Reaction::RemovePatch(patch_producer, patch_consumer))
        }
    }
}

fn find_attribute_with_consumer<NI, CI, PI>(
    modules: &mut [Module<NI, CI, PI>],
    seeked_consumer: CI,
) -> Option<&mut Attribute<CI, PI>>
where
    CI: PartialEq + Copy,
{
    modules
        .iter_mut()
        .flat_map(|m| m.attributes.iter_mut())
        .find(|a| {
            if let Socket::Consumer(attribute_consumer) = a.socket {
                attribute_consumer == seeked_consumer
            } else {
                false
            }
        })
}

fn find_attribute_with_producer<NI, CI, PI>(
    modules: &mut [Module<NI, CI, PI>],
    seeked_producer: PI,
) -> Option<&mut Attribute<CI, PI>>
where
    PI: PartialEq + Copy,
{
    modules
        .iter_mut()
        .flat_map(|m| m.attributes.iter_mut())
        .find(|a| {
            if let Socket::Producer(attribute_producer) = a.socket {
                attribute_producer == seeked_producer
            } else {
                false
            }
        })
}

fn find_connected_patch_with_producer<CI, PI>(
    patches: &[Patch<CI, PI>],
    seeked_producer: PI,
) -> Option<&Patch<CI, PI>>
where
    PI: PartialEq + Copy,
{
    patches.iter().find(|p| {
        if let Some(source) = &p.source {
            source.producer == seeked_producer
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
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

    struct TestContext {
        graph: TestGraph,
        pub state: State<__NodeIndex, __ConsumerIndex, __ProducerIndex>,
    }

    impl TestContext {
        fn new() -> Self {
            let graph = TestGraph::new();
            let state = State::<__NodeIndex, __ConsumerIndex, __ProducerIndex>::default();
            Self { graph, state }
        }

        fn with_two_modules(mut self) -> Self {
            self.add_source_module();
            self.add_destination_module();
            self
        }

        fn with_one_patch(mut self) -> Self {
            let source = self.add_source_module();
            let destination = self.add_destination_module();
            self.add_patch(source, destination);
            self
        }

        fn with_two_patches(mut self) -> Self {
            let source = self.add_source_module();
            let destination1 = self.add_destination_module();
            let destination2 = self.add_destination_module();
            self.add_patch(source, destination1);
            self.add_patch(source, destination2);
            self
        }

        fn add_source_module(&mut self) -> __NodeIndex {
            let node_handle = self.graph.add_node(TestNode);
            self.state
                .modules
                .push(Module::new_for_node(node_handle).with_attribute(
                    Attribute::new_for_producer(node_handle.producer(TestProducer)),
                ));
            node_handle
        }

        fn add_destination_module(&mut self) -> __NodeIndex {
            let node_handle = self.graph.add_node(TestNode);
            self.state
                .modules
                .push(Module::new_for_node(node_handle).with_attribute(
                    Attribute::new_for_consumer(node_handle.consumer(TestConsumer)),
                ));
            self.state
                .patches
                .push(Patch::new_for_consumer(node_handle.consumer(TestConsumer)));
            node_handle
        }

        fn find_module_mut(
            &mut self,
            handle: __NodeIndex,
        ) -> Option<&mut Module<__NodeIndex, __ConsumerIndex, __ProducerIndex>> {
            self.state.modules.iter_mut().find(|m| m.handle == handle)
        }

        fn find_patch_mut(
            &mut self,
            consumer: __ConsumerIndex,
        ) -> Option<&mut Patch<__ConsumerIndex, __ProducerIndex>> {
            self.state
                .patches
                .iter_mut()
                .find(|p| p.destination.consumer == consumer)
        }

        fn add_patch(&mut self, source: __NodeIndex, destination: __NodeIndex) {
            self.graph.must_add_edge(
                source.producer(TestProducer),
                destination.consumer(TestConsumer),
            );
            self.find_patch_mut(destination.consumer(TestConsumer))
                .unwrap()
                .source = Some(Source {
                producer: source.producer(TestProducer),
                module_name: "",
                module_index: 0,
                attribute_name: "",
            });
            self.find_module_mut(source).unwrap().attributes[0].connected = true;
            self.find_module_mut(destination).unwrap().attributes[0].connected = true;
        }
    }

    #[test]
    fn when_clicked_alpha_toggles_between_modules_and_patches() {
        let mut context = TestContext::new();
        assert!(matches!(context.state.view, View::Modules));

        reduce(&mut context.state, Action::AlphaClick);
        assert!(matches!(context.state.view, View::Patches));

        reduce(&mut context.state, Action::AlphaClick);
        assert!(matches!(context.state.view, View::Modules));
    }

    #[test]
    fn when_modules_are_empty_alpha_up_does_nothing() {
        let mut context = TestContext::new();
        let original_selected_module = context.state.selected_module;

        reduce(&mut context.state, Action::AlphaUp);
        assert!(context.state.selected_module == original_selected_module);
    }

    #[test]
    fn when_at_the_top_of_modules_alpha_up_moves_to_last() {
        let mut context = TestContext::new().with_two_modules();
        assert_eq!(context.state.selected_module, 0);
        reduce(&mut context.state, Action::AlphaUp);
        assert_eq!(context.state.selected_module, 1);
    }

    #[test]
    fn when_at_the_bottom_of_modules_alpha_up_goes_to_previous() {
        let mut context = TestContext::new().with_two_modules();
        context.state.selected_module = 1;

        reduce(&mut context.state, Action::AlphaUp);
        assert_eq!(context.state.selected_module, 0);
    }

    #[test]
    fn when_modules_are_empty_alpha_down_does_nothing() {
        let mut context = TestContext::new();
        let original_selected_module = context.state.selected_module;

        reduce(&mut context.state, Action::AlphaDown);
        assert!(context.state.selected_module == original_selected_module);
    }

    #[test]
    fn when_at_the_top_of_modules_alpha_down_moves_to_next() {
        let mut context = TestContext::new().with_two_modules();
        assert_eq!(context.state.selected_module, 0);
        reduce(&mut context.state, Action::AlphaDown);
        assert_eq!(context.state.selected_module, 1);
    }

    #[test]
    fn when_at_the_bottom_of_modules_alpha_down_goes_to_start() {
        let mut context = TestContext::new().with_two_modules();
        context.state.selected_module = 1;

        reduce(&mut context.state, Action::AlphaDown);
        assert_eq!(context.state.selected_module, 0);
    }

    #[test]
    fn when_patches_are_empty_alpha_up_does_nothing() {
        let mut context = TestContext::new();
        context.state.view = View::Patches;
        let original_selected_patch = context.state.selected_patch;

        reduce(&mut context.state, Action::AlphaUp);
        assert!(context.state.selected_patch == original_selected_patch);
    }

    #[test]
    fn when_at_the_top_of_patches_alpha_up_moves_to_last() {
        let mut context = TestContext::new().with_two_patches();
        context.state.view = View::Patches;

        assert_eq!(context.state.selected_patch, 0);
        reduce(&mut context.state, Action::AlphaUp);
        assert_eq!(context.state.selected_patch, 1);
    }

    #[test]
    fn when_at_the_bottom_of_patches_alpha_up_goes_to_previous() {
        let mut context = TestContext::new().with_two_patches();
        context.state.view = View::Patches;
        context.state.selected_patch = 1;

        reduce(&mut context.state, Action::AlphaUp);
        assert_eq!(context.state.selected_patch, 0);
    }

    #[test]
    fn when_patches_are_empty_alpha_down_does_nothing() {
        let mut context = TestContext::new();
        context.state.view = View::Patches;
        let original_selected_patch = context.state.selected_patch;

        reduce(&mut context.state, Action::AlphaDown);
        assert!(context.state.selected_patch == original_selected_patch);
    }

    #[test]
    fn when_at_the_top_of_patches_alpha_down_moves_to_next() {
        let mut context = TestContext::new().with_two_patches();
        context.state.view = View::Patches;

        assert_eq!(context.state.selected_patch, 0);
        reduce(&mut context.state, Action::AlphaDown);
        assert_eq!(context.state.selected_patch, 1);
    }

    #[test]
    fn when_at_the_bottom_of_patches_alpha_down_goes_to_start() {
        let mut context = TestContext::new().with_two_patches();
        context.state.view = View::Patches;
        context.state.selected_patch = 1;

        reduce(&mut context.state, Action::AlphaDown);
        assert_eq!(context.state.selected_patch, 0);
    }

    #[test]
    fn when_turned_up_alpha_scrolls_only_selected_view() {
        let mut context = TestContext::new().with_two_patches();

        context.state.view = View::Modules;
        let original_selected_patch = context.state.selected_patch;
        reduce(&mut context.state, Action::AlphaUp);
        assert_eq!(context.state.selected_patch, original_selected_patch);

        context.state.view = View::Patches;
        let original_selected_module = context.state.selected_module;
        reduce(&mut context.state, Action::AlphaUp);
        assert_eq!(context.state.selected_module, original_selected_module);
    }

    #[test]
    fn when_turned_down_alpha_scrolls_only_selected_view() {
        let mut context = TestContext::new().with_two_patches();

        context.state.view = View::Modules;
        let original_selected_patch = context.state.selected_patch;
        reduce(&mut context.state, Action::AlphaDown);
        assert_eq!(context.state.selected_patch, original_selected_patch);

        context.state.view = View::Patches;
        let original_selected_module = context.state.selected_module;
        reduce(&mut context.state, Action::AlphaDown);
        assert_eq!(context.state.selected_module, original_selected_module);
    }

    #[test]
    fn when_holding_alpha_on_connected_patch_it_removes_source() {
        let mut context = TestContext::new().with_one_patch();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        assert!(context.state.patches[0].source.is_some());
        reduce(&mut context.state, Action::AlphaHold);
        assert!(context.state.patches[0].source.is_none());
    }

    #[test]
    fn when_holding_alpha_on_connected_patch_it_responses_with_delete_patch_reaction() {
        let mut context = TestContext::new().with_one_patch();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        let producer = context.state.patches[0].source.as_ref().unwrap().producer;
        let consumer = context.state.patches[0].destination.consumer;

        let reaction = reduce(&mut context.state, Action::AlphaHold).unwrap();
        assert!(reaction == Reaction::RemovePatch(producer, consumer));
    }

    #[test]
    fn when_holding_alpha_on_connected_patch_it_sets_consumer_disconnected() {
        let mut context = TestContext::new().with_one_patch();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        assert!(context.state.modules[1].attributes[0].connected);
        reduce(&mut context.state, Action::AlphaHold);
        assert!(!context.state.modules[1].attributes[0].connected);
    }

    #[test]
    fn when_holding_alpha_on_disconnected_patch_it_does_nothing() {
        let mut context = TestContext::new().with_two_modules();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        let original_state = context.state.clone();
        reduce(&mut context.state, Action::AlphaDown);
        assert!(context.state == original_state);
    }

    #[test]
    fn when_holding_alpha_on_one_of_many_patches_of_producer_it_keeps_producer_connected() {
        let mut context = TestContext::new().with_two_patches();
        context.state.view = View::Patches;

        assert!(context.state.modules[0].attributes[0].connected);

        context.state.selected_patch = 0;
        reduce(&mut context.state, Action::AlphaHold);
        assert!(context.state.modules[0].attributes[0].connected);

        context.state.selected_patch = 1;
        reduce(&mut context.state, Action::AlphaHold);
        assert!(!context.state.modules[0].attributes[0].connected);
    }

    #[test]
    fn when_holding_alpha_on_the_only_patch_of_producer_it_sets_producer_disconnected() {
        let mut context = TestContext::new().with_one_patch();

        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        assert!(context.state.modules[0].attributes[0].connected);
        reduce(&mut context.state, Action::AlphaHold);
        assert!(!context.state.modules[0].attributes[0].connected);
    }
}

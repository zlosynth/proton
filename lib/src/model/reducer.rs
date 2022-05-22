use alloc::vec;
use alloc::vec::Vec;
use core::cmp::PartialEq;

use hashbrown::HashSet;

use super::action::Action;
use super::state::*;

pub type Registrator<N, NI, CI, PI> = fn(
    &'static str,
    &mut graphity::signal::SignalGraph<N, NI, CI, PI>,
    &mut Vec<Module<NI, CI, PI>>,
);

pub fn reduce<N, NI, C, CI, P, PI>(
    registrator: Registrator<N, NI, CI, PI>,
    graph: &mut graphity::signal::SignalGraph<N, NI, CI, PI>,
    state: &mut State<NI, CI, PI>,
    action: Action,
) where
    N: graphity::NodeWrapper<Class = NI::Class, Consumer = NI::Consumer, Producer = NI::Producer>,
    NI: graphity::NodeIndex<ProducerIndex = PI, ConsumerIndex = CI>,
    CI: graphity::node::ConsumerIndex<NodeIndex = NI, Consumer = NI::Consumer>,
    PI: graphity::node::ProducerIndex<NodeIndex = NI, Producer = NI::Producer>,
    N: From<N>,
    <NI as graphity::NodeIndex>::Consumer: From<C>,
    <NI as graphity::NodeIndex>::Producer: From<P>,
{
    match state.view {
        View::Modules => match action {
            Action::AlphaUp => select_previous_module(state),
            Action::AlphaDown => select_next_module(state),
            Action::AlphaClick => switch_to_patches(state),
            Action::AlphaHold => remove_module(graph, state),
            Action::BetaUp => todo!(),
            Action::BetaDown => todo!(),
            Action::BetaClick => todo!(),
            Action::BetaHold => todo!(),
            Action::BothClick => switch_to_module_add(state),
        },
        View::ModuleAdd => match action {
            Action::AlphaUp => select_previous_class(state),
            Action::AlphaDown => select_next_class(state),
            Action::AlphaClick => instantiate_selected_class(registrator, graph, state),
            Action::AlphaHold => switch_to_modules(state),
            Action::BetaUp => select_previous_class(state),
            Action::BetaDown => select_next_class(state),
            Action::BetaClick => instantiate_selected_class(registrator, graph, state),
            Action::BetaHold => switch_to_modules(state),
            Action::BothClick => (),
        },
        View::Patches => match action {
            Action::AlphaUp => select_previous_patch(state),
            Action::AlphaDown => select_next_patch(state),
            Action::AlphaClick => switch_to_modules(state),
            Action::AlphaHold => (),
            Action::BetaUp => select_previous_patch(state),
            Action::BetaDown => select_next_patch(state),
            Action::BetaClick => enter_patch_edit(state),
            Action::BetaHold => disconnect_source(graph, state),
            Action::BothClick => (),
        },
        View::PatchEdit => match action {
            Action::AlphaUp => exit_patch_edit(state),
            Action::AlphaDown => exit_patch_edit(state),
            Action::AlphaClick => exit_patch_edit(state),
            Action::AlphaHold => exit_patch_edit(state),
            Action::BetaUp => select_previous_source(state),
            Action::BetaDown => select_next_source(state),
            Action::BetaClick => connect_selected_source(graph, state),
            Action::BetaHold => connect_selected_source(graph, state),
            Action::BothClick => (),
        },
    }
}

fn switch_to_modules<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    state.view = View::Modules;
}

fn switch_to_patches<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    state.view = View::Patches;
}

fn switch_to_module_add<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    state.view = View::ModuleAdd;
}

fn select_previous_module<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    if state.modules.is_empty() {
        return;
    }

    state.selected_module =
        ((state.selected_module as i32 - 1).rem_euclid(state.modules.len() as i32)) as usize;
}

fn select_next_module<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    if state.modules.is_empty() {
        return;
    }

    state.selected_module += 1;
    state.selected_module %= state.modules.len();
}

fn select_previous_patch<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    if state.patches.is_empty() {
        return;
    }

    state.selected_patch =
        ((state.selected_patch as i32 - 1).rem_euclid(state.patches.len() as i32)) as usize;
}

fn select_next_patch<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    if state.patches.is_empty() {
        return;
    }

    state.selected_patch += 1;
    state.selected_patch %= state.patches.len();
}

fn select_previous_class<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    if state.classes.is_empty() {
        return;
    }

    state.selected_class =
        ((state.selected_class as i32 - 1).rem_euclid(state.classes.len() as i32)) as usize;
}

fn select_next_class<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    if state.classes.is_empty() {
        return;
    }

    state.selected_class += 1;
    state.selected_class %= state.classes.len();
}

fn instantiate_selected_class<N, NI, C, CI, P, PI>(
    registrator: Registrator<N, NI, CI, PI>,
    graph: &mut graphity::signal::SignalGraph<N, NI, CI, PI>,
    state: &mut State<NI, CI, PI>,
) where
    N: graphity::NodeWrapper<Class = NI::Class, Consumer = NI::Consumer, Producer = NI::Producer>,
    NI: graphity::NodeIndex<ConsumerIndex = CI, ProducerIndex = PI>,
    CI: graphity::node::ConsumerIndex<NodeIndex = NI, Consumer = NI::Consumer>,
    PI: graphity::node::ProducerIndex<NodeIndex = NI, Producer = NI::Producer>,
    N: From<N>,
    <NI as graphity::NodeIndex>::Consumer: From<C>,
    <NI as graphity::NodeIndex>::Producer: From<P>,
{
    if state.classes.is_empty() {
        return;
    }

    let class = &mut state.classes[state.selected_class];
    let original_modules_len = state.modules.len();
    registrator(class.name, graph, &mut state.modules);
    debug_assert_eq!(
        state.modules.len(),
        original_modules_len + 1,
        "Registrator must add a single module"
    );

    sync_last_module(state);

    state.view = View::Modules;
    state.selected_module = state.modules.len() - 1;
}

pub fn sync_last_module<NI, CI, PI>(state: &mut State<NI, CI, PI>)
where
    NI: graphity::NodeIndex<ConsumerIndex = CI, ProducerIndex = PI>,
    CI: graphity::node::ConsumerIndex<NodeIndex = NI, Consumer = NI::Consumer>,
    PI: graphity::node::ProducerIndex<NodeIndex = NI, Producer = NI::Producer>,
{
    let module = &state.modules.last().unwrap();

    for attribute in module.attributes.iter() {
        if let Socket::Consumer(consumer) = attribute.socket {
            state.patches.push(Patch {
                source: None,
                destination: Destination {
                    consumer,
                    module_name: module.name,
                    module_index: module.index,
                    attribute_name: attribute.name,
                },
            });
        }
    }

    let index = {
        let used_indices: HashSet<usize> = state.modules[..state.modules.len() - 1]
            .iter()
            .filter_map(|m| {
                if m.name == module.name {
                    Some(m.index)
                } else {
                    None
                }
            })
            .collect();

        if used_indices.is_empty() {
            1
        } else {
            let max_used_index = used_indices.iter().fold(0, |a, b| usize::max(a, *b));
            if max_used_index == 99 {
                let possible_indices: HashSet<usize> = (1..=99).collect();
                let available_indices = possible_indices.difference(&used_indices);
                *available_indices.min().unwrap()
            } else {
                max_used_index + 1
            }
        }
    };

    let module = &mut state.modules.last_mut().unwrap();
    module.index = index;
}

fn remove_module<N, NI, CI, PI>(
    graph: &mut graphity::signal::SignalGraph<N, NI, CI, PI>,
    state: &mut State<NI, CI, PI>,
) where
    N: graphity::NodeWrapper<Class = NI::Class, Consumer = NI::Consumer, Producer = NI::Producer>,
    NI: graphity::NodeIndex<ProducerIndex = PI, ConsumerIndex = CI>,
    CI: graphity::node::ConsumerIndex<NodeIndex = NI, Consumer = NI::Consumer>,
    PI: graphity::node::ProducerIndex<NodeIndex = NI, Producer = NI::Producer>,
{
    if state.modules[state.selected_module].persistent {
        return;
    }

    let module = &state.modules[state.selected_module];
    let removed_consumers: HashSet<_> = module
        .attributes
        .iter()
        .filter_map(|a| {
            if let Socket::Consumer(consumer) = &a.socket {
                Some(*consumer)
            } else {
                None
            }
        })
        .collect();
    state
        .patches
        .retain(|p| !removed_consumers.contains(&p.destination.consumer));

    graph.remove_node(module.handle);
    state.modules.remove(state.selected_module);
    state.selected_module = usize::min(state.selected_module, state.modules.len() - 1);
}

fn exit_patch_edit<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    state.view = state.patch_edit_origin.unwrap();
    state.patch_edit_sources = vec![];
    state.patch_edit_origin = None;
}

fn enter_patch_edit<NI, CI, PI>(state: &mut State<NI, CI, PI>)
where
    PI: PartialEq + Copy,
{
    state.patch_edit_origin = Some(state.view);
    state.view = View::PatchEdit;
    for module in state.modules.iter() {
        for attribute in module.attributes.iter() {
            if let Socket::Producer(producer) = &attribute.socket {
                state.patch_edit_sources.push(Source {
                    producer: *producer,
                    module_name: module.name,
                    module_index: module.index,
                    attribute_name: attribute.name,
                })
            }
        }
    }
    state.patch_edit_selected_source = {
        let selected_patch = &state.patches[state.selected_patch];
        if let Some(source) = &selected_patch.source {
            state
                .patch_edit_sources
                .iter()
                .enumerate()
                .find(|(_i, s)| s.producer == source.producer)
                .unwrap()
                .0
        } else {
            0
        }
    };
}

fn select_next_source<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    if state.patch_edit_sources.is_empty() {
        return;
    }
    state.patch_edit_selected_source += 1;
    state.patch_edit_selected_source %= state.patch_edit_sources.len();
}

fn select_previous_source<NI, CI, PI>(state: &mut State<NI, CI, PI>) {
    if state.patch_edit_sources.is_empty() {
        return;
    }
    state.patch_edit_selected_source = ((state.patch_edit_selected_source as i32 - 1)
        .rem_euclid(state.patch_edit_sources.len() as i32))
        as usize;
}

fn connect_selected_source<N, NI, CI, PI>(
    graph: &mut graphity::signal::SignalGraph<N, NI, CI, PI>,
    state: &mut State<NI, CI, PI>,
) where
    N: graphity::NodeWrapper<Class = NI::Class, Consumer = NI::Consumer, Producer = NI::Producer>,
    NI: graphity::NodeIndex<ProducerIndex = PI, ConsumerIndex = CI>,
    CI: graphity::node::ConsumerIndex<NodeIndex = NI, Consumer = NI::Consumer>,
    PI: graphity::node::ProducerIndex<NodeIndex = NI, Producer = NI::Producer>,
{
    if state.patch_edit_sources.is_empty() {
        return;
    }

    let patch = &state.patches[state.selected_patch];
    let patch_consumer = patch.destination.consumer;
    if let Some(source) = patch.source.as_ref() {
        graph.remove_edge(source.producer, patch_consumer);
    }

    let new_patch_source = state.patch_edit_sources[state.patch_edit_selected_source].clone();
    let patch = &mut state.patches[state.selected_patch];
    patch.source = Some(new_patch_source);

    let patch = &state.patches[state.selected_patch];
    let patch_consumer = patch.destination.consumer;
    let patch_producer = patch.source.as_ref().unwrap().producer;

    state.view = state.patch_edit_origin.unwrap();
    state.patch_edit_sources = vec![];
    state.patch_edit_origin = None;

    graph.must_add_edge(patch_producer, patch_consumer);
}

fn disconnect_source<N, NI, CI, PI>(
    graph: &mut graphity::signal::SignalGraph<N, NI, CI, PI>,
    state: &mut State<NI, CI, PI>,
) where
    N: graphity::NodeWrapper<Class = NI::Class, Consumer = NI::Consumer, Producer = NI::Producer>,
    NI: graphity::NodeIndex<ProducerIndex = PI, ConsumerIndex = CI>,
    CI: graphity::node::ConsumerIndex<NodeIndex = NI, Consumer = NI::Consumer>,
    PI: graphity::node::ProducerIndex<NodeIndex = NI, Producer = NI::Producer>,
{
    if state.patches.is_empty() {
        return;
    }

    let patch = &mut state.patches[state.selected_patch];
    #[allow(clippy::question_mark)]
    if patch.source.is_none() {
        return;
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

    graph.remove_edge(patch_producer, patch_consumer);
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

    pub fn register<N, NI, CI, PI>(
        name: &'static str,
        graph: &mut graphity::signal::SignalGraph<N, NI, CI, PI>,
        modules: &mut Vec<Module<NI, CI, PI>>,
    ) where
        N: graphity::NodeWrapper<
            Class = NI::Class,
            Consumer = NI::Consumer,
            Producer = NI::Producer,
        >,
        NI: graphity::NodeIndex<ConsumerIndex = CI, ProducerIndex = PI>,
        CI: graphity::node::ConsumerIndex<NodeIndex = NI, Consumer = NI::Consumer>,
        PI: graphity::node::ProducerIndex<NodeIndex = NI, Producer = NI::Producer>,
        N: From<__Node>,
        <NI as graphity::NodeIndex>::Consumer: From<__Consumer>,
        <NI as graphity::NodeIndex>::Producer: From<__Producer>,
    {
        let node = graph.add_node::<__Node>(TestNode.into());
        modules.push(Module::new_for_node(node).with_name(name).with_attribute(
            Attribute::new_for_consumer(node.consumer::<__Consumer>(TestConsumer.into())),
        ));
    }

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

        fn with_two_classes(mut self) -> Self {
            self.add_class("CL1");
            self.add_class("CL2");
            self
        }

        fn add_class(&mut self, name: &'static str) {
            self.state.classes.push(Class {
                name: name,
                description: "",
            });
        }

        fn add_source_module(&mut self) -> __NodeIndex {
            let node_handle = self.graph.add_node(TestNode);
            self.state.modules.push(
                Module::new_for_node(node_handle)
                    .with_name("SRC")
                    .with_attribute(Attribute::new_for_producer(
                        node_handle.producer(TestProducer),
                    )),
            );
            node_handle
        }

        fn add_destination_module(&mut self) -> __NodeIndex {
            let node_handle = self.graph.add_node(TestNode);
            self.state.modules.push(
                Module::new_for_node(node_handle)
                    .with_name("DST")
                    .with_attribute(Attribute::new_for_consumer(
                        node_handle.consumer(TestConsumer),
                    )),
            );
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

        fn reduce(&mut self, action: Action) {
            reduce::<__Node, __NodeIndex, __Consumer, __ConsumerIndex, __Producer, __ProducerIndex>(
                register,
                &mut self.graph,
                &mut self.state,
                action,
            );
        }
    }

    #[test]
    fn when_clicked_alpha_toggles_between_modules_and_patches() {
        let mut context = TestContext::new();
        assert!(matches!(context.state.view, View::Modules));

        context.reduce(Action::AlphaClick);
        assert!(matches!(context.state.view, View::Patches));

        context.reduce(Action::AlphaClick);
        assert!(matches!(context.state.view, View::Modules));
    }

    #[test]
    fn when_modules_are_empty_alpha_up_does_nothing() {
        let mut context = TestContext::new();
        let original_selected_module = context.state.selected_module;

        context.reduce(Action::AlphaUp);
        assert!(context.state.selected_module == original_selected_module);
    }

    #[test]
    fn when_at_the_top_of_modules_alpha_up_moves_to_last() {
        let mut context = TestContext::new().with_two_modules();
        assert_eq!(context.state.selected_module, 0);
        context.reduce(Action::AlphaUp);
        assert_eq!(context.state.selected_module, 1);
    }

    #[test]
    fn when_at_the_bottom_of_modules_alpha_up_goes_to_previous() {
        let mut context = TestContext::new().with_two_modules();
        context.state.selected_module = 1;

        context.reduce(Action::AlphaUp);
        assert_eq!(context.state.selected_module, 0);
    }

    #[test]
    fn when_modules_are_empty_alpha_down_does_nothing() {
        let mut context = TestContext::new();
        let original_selected_module = context.state.selected_module;

        context.reduce(Action::AlphaDown);
        assert!(context.state.selected_module == original_selected_module);
    }

    #[test]
    fn when_at_the_top_of_modules_alpha_down_moves_to_next() {
        let mut context = TestContext::new().with_two_modules();
        assert_eq!(context.state.selected_module, 0);
        context.reduce(Action::AlphaDown);
        assert_eq!(context.state.selected_module, 1);
    }

    #[test]
    fn when_at_the_bottom_of_modules_alpha_down_goes_to_start() {
        let mut context = TestContext::new().with_two_modules();
        context.state.selected_module = 1;

        context.reduce(Action::AlphaDown);
        assert_eq!(context.state.selected_module, 0);
    }

    #[test]
    fn when_patches_are_empty_alpha_or_beta_up_does_nothing() {
        for action in [Action::AlphaUp, Action::BetaUp] {
            let mut context = TestContext::new();
            context.state.view = View::Patches;
            let original_selected_patch = context.state.selected_patch;

            context.reduce(action);
            assert!(context.state.selected_patch == original_selected_patch);
        }
    }

    #[test]
    fn when_at_the_top_of_patches_alpha_or_beta_up_moves_to_last() {
        for action in [Action::AlphaUp, Action::BetaUp] {
            let mut context = TestContext::new().with_two_patches();
            context.state.view = View::Patches;

            assert_eq!(context.state.selected_patch, 0);
            context.reduce(action);
            assert_eq!(context.state.selected_patch, 1);
        }
    }

    #[test]
    fn when_at_the_bottom_of_patches_alpha_or_beta_up_goes_to_previous() {
        for action in [Action::AlphaUp, Action::BetaUp] {
            let mut context = TestContext::new().with_two_patches();
            context.state.view = View::Patches;
            context.state.selected_patch = 1;

            context.reduce(action);
            assert_eq!(context.state.selected_patch, 0);
        }
    }

    #[test]
    fn when_patches_are_empty_alpha_or_beta_down_does_nothing() {
        for action in [Action::AlphaDown, Action::BetaDown] {
            let mut context = TestContext::new();
            context.state.view = View::Patches;
            let original_selected_patch = context.state.selected_patch;

            context.reduce(action);
            assert!(context.state.selected_patch == original_selected_patch);
        }
    }

    #[test]
    fn when_at_the_top_of_patches_alpha_or_beta_down_moves_to_next() {
        for action in [Action::AlphaDown, Action::BetaDown] {
            let mut context = TestContext::new().with_two_patches();
            context.state.view = View::Patches;

            assert_eq!(context.state.selected_patch, 0);
            context.reduce(action);
            assert_eq!(context.state.selected_patch, 1);
        }
    }

    #[test]
    fn when_at_the_bottom_of_patches_alpha_or_beta_down_goes_to_start() {
        for action in [Action::AlphaDown, Action::BetaDown] {
            let mut context = TestContext::new().with_two_patches();
            context.state.view = View::Patches;
            context.state.selected_patch = 1;

            context.reduce(action);
            assert_eq!(context.state.selected_patch, 0);
        }
    }

    #[test]
    fn when_turned_up_alpha_scrolls_only_selected_view() {
        let mut context = TestContext::new().with_two_patches();

        context.state.view = View::Modules;
        let original_selected_patch = context.state.selected_patch;
        context.reduce(Action::AlphaUp);
        assert_eq!(context.state.selected_patch, original_selected_patch);

        context.state.view = View::Patches;
        let original_selected_module = context.state.selected_module;
        context.reduce(Action::AlphaUp);
        assert_eq!(context.state.selected_module, original_selected_module);
    }

    #[test]
    fn when_turned_down_alpha_scrolls_only_selected_view() {
        let mut context = TestContext::new().with_two_patches();

        context.state.view = View::Modules;
        let original_selected_patch = context.state.selected_patch;
        context.reduce(Action::AlphaDown);
        assert_eq!(context.state.selected_patch, original_selected_patch);

        context.state.view = View::Patches;
        let original_selected_module = context.state.selected_module;
        context.reduce(Action::AlphaDown);
        assert_eq!(context.state.selected_module, original_selected_module);
    }

    #[test]
    fn when_holding_beta_on_connected_patch_it_removes_source() {
        let mut context = TestContext::new().with_one_patch();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        assert!(context.state.patches[0].source.is_some());
        context.reduce(Action::BetaHold);
        assert!(context.state.patches[0].source.is_none());
    }

    #[test]
    fn when_holding_beta_on_connected_patch_it_responses_with_delete_patch_reaction() {
        let mut context = TestContext::new().with_one_patch();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        let producer = context.state.patches[0].source.as_ref().unwrap().producer;
        let consumer = context.state.patches[0].destination.consumer;

        context.reduce(Action::BetaHold);
        assert!(!context.graph.has_edge(producer, consumer));
    }

    #[test]
    fn when_holding_beta_on_connected_patch_it_sets_consumer_disconnected() {
        let mut context = TestContext::new().with_one_patch();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        assert!(context.state.modules[1].attributes[0].connected);
        context.reduce(Action::BetaHold);
        assert!(!context.state.modules[1].attributes[0].connected);
    }

    #[test]
    fn when_holding_alpha_on_disconnected_patch_it_does_nothing() {
        let mut context = TestContext::new().with_two_modules();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        let original_state = context.state.clone();
        context.reduce(Action::AlphaDown);
        assert!(context.state == original_state);
    }

    #[test]
    fn when_holding_beta_on_one_of_many_patches_of_producer_it_keeps_producer_connected() {
        let mut context = TestContext::new().with_two_patches();
        context.state.view = View::Patches;

        assert!(context.state.modules[0].attributes[0].connected);

        context.state.selected_patch = 0;
        context.reduce(Action::BetaHold);
        assert!(context.state.modules[0].attributes[0].connected);

        context.state.selected_patch = 1;
        context.reduce(Action::BetaHold);
        assert!(!context.state.modules[0].attributes[0].connected);
    }

    #[test]
    fn when_holding_beta_on_the_only_patch_of_producer_it_sets_producer_disconnected() {
        let mut context = TestContext::new().with_one_patch();

        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        assert!(context.state.modules[0].attributes[0].connected);
        context.reduce(Action::BetaHold);
        assert!(!context.state.modules[0].attributes[0].connected);
    }

    #[test]
    fn given_selected_patch_when_click_beta_it_enters_patch_edit() {
        let mut context = TestContext::new().with_one_patch();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        context.reduce(Action::BetaClick);
        assert!(context.state.view == View::PatchEdit);
    }

    #[test]
    fn given_patch_edit_view_when_moves_or_clicks_alpha_it_exits_edit_and_keeps_original_source() {
        for origin in &[View::Modules, View::Patches] {
            for action in &[
                Action::AlphaUp,
                Action::AlphaDown,
                Action::AlphaClick,
                Action::AlphaHold,
            ] {
                let mut context = TestContext::new().with_one_patch();
                context.state.view = View::PatchEdit;
                context.state.patch_edit_origin = Some(*origin);
                context.state.selected_patch = 0;

                context.reduce(*action);
                assert!(context.state.view == *origin);
                assert!(context.state.patch_edit_origin.is_none());
            }
        }
    }

    #[test]
    fn given_patch_edit_view_when_exits_it_clears_origin_list() {
        let mut context = TestContext::new();
        context.add_source_module();
        context.add_source_module();
        context.add_destination_module();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        context.reduce(Action::BetaClick);
        assert!(!context.state.patch_edit_sources.is_empty());

        context.reduce(Action::BetaClick);
        assert!(context.state.patch_edit_sources.is_empty());
    }

    #[test]
    fn given_selected_top_source_in_patch_edit_when_beta_up_it_moves_to_last() {
        let mut context = TestContext::new();
        context.add_source_module();
        context.add_source_module();
        context.add_destination_module();
        context.state.view = View::Patches;
        context.reduce(Action::BetaClick);

        assert_eq!(context.state.patch_edit_selected_source, 0);
        context.reduce(Action::BetaUp);
        assert_eq!(context.state.patch_edit_selected_source, 1);
    }

    #[test]
    fn given_selected_bottom_source_in_patch_edit_when_beta_down_it_moves_to_first() {
        let mut context = TestContext::new();
        context.add_source_module();
        context.add_source_module();
        context.add_destination_module();
        context.state.view = View::Patches;
        context.reduce(Action::BetaClick);
        context.state.patch_edit_selected_source = 1;

        context.reduce(Action::BetaDown);
        assert_eq!(context.state.patch_edit_selected_source, 0);
    }

    #[test]
    fn given_selected_a_source_in_patch_edit_when_beta_down_it_moves_to_next() {
        let mut context = TestContext::new();
        context.add_source_module();
        context.add_source_module();
        context.add_destination_module();
        context.state.view = View::Patches;
        context.reduce(Action::BetaClick);

        assert_eq!(context.state.patch_edit_selected_source, 0);
        context.reduce(Action::BetaDown);
        assert_eq!(context.state.patch_edit_selected_source, 1);
    }

    #[test]
    fn given_selected_a_source_in_patch_edit_when_beta_up_it_moves_to_previous() {
        let mut context = TestContext::new().with_two_patches();
        context.state.view = View::Patches;
        context.reduce(Action::BetaClick);
        context.state.patch_edit_selected_source = 1;

        context.reduce(Action::BetaUp);
        assert_eq!(context.state.patch_edit_selected_source, 0);
    }

    #[test]
    fn given_patch_edit_view_when_clicks_or_holds_beta_it_selects_source_and_exits_edit() {
        for action in [Action::BetaClick, Action::BetaHold] {
            let mut context = TestContext::new().with_two_modules();

            context.state.view = View::Patches;
            context.state.selected_patch = 0;

            context.reduce(Action::BetaClick);
            assert!(context.state.patches[0].source.is_none());
            context.reduce(action);
            assert!(context.state.patches[0].source.is_some());

            let producer = context.state.patches[0].source.as_ref().unwrap().producer;
            let consumer = context.state.patches[0].destination.consumer;
            assert!(context.graph.has_edge(producer, consumer));

            assert!(context.state.view == View::Patches);
        }
    }

    #[test]
    fn when_enters_connected_patch_edit_it_selects_the_current_sources_index() {
        let mut context = TestContext::new();
        let _source1 = context.add_source_module();
        let source2 = context.add_source_module();
        let destination = context.add_destination_module();
        context.add_patch(source2, destination);

        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        context.reduce(Action::BetaClick);
        assert_eq!(context.state.patch_edit_selected_source, 1);
    }

    #[test]
    fn when_enters_disconnected_patch_edit_it_selects_the_first_source() {
        let mut context = TestContext::new();
        let _source1 = context.add_source_module();
        let _source2 = context.add_source_module();
        let _destination = context.add_destination_module();

        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        context.reduce(Action::BetaClick);
        assert_eq!(context.state.patch_edit_selected_source, 0);
    }

    #[test]
    fn when_enters_patch_edit_it_lists_all_available_sources() {
        let mut context = TestContext::new().with_two_patches();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        context.reduce(Action::BetaClick);
        assert!(context.state.view == View::PatchEdit);

        assert_correct_patch_edit_sources(&context.state);
    }

    #[test]
    fn given_patch_edit_when_new_module_is_added_it_extends_list_of_available_sources() {
        let mut context = TestContext::new().with_two_patches();
        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        context.reduce(Action::BetaClick);
        context.reduce(Action::AlphaClick);

        context.add_source_module();
        context.reduce(Action::BetaClick);

        assert_correct_patch_edit_sources(&context.state);
    }

    #[test]
    fn given_patch_connected_to_a_source_when_connecting_to_another_source_it_switches() {
        let mut context = TestContext::new();
        let _source1 = context.add_source_module();
        let _source2 = context.add_source_module();
        let _destination = context.add_destination_module();

        context.state.view = View::Patches;
        context.state.selected_patch = 0;

        context.reduce(Action::BetaClick);
        context.reduce(Action::BetaClick);
        let original_source = context.state.patches[0].source.clone();

        context.reduce(Action::BetaClick);
        context.reduce(Action::BetaDown);
        context.reduce(Action::BetaClick);
        let new_source = context.state.patches[0].source.clone();
        assert!(new_source != original_source);
    }

    fn assert_correct_patch_edit_sources(
        state: &State<__NodeIndex, __ConsumerIndex, __ProducerIndex>,
    ) {
        list_producers(state).enumerate().for_each(|(i, p)| {
            assert!(state.patch_edit_sources[i].producer == p);
        });
        assert_eq!(
            list_producers(state).count(),
            state.patch_edit_sources.len()
        );
    }

    fn list_producers(
        state: &State<__NodeIndex, __ConsumerIndex, __ProducerIndex>,
    ) -> impl Iterator<Item = __ProducerIndex> + '_ {
        state
            .modules
            .iter()
            .flat_map(|m| m.attributes.iter())
            .filter_map(|a| {
                if let Socket::Producer(producer) = a.socket {
                    Some(producer)
                } else {
                    None
                }
            })
    }

    #[test]
    fn given_modules_view_when_holding_alpha_it_switches_to_module_add() {
        let mut context = TestContext::new().with_two_patches();
        context.state.view = View::Modules;

        context.reduce(Action::BothClick);
        assert!(context.state.view == View::ModuleAdd);
    }

    #[test]
    fn given_module_add_view_when_hold_alpha_or_beta_it_exits() {
        for action in [Action::AlphaHold, Action::BetaHold] {
            let mut context = TestContext::new();
            context.state.view = View::ModuleAdd;

            context.reduce(action);
            assert!(context.state.view == View::Modules);
        }
    }

    #[test]
    fn given_module_add_view_on_bottom_when_alpha_or_beta_down_it_moves_to_beginning() {
        for action in [Action::AlphaDown, Action::BetaDown] {
            let mut context = TestContext::new().with_two_classes();
            context.state.view = View::ModuleAdd;

            context.state.selected_class = 1;
            context.reduce(action);
            assert_eq!(context.state.selected_class, 0);
        }
    }

    #[test]
    fn given_module_add_view_on_top_when_alpha_or_beta_down_it_moves_to_next() {
        for action in [Action::AlphaDown, Action::BetaDown] {
            let mut context = TestContext::new().with_two_classes();
            context.state.view = View::ModuleAdd;

            assert_eq!(context.state.selected_class, 0);
            context.reduce(action);
            assert_eq!(context.state.selected_class, 1);
        }
    }

    #[test]
    fn given_module_add_view_on_top_when_alpha_or_beta_up_it_moves_to_last() {
        for action in [Action::AlphaUp, Action::BetaUp] {
            let mut context = TestContext::new().with_two_classes();
            context.state.view = View::ModuleAdd;

            assert_eq!(context.state.selected_class, 0);
            context.reduce(action);
            assert_eq!(context.state.selected_class, 1);
        }
    }

    #[test]
    fn given_module_add_view_on_top_when_alpha_or_beta_up_it_moves_to_previous() {
        for action in [Action::AlphaUp, Action::BetaUp] {
            let mut context = TestContext::new().with_two_classes();
            context.state.view = View::ModuleAdd;

            context.state.selected_class = 1;
            context.reduce(action);
            assert_eq!(context.state.selected_class, 0);
        }
    }

    #[test]
    fn given_module_add_view_when_alpha_or_beta_click_it_instantiates_class_and_selects_it() {
        for action in [Action::AlphaClick, Action::BetaClick] {
            let mut context = TestContext::new().with_two_modules().with_two_classes();
            context.state.view = View::ModuleAdd;
            context.state.selected_class = 0;

            let original_modules_len = context.state.modules.len();
            let original_patches_len = context.state.patches.len();
            context.reduce(action);
            assert_eq!(context.state.modules.len(), original_modules_len + 1);
            assert_eq!(context.state.patches.len(), original_patches_len + 1);
            assert!(context.state.view == View::Modules);
            assert_eq!(
                context.state.selected_module,
                context.state.modules.len() - 1
            );
        }
    }

    #[test]
    fn given_modules_view_selected_regular_module_when_holds_alpha_it_removes_the_module() {
        let mut context = TestContext::new().with_one_patch();
        context.state.view = View::Modules;
        context.state.selected_module = 1;

        let destination_node = context.state.modules[1].handle;
        let consumer = context.state.modules[1].attributes[0].socket.consumer();
        let producer = context.state.modules[0].attributes[0].socket.producer();
        assert!(context.graph.node(&destination_node).is_some());
        assert!(context.graph.has_edge(producer, consumer));

        let original_modules_len = context.state.modules.len();
        let original_patches_len = context.state.patches.len();
        context.reduce(Action::AlphaHold);
        assert_eq!(context.state.modules.len(), original_modules_len - 1);
        assert_eq!(context.state.patches.len(), original_patches_len - 1);

        assert!(context.graph.node(&destination_node).is_none());
        assert!(!context.graph.has_edge(producer, consumer));
    }

    #[test]
    fn given_modules_view_selected_last_module_when_holds_alpha_it_removes_it_and_selects_previous()
    {
        let mut context = TestContext::new().with_two_modules();
        context.state.view = View::Modules;
        context.state.selected_module = context.state.modules.len() - 1;

        context.reduce(Action::AlphaHold);
        assert_eq!(
            context.state.selected_module,
            context.state.modules.len() - 1
        );
    }

    #[test]
    fn given_modules_view_selected_persistent_module_when_holds_alpha_it_does_nothing() {
        let mut context = TestContext::new().with_two_modules();
        context.state.view = View::Modules;
        context.state.modules[0].persistent = true;

        let original_modules_len = context.state.modules.len();
        context.reduce(Action::AlphaHold);
        assert_eq!(context.state.modules.len(), original_modules_len);
    }

    #[test]
    fn given_new_module_added_when_its_first_99_it_gets_increasing_index() {
        let mut context = TestContext::new().with_two_classes();

        for i in 0..98 {
            context.reduce(Action::BothClick);
            context.reduce(Action::AlphaClick);
            let new_module = &context.state.modules[context.state.modules.len() - 1];
            assert_eq!(new_module.index, i + 1);
        }
    }

    #[test]
    fn given_new_modules_from_two_different_classes_are_added_they_dont_share_pool_of_indices() {
        let mut context = TestContext::new().with_two_classes();

        for i in 0..10 {
            context.reduce(Action::BothClick);
            context.state.selected_class = 0;
            context.reduce(Action::AlphaClick);
            let new_module = &context.state.modules[context.state.modules.len() - 1];
            assert_eq!(new_module.index, i + 1);

            context.reduce(Action::BothClick);
            context.state.selected_class = 1;
            context.reduce(Action::AlphaClick);
            let new_module = &context.state.modules[context.state.modules.len() - 1];
            assert_eq!(new_module.index, i + 1);
        }
    }

    #[test]
    fn given_new_module_added_when_its_after_first_99_it_gets_index_of_freed_up_sub_99() {
        let mut context = TestContext::new().with_two_classes();

        for _ in 0..99 {
            context.reduce(Action::BothClick);
            context.reduce(Action::AlphaClick);
        }

        let old_module_12 = &context.state.modules[11];
        assert_eq!(old_module_12.index, 12);

        context.state.selected_module = 11;
        context.reduce(Action::AlphaHold);

        context.reduce(Action::BothClick);
        context.reduce(Action::AlphaClick);

        let new_module_12 = &context.state.modules[context.state.modules.len() - 1];
        assert_eq!(new_module_12.index, 12);
    }
}
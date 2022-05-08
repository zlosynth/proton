use alloc::vec::Vec;

use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::BinaryColor;
use graphity::NodeIndex;

use crate::display::Display;
use crate::model::action::Action;
use crate::model::reducer;
use crate::model::state::{Attribute, Class, Module, State};
use crate::modules::audio_output::*;
use crate::modules::control_input::*;
use crate::modules::mixer::{self, MixerConsumer, MixerNode, MixerProducer};
use crate::modules::oscillator::{self, OscillatorConsumer, OscillatorNode, OscillatorProducer};
use crate::signal::Signal;

graphity!(
    Graph<Signal>;
    ControlInput = {ControlInput, ControlInputConsumer, ControlInputProducer},
    AudioOutput = {AudioOutput, AudioOutputConsumer, AudioOutputProducer},
    OscillatorNode = {OscillatorNode, OscillatorConsumer, OscillatorProducer},
    MixerNode = {MixerNode, MixerConsumer, MixerProducer},
);

pub fn register<N, NI, CI, PI>(
    name: &'static str,
    graph: &mut graphity::signal::SignalGraph<N, NI, CI, PI>,
    modules: &mut Vec<Module<NI, CI, PI>>,
) where
    N: graphity::NodeWrapper<Class = NI::Class, Consumer = NI::Consumer, Producer = NI::Producer>,
    NI: graphity::NodeIndex<ConsumerIndex = CI, ProducerIndex = PI>,
    CI: graphity::node::ConsumerIndex<NodeIndex = NI, Consumer = NI::Consumer>,
    PI: graphity::node::ProducerIndex<NodeIndex = NI, Producer = NI::Producer>,
    N: From<__Node>,
    <NI as graphity::NodeIndex>::Consumer: From<__Consumer>,
    <NI as graphity::NodeIndex>::Producer: From<__Producer>,
{
    if name == "OSC" {
        oscillator::register(graph, modules);
    } else if name == "MIX" {
        mixer::register(graph, modules);
    }
}

pub struct Instrument<D> {
    display: Option<Display<D>>,
    state: State<__NodeIndex, __ConsumerIndex, __ProducerIndex>,

    graph: Graph,
    control1_input_cell: ControlInputCell,
    control2_input_cell: ControlInputCell,
    audio_output_cell: AudioOutputCell,
}

#[allow(clippy::new_without_default)]
impl<D> Instrument<D> {
    pub fn new() -> Self {
        let mut state = State::default();
        let mut graph = Graph::new();

        let (control1_input, control1_input_cell) = ControlInput::new();
        let (control2_input, control2_input_cell) = ControlInput::new();
        let (audio_output, audio_output_cell) = AudioOutput::new();

        // Pretend initialization
        let control_input = graph.add_node(control1_input);
        state.modules.push(
            Module::new_for_node(control_input)
                .with_name(">CV")
                .persistent()
                .with_attribute(Attribute::new_for_producer(
                    control_input.producer(ControlInputProducer),
                )),
        );
        reducer::sync_last_module(&mut state);

        let control_input = graph.add_node(control2_input);
        state.modules.push(
            Module::new_for_node(control_input)
                .with_name(">CV")
                .persistent()
                .with_attribute(Attribute::new_for_producer(
                    control_input.producer(ControlInputProducer),
                )),
        );
        reducer::sync_last_module(&mut state);

        let audio_output = graph.add_node(audio_output);
        state.modules.push(
            Module::new_for_node(audio_output)
                .with_name("<AU")
                .persistent()
                .with_attribute(Attribute::new_for_consumer(
                    audio_output.consumer(AudioOutputConsumer),
                )),
        );
        reducer::sync_last_module(&mut state);

        state.classes.push(Class {
            name: "OSC",
            description: "Basic saw osc-\nillator with \nfrequency con-\ntrol",
        });
        state.classes.push(Class {
            name: "MIX",
            description: "Description",
        });

        Self {
            display: None,
            graph,
            state,
            audio_output_cell,
            control1_input_cell,
            control2_input_cell,
        }
    }

    pub fn set_control1(&mut self, value: f32) {
        self.control1_input_cell.set(value);
    }

    pub fn set_control2(&mut self, value: f32) {
        self.control2_input_cell.set(value);
    }

    pub fn tick(&mut self) {
        self.graph.tick();
    }

    pub fn get_audio(&self) -> [f32; 32] {
        self.audio_output_cell.get()
    }

    pub fn alpha_up(&mut self) {
        self.reduce(Action::AlphaUp);
    }

    pub fn alpha_down(&mut self) {
        self.reduce(Action::AlphaDown);
    }

    pub fn alpha_click(&mut self) {
        self.reduce(Action::AlphaClick);
    }

    pub fn alpha_hold(&mut self) {
        self.reduce(Action::AlphaHold);
    }

    pub fn beta_up(&mut self) {
        self.reduce(Action::BetaUp);
    }

    pub fn beta_down(&mut self) {
        self.reduce(Action::BetaDown);
    }

    pub fn beta_click(&mut self) {
        self.reduce(Action::BetaClick);
    }

    pub fn beta_hold(&mut self) {
        self.reduce(Action::BetaHold);
    }

    pub fn both_click(&mut self) {
        self.reduce(Action::BothClick);
    }

    fn reduce(&mut self, action: Action) {
        reducer::reduce::<
            __Node,
            __NodeIndex,
            __Consumer,
            __ConsumerIndex,
            __Producer,
            __ProducerIndex,
        >(register, &mut self.graph, &mut self.state, action);
    }
}

#[allow(clippy::new_without_default)]
impl<D> Instrument<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    pub fn register_display(&mut self, display: D) {
        self.display = Some(Display::new(display));
    }

    pub fn update_display(&mut self) {
        self.display.as_mut().unwrap().update(&self.state);
    }

    pub fn mut_display(&mut self) -> &mut D {
        &mut self.display.as_mut().unwrap().display
    }
}

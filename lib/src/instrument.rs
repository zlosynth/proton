use alloc::vec;
use alloc::vec::Vec;

use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::BinaryColor;
use graphity::NodeIndex;

use crate::core::signal::Signal;
use crate::display::Display;
use crate::model::action::Action;
use crate::model::reduce::reduce;
use crate::model::state::{Attribute, Class, Destination, Module, Patch, Socket, State};
use crate::modules::audio_output::*;
use crate::modules::control_input::*;
use crate::modules::mixer::{self, MixerConsumer, MixerNode, MixerProducer};
use crate::modules::oscillator::{self, OscillatorConsumer, OscillatorNode, OscillatorProducer};

graphity!(
    Graph<Signal>;
    ControlInput = {ControlInput, ControlInputConsumer, ControlInputProducer},
    AudioOutput = {AudioOutput, AudioOutputConsumer, AudioOutputProducer},
    OscillatorNode = {OscillatorNode, OscillatorConsumer, OscillatorProducer},
    MixerNode = {MixerNode, MixerConsumer, MixerProducer},
);

// TODO: Generate this with a macro
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
        state.modules.push(Module {
            handle: control_input,
            name: ">CV",
            index: 1,
            attributes: vec![Attribute {
                socket: Socket::Producer(control_input.producer(ControlInputProducer)),
                name: "OUT",
                connected: true,
            }],
            selected_attribute: 0,
            persistent: true,
        });
        let control_input = graph.add_node(control2_input);
        state.modules.push(Module {
            handle: control_input,
            name: ">CV",
            index: 2,
            attributes: vec![Attribute {
                socket: Socket::Producer(control_input.producer(ControlInputProducer)),
                name: "OUT",
                connected: true,
            }],
            selected_attribute: 0,
            persistent: true,
        });
        let audio_output = graph.add_node(audio_output);
        state.modules.push(Module {
            handle: audio_output,
            name: "<AU",
            index: 0,
            attributes: vec![Attribute {
                socket: Socket::Consumer(audio_output.consumer(AudioOutputConsumer)),
                name: "IN",
                connected: true,
            }],
            selected_attribute: 0,
            persistent: true,
        });
        state.patches.push(Patch {
            source: None,
            destination: Destination {
                consumer: state.modules[2].attributes[0].socket.consumer(),
                module_name: state.modules[2].name,
                module_index: state.modules[2].index,
                attribute_name: state.modules[2].attributes[0].name,
            },
        });

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
        reduce::<__Node, __NodeIndex, __Consumer, __ConsumerIndex, __Producer, __ProducerIndex>(
            register,
            &mut self.graph,
            &mut self.state,
            action,
        );
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

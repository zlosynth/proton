use alloc::vec;

use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::BinaryColor;
use graphity::NodeIndex;

use crate::core::module::Module as _;
use crate::core::signal::Signal;
use crate::display::Display;
use crate::model::action::Action;
use crate::model::reaction::Reaction;
use crate::model::reduce::reduce;
use crate::model::state::{Attribute, Destination, Module, Patch, Socket, Source, State};
use crate::modules::audio_output::*;
use crate::modules::control_input::*;
use crate::modules::oscillator::*;

graphity!(
    Graph<Signal>;
    ControlInput = {ControlInput, ControlInputConsumer, ControlInputProducer},
    AudioOutput = {AudioOutput, AudioOutputConsumer, AudioOutputProducer},
    OscillatorNode = {OscillatorNode, OscillatorConsumer, OscillatorProducer},
);

pub struct Instrument<D> {
    display: Option<Display<D>>,
    state: State<__NodeIndex, __ConsumerIndex, __ProducerIndex>,

    graph: Graph,
    control_input_cell: ControlInputCell,
    audio_output_cell: AudioOutputCell,
}

#[allow(clippy::new_without_default)]
impl<D> Instrument<D> {
    pub fn new() -> Self {
        let mut state = State::default();
        let mut graph = Graph::new();

        let (control_input, control_input_cell) = ControlInput::new();
        let (audio_output, audio_output_cell) = AudioOutput::new();

        // Pretend initialization
        let control_input = graph.add_node(control_input);
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
        });

        // Pretend store load / user interaction
        let oscillator = Oscillator;
        oscillator.register(&mut graph, &mut state);

        // Pretend store load / user interaction
        graph.must_add_edge(
            state.modules[0].attributes[0].socket.producer(),
            state.modules[2].attributes[0].socket.consumer(),
        );
        state.patches.push(Patch {
            source: Some(Source {
                producer: state.modules[0].attributes[0].socket.producer(),
                module_name: state.modules[0].name,
                module_index: state.modules[0].index,
                attribute_name: state.modules[0].attributes[0].name,
            }),
            destination: Destination {
                consumer: state.modules[2].attributes[0].socket.consumer(),
                module_name: state.modules[2].name,
                module_index: state.modules[2].index,
                attribute_name: state.modules[2].attributes[0].name,
            },
        });

        graph.must_add_edge(
            state.modules[2].attributes[1].socket.producer(),
            state.modules[1].attributes[0].socket.consumer(),
        );
        state.patches.push(Patch {
            source: Some(Source {
                producer: state.modules[2].attributes[1].socket.producer(),
                module_name: state.modules[2].name,
                module_index: state.modules[2].index,
                attribute_name: state.modules[2].attributes[1].name,
            }),
            destination: Destination {
                consumer: state.modules[1].attributes[0].socket.consumer(),
                module_name: state.modules[1].name,
                module_index: state.modules[1].index,
                attribute_name: state.modules[1].attributes[0].name,
            },
        });

        for i in 3..8 {
            let (audio_output, _audio_output_cell) = AudioOutput::new();
            let audio_output = graph.add_node(audio_output);
            state.modules.push(Module {
                handle: audio_output,
                name: "<AU",
                index: i,
                attributes: vec![Attribute {
                    socket: Socket::Consumer(audio_output.consumer(AudioOutputConsumer)),
                    name: "IN",
                    connected: true,
                }],
                selected_attribute: 0,
            });
            state.patches.push(Patch {
                source: None,
                destination: Destination {
                    consumer: state.modules[i].attributes[0].socket.consumer(),
                    module_name: state.modules[i].name,
                    module_index: state.modules[i].index,
                    attribute_name: state.modules[i].attributes[0].name,
                },
            });
        }

        for i in 8..12 {
            let (control_input, _control_input_cell) = ControlInput::new();
            let control_input = graph.add_node(control_input);
            state.modules.push(Module {
                handle: control_input,
                name: ">CV",
                index: i,
                attributes: vec![Attribute {
                    socket: Socket::Producer(control_input.producer(ControlInputProducer)),
                    name: "OUT",
                    connected: true,
                }],
                selected_attribute: 0,
            });
        }

        Self {
            display: None,
            graph,
            state,
            audio_output_cell,
            control_input_cell,
        }
    }

    pub fn set_control(&mut self, value: f32) {
        self.control_input_cell.set(value);
    }

    pub fn tick(&mut self) {
        self.graph.tick();
    }

    pub fn get_audio(&self) -> [f32; 32] {
        self.audio_output_cell.get()
    }

    pub fn alpha_up(&mut self) {
        reduce(&mut self.state, Action::AlphaUp);
    }

    pub fn alpha_down(&mut self) {
        reduce(&mut self.state, Action::AlphaDown);
    }

    pub fn alpha_click(&mut self) {
        reduce(&mut self.state, Action::AlphaClick);
    }

    pub fn alpha_hold(&mut self) {
        reduce(&mut self.state, Action::AlphaHold);
    }

    pub fn beta_up(&mut self) {
        reduce(&mut self.state, Action::BetaUp);
    }

    pub fn beta_down(&mut self) {
        reduce(&mut self.state, Action::BetaDown);
    }

    pub fn beta_click(&mut self) {
        let reaction = reduce(&mut self.state, Action::BetaClick);
        if let Some(Reaction::ConnectPatch(producer, consumer)) = reaction {
            self.graph.must_add_edge(producer, consumer);
        }
    }

    pub fn beta_hold(&mut self) {
        let reaction = reduce(&mut self.state, Action::BetaHold);
        if let Some(Reaction::RemovePatch(producer, consumer)) = reaction {
            self.graph.remove_edge(producer, consumer);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_initialized() {
        let _instrument: Instrument<()> = Instrument::new();
    }

    #[test]
    fn set_arbitrary_control_tick_and_get() {
        let mut instrument: Instrument<()> = Instrument::new();

        instrument.set_control(0.0); // Frequency
        instrument.tick();
        let out = instrument.get_audio();
        for x in out {
            assert_relative_eq!(x, 0.0);
        }

        instrument.set_control(1.0); // Frequency
        instrument.tick();
        let out = instrument.get_audio();
        let average = out.iter().fold(0.0, |a, b| a + b.abs()) / out.len() as f32;
        assert!(average > 0.0);
    }
}

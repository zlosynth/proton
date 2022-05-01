use alloc::vec;

use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::BinaryColor;
use graphity::NodeIndex;

use crate::display::Display;
use crate::model::state::{Attribute, Module, Socket, State};

use crate::core::signal::Signal;
use crate::modules::audio_output::*;
use crate::modules::control_input::*;
use crate::modules::oscillator::*;

graphity!(
    Graph<Signal>;
    ControlInput = {ControlInput, ControlInputConsumer, ControlInputProducer},
    AudioOutput = {AudioOutput, AudioOutputConsumer, AudioOutputProducer},
    Oscillator = {Oscillator, OscillatorConsumer, OscillatorProducer},
);

pub struct Instrument<D> {
    display: Option<Display<D>>,
    state: State,

    graph: Graph,
    control_input_cell: ControlInputCell,
    audio_output_cell: AudioOutputCell,
}

#[allow(clippy::new_without_default)]
impl<D> Instrument<D> {
    pub fn new() -> Self {
        let mut state = State {
            modules: vec![],
            selected_module: 2,
        };

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
                value: "",
                connected: false,
            }],
            selected_attribute: 0,
        });
        let audio_output = graph.add_node(audio_output);
        state.modules.push(Module {
            handle: audio_output,
            name: "<AU",
            index: 1,
            attributes: vec![Attribute {
                socket: Socket::Consumer(audio_output.consumer(AudioOutputConsumer)),
                name: "IN",
                value: "",
                connected: false,
            }],
            selected_attribute: 0,
        });

        // Pretend store load / user interaction
        let oscillator = Oscillator::new();
        let oscillator = graph.add_node(oscillator);
        state.modules.push(Module {
            handle: oscillator,
            name: "OSC",
            index: 1,
            attributes: vec![
                Attribute {
                    socket: Socket::Consumer(oscillator.consumer(OscillatorConsumer::Frequency)),
                    name: "FRQ",
                    value: "16000",
                    connected: false,
                },
                Attribute {
                    socket: Socket::Producer(oscillator.producer(OscillatorProducer)),
                    name: "OUT",
                    value: "",
                    connected: false,
                },
            ],
            selected_attribute: 0,
        });

        // Pretend store load / user interaction
        graph.must_add_edge(
            state.modules[0].attributes[0].socket.producer(),
            state.modules[2].attributes[0].socket.consumer(),
        );
        graph.must_add_edge(
            state.modules[2].attributes[1].socket.producer(),
            state.modules[1].attributes[0].socket.consumer(),
        );

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

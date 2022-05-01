use alloc::vec;

use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::BinaryColor;
use graphity::NodeIndex;

use crate::display::Display;
use crate::model::state::{Attribute, Module, State};

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
        let state = State {
            modules: vec![
                Module {
                    name: "ENV",
                    index: 2,
                    attributes: vec![
                        Attribute {
                            name: "A",
                            connected: true,
                            value: "100% +0.50 ",
                        },
                        Attribute {
                            name: "B",
                            connected: false,
                            value: "0.58       ",
                        },
                    ],
                    selected_attribute: 1,
                },
                Module {
                    name: "MIX",
                    index: 1,
                    attributes: vec![],
                    selected_attribute: 0,
                },
                Module {
                    name: "OSC",
                    index: 3,
                    attributes: vec![],
                    selected_attribute: 0,
                },
                Module {
                    name: ">CV",
                    index: 9,
                    attributes: vec![],
                    selected_attribute: 0,
                },
                Module {
                    name: "<AO",
                    index: 1,
                    attributes: vec![],
                    selected_attribute: 0,
                },
                Module {
                    name: "FOL",
                    index: 3,
                    attributes: vec![],
                    selected_attribute: 0,
                },
                Module {
                    name: "DIS",
                    index: 3,
                    attributes: vec![],
                    selected_attribute: 0,
                },
            ],
            selected_module: 0,
        };

        let mut graph = Graph::new();

        let (control_input, control_input_cell) = ControlInput::new();
        let (audio_output, audio_output_cell) = AudioOutput::new();
        let oscillator = Oscillator::new();

        let control_input = graph.add_node(control_input);
        let audio_output = graph.add_node(audio_output);
        let oscillator = graph.add_node(oscillator);

        graph.must_add_edge(
            control_input.producer(ControlInputProducer),
            oscillator.consumer(OscillatorConsumer::Frequency),
        );
        graph.must_add_edge(
            oscillator.producer(OscillatorProducer),
            audio_output.consumer(AudioOutputConsumer),
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

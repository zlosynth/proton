use alloc::rc::Rc;
use core::cell::RefCell;

use graphity::NodeIndex;

use crate::modules::audio_output::*;
use crate::modules::control_input::*;
use crate::modules::oscillator::*;

graphity!(
    Graph<[f32; 32]>;
    ControlInput = {ControlInput, ControlInputConsumer, ControlInputProducer},
    AudioOutput = {AudioOutput, AudioOutputConsumer, AudioOutputProducer},
    Oscillator = {Oscillator, OscillatorConsumer, OscillatorProducer},
);

pub struct Engine {
    graph: Graph,
    control_input_buffer: Rc<RefCell<f32>>,
    audio_output_buffer: Rc<RefCell<[f32; 32]>>,
}

#[allow(clippy::new_without_default)]
impl Engine {
    pub fn new() -> Self {
        let mut graph = Graph::new();

        let (control_input, control_input_buffer) = ControlInput::new();
        let (audio_output, audio_output_buffer) = AudioOutput::new();
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
            graph,
            control_input_buffer,
            audio_output_buffer,
        }
    }

    pub fn set_control(&mut self, value: f32) {
        *self.control_input_buffer.borrow_mut() = value;
    }

    pub fn tick(&mut self) {
        self.graph.tick();
    }

    pub fn get_audio(&self) -> [f32; 32] {
        *self.audio_output_buffer.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_initialized() {
        let _engine = Engine::new();
    }

    #[test]
    fn when_configured_with_simple_graph_passes_end_to_end() {
        let mut engine = Engine::new();

        engine.set_control(0.0); // Frequency
        engine.tick();
        let out = engine.get_audio();
        for x in out {
            assert_relative_eq!(x, 0.0);
        }

        engine.set_control(1.0); // Frequency
        engine.tick();
        let out = engine.get_audio();
        let average = out.iter().fold(0.0, |a, b| a + b.abs()) / out.len() as f32;
        assert!(average > 0.0);
    }
}

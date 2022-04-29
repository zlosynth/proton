use graphity::Node;

use crate::primitives;

pub struct Oscillator {
    oscillator: primitives::oscillator::Oscillator,
    buffer: [f32; 32],
}

#[allow(clippy::new_without_default)]
impl Oscillator {
    pub fn new() -> Self {
        Self {
            oscillator: primitives::oscillator::Oscillator::new(48_000.0),
            buffer: [0.0; 32],
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum OscillatorConsumer {
    Frequency,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct OscillatorProducer;

impl Node<[f32; 32]> for Oscillator {
    type Consumer = OscillatorConsumer;
    type Producer = OscillatorProducer;

    fn write(&mut self, _consumer: Self::Consumer, input: [f32; 32]) {
        self.oscillator.frequency = input[0];
    }

    fn read(&self, _producer: Self::Producer) -> [f32; 32] {
        self.buffer
    }

    fn tick(&mut self) {
        let buffer = &mut self.buffer;
        self.oscillator.populate(buffer);
    }
}

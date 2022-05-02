use alloc::vec;

use graphity::Node;

use crate::core::signal::Signal;
use crate::model::state::*;
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

impl Node<Signal> for Oscillator {
    type Consumer = OscillatorConsumer;
    type Producer = OscillatorProducer;

    fn write(&mut self, _consumer: Self::Consumer, input: Signal) {
        self.oscillator.frequency = input.as_control();
    }

    fn read(&self, _producer: Self::Producer) -> Signal {
        Signal::from_audio(self.buffer)
    }

    fn tick(&mut self) {
        let buffer = &mut self.buffer;
        self.oscillator.populate(buffer);
    }
}

pub fn new_module<NI, CI, PI>(handle: NI, index: usize) -> Module<NI, CI, PI>
where
    NI: graphity::NodeIndex<ConsumerIndex = CI, ProducerIndex = PI>,
    <NI as graphity::NodeIndex>::Consumer: core::convert::From<OscillatorConsumer>,
    <NI as graphity::NodeIndex>::Producer: core::convert::From<OscillatorProducer>,
{
    Module {
        handle,
        name: "OSC",
        index,
        attributes: vec![
            Attribute {
                socket: Socket::Consumer(handle.consumer(OscillatorConsumer::Frequency)),
                name: "FRQ",
                connected: false,
            },
            Attribute {
                socket: Socket::Producer(handle.producer(OscillatorProducer)),
                name: "OUT",
                connected: false,
            },
        ],
        selected_attribute: 0,
    }
}

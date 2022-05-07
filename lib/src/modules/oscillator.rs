use alloc::vec;
use alloc::vec::Vec;

use graphity::signal::SignalGraph;
use graphity::Node;

use crate::core::signal::Signal;
use crate::model::state::*;
use crate::primitives;

pub struct OscillatorNode {
    oscillator: primitives::oscillator::Oscillator,
    buffer: [f32; 32],
}

#[allow(clippy::new_without_default)]
impl OscillatorNode {
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

impl Node<Signal> for OscillatorNode {
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

pub fn register<N, NI, CI, PI>(
    graph: &mut SignalGraph<N, NI, CI, PI>,
    modules: &mut Vec<Module<NI, CI, PI>>,
) where
    N: graphity::NodeWrapper<Class = NI::Class, Consumer = NI::Consumer, Producer = NI::Producer>,
    NI: graphity::NodeIndex<ConsumerIndex = CI, ProducerIndex = PI>,
    CI: graphity::node::ConsumerIndex<NodeIndex = NI, Consumer = NI::Consumer>,
    PI: graphity::node::ProducerIndex<NodeIndex = NI, Producer = NI::Producer>,
    N: From<crate::instrument::__Node>,
    <NI as graphity::NodeIndex>::Consumer: From<crate::instrument::__Consumer>,
    <NI as graphity::NodeIndex>::Producer: From<crate::instrument::__Producer>,
{
    let oscillator = OscillatorNode::new();

    use crate::instrument::{__Consumer, __Node, __Producer};
    let oscillator = graph.add_node::<__Node>(oscillator.into());
    let consumer_frequency =
        oscillator.consumer::<__Consumer>(OscillatorConsumer::Frequency.into());
    let producer = oscillator.producer::<__Producer>(OscillatorProducer.into());

    modules.push(Module {
        handle: oscillator,
        name: "OSC",
        index: 0,
        attributes: vec![
            Attribute {
                socket: Socket::Consumer(consumer_frequency),
                name: "FRQ",
                connected: false,
            },
            Attribute {
                socket: Socket::Producer(producer),
                name: "OUT",
                connected: false,
            },
        ],
        selected_attribute: 0,
        persistent: false,
    });
}

use alloc::vec;
use alloc::vec::Vec;

use graphity::signal::SignalGraph;
use graphity::Node;

use crate::model::state::*;
use crate::signal::Signal;

pub struct MixerNode {
    in1: [f32; 32],
    in2: [f32; 32],
    out: [f32; 32],
}

#[allow(clippy::new_without_default)]
impl MixerNode {
    pub fn new() -> Self {
        Self {
            in1: [0.0; 32],
            in2: [0.0; 32],
            out: [0.0; 32],
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MixerConsumer {
    In1,
    In2,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct MixerProducer;

impl Node<Signal> for MixerNode {
    type Consumer = MixerConsumer;
    type Producer = MixerProducer;

    fn write(&mut self, consumer: Self::Consumer, input: Signal) {
        match consumer {
            MixerConsumer::In1 => self.in1 = input.as_audio(),
            MixerConsumer::In2 => self.in2 = input.as_audio(),
        }
    }

    fn read(&self, _producer: Self::Producer) -> Signal {
        Signal::from_audio(self.out)
    }

    fn tick(&mut self) {
        self.in1
            .iter()
            .zip(self.in2.iter())
            .enumerate()
            .for_each(|(i, (in1, in2))| self.out[i] = in1 + in2);
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
    let mixer = MixerNode::new();

    use crate::instrument::{__Consumer, __Node, __Producer};
    let mixer = graph.add_node::<__Node>(mixer.into());
    let consumer_in1 = mixer.consumer::<__Consumer>(MixerConsumer::In1.into());
    let consumer_in2 = mixer.consumer::<__Consumer>(MixerConsumer::In2.into());
    let producer = mixer.producer::<__Producer>(MixerProducer.into());

    modules.push(Module {
        handle: mixer,
        name: "MIX",
        index: 0,
        attributes: vec![
            Attribute {
                socket: Socket::Consumer(consumer_in1),
                name: "IN1",
                connected: false,
            },
            Attribute {
                socket: Socket::Consumer(consumer_in2),
                name: "IN2",
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

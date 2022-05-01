use alloc::vec::Vec;

use crate::instrument::__ConsumerIndex as ConsumerIndex;
use crate::instrument::__NodeIndex as NodeIndex;
use crate::instrument::__ProducerIndex as ProducerIndex;

#[derive(Clone, PartialEq)]
pub struct State {
    pub modules: Vec<Module>,
    pub selected_module: usize,
}

#[derive(Clone, PartialEq)]
pub struct Module {
    pub handle: NodeIndex,
    pub name: &'static str,
    pub index: usize,
    pub attributes: Vec<Attribute>,
    pub selected_attribute: usize,
}

#[derive(Clone, PartialEq)]
pub struct Attribute {
    pub socket: Socket,
    pub name: &'static str,
    pub connected: bool,
    pub value: &'static str,
}

#[derive(Clone, PartialEq)]
pub enum Socket {
    Consumer(ConsumerIndex),
    Producer(ProducerIndex),
}

impl Socket {
    pub fn consumer(&self) -> ConsumerIndex {
        if let Socket::Consumer(consumer) = self {
            *consumer
        } else {
            panic!();
        }
    }

    pub fn producer(&self) -> ProducerIndex {
        if let Socket::Producer(producer) = self {
            *producer
        } else {
            panic!();
        }
    }
}

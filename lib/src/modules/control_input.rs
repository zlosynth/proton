use alloc::rc::Rc;
use core::cell::RefCell;

use graphity::Node;

use crate::core::signal::Signal;

pub struct ControlInput {
    buffer: Rc<RefCell<f32>>,
}

#[allow(clippy::new_without_default)]
impl ControlInput {
    pub fn new() -> (Self, Rc<RefCell<f32>>) {
        let buffer = Rc::new(RefCell::new(0.0));
        let control_input = Self {
            buffer: Rc::clone(&buffer),
        };
        (control_input, buffer)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ControlInputConsumer {}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ControlInputProducer;

impl Node<Signal> for ControlInput {
    type Consumer = ControlInputConsumer;
    type Producer = ControlInputProducer;

    fn read(&self, _producer: Self::Producer) -> Signal {
        Signal::from_control(*self.buffer.borrow())
    }
}

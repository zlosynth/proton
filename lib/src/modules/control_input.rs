use alloc::rc::Rc;
use core::cell::RefCell;

use graphity::Node;

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
pub enum NoConsumer {}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ControlInputProducer;

impl Node<[f32; 32]> for ControlInput {
    type Consumer = NoConsumer;
    type Producer = ControlInputProducer;

    fn read(&self, _producer: Self::Producer) -> [f32; 32] {
        [*self.buffer.borrow(); 32]
    }
}

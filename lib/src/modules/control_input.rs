use alloc::rc::Rc;
use core::cell::RefCell;

use graphity::Node;

use crate::core::signal::Signal;

type Buffer = f32;

pub struct ControlInput {
    cell: ControlInputCell,
}

#[allow(clippy::new_without_default)]
impl ControlInput {
    pub fn new() -> (Self, ControlInputCell) {
        let (cell_a, cell_b) = ControlInputCell::new_pair();
        let control_input = Self { cell: cell_a };
        (control_input, cell_b)
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
        Signal::from_control(self.cell.get())
    }
}

pub struct ControlInputCell {
    cell: Rc<RefCell<Buffer>>,
}

#[allow(clippy::new_without_default)]
impl ControlInputCell {
    fn new_pair() -> (Self, Self) {
        let cell_a = Self {
            cell: Rc::new(RefCell::new(Buffer::default())),
        };
        let cell_b = Self {
            cell: Rc::clone(&cell_a.cell),
        };
        (cell_a, cell_b)
    }

    fn get(&self) -> Buffer {
        *self.cell.borrow()
    }

    pub fn set(&mut self, value: Buffer) {
        *self.cell.borrow_mut() = value;
    }
}

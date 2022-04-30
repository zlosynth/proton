use alloc::rc::Rc;
use core::cell::RefCell;

use graphity::Node;

use crate::core::signal::Signal;

type Buffer = [f32; 32];

pub struct AudioOutput {
    cell: AudioOutputCell,
}

#[allow(clippy::new_without_default)]
impl AudioOutput {
    pub fn new() -> (Self, AudioOutputCell) {
        let (cell_a, cell_b) = AudioOutputCell::new_pair();
        let audio_output = Self { cell: cell_a };
        (audio_output, cell_b)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct AudioOutputConsumer;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum AudioOutputProducer {}

impl Node<Signal> for AudioOutput {
    type Consumer = AudioOutputConsumer;
    type Producer = AudioOutputProducer;

    fn write(&mut self, _consumer: Self::Consumer, input: Signal) {
        self.cell.set(input.as_audio());
    }
}

pub struct AudioOutputCell {
    cell: Rc<RefCell<Buffer>>,
}

#[allow(clippy::new_without_default)]
impl AudioOutputCell {
    fn new_pair() -> (Self, Self) {
        let cell_a = Self {
            cell: Rc::new(RefCell::new(Buffer::default())),
        };
        let cell_b = Self {
            cell: Rc::clone(&cell_a.cell),
        };
        (cell_a, cell_b)
    }

    pub fn get(&self) -> Buffer {
        *self.cell.borrow()
    }

    fn set(&mut self, value: Buffer) {
        *self.cell.borrow_mut() = value;
    }
}

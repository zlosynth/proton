use alloc::rc::Rc;
use core::cell::RefCell;

use graphity::Node;

pub struct AudioOutput {
    buffer: Rc<RefCell<[f32; 32]>>,
}

#[allow(clippy::new_without_default)]
impl AudioOutput {
    pub fn new() -> (Self, Rc<RefCell<[f32; 32]>>) {
        let buffer = Rc::new(RefCell::new([0.0; 32]));
        let audio_output = Self {
            buffer: Rc::clone(&buffer),
        };
        (audio_output, buffer)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct AudioOutputConsumer;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum AudioOutputProducer {}

impl Node<[f32; 32]> for AudioOutput {
    type Consumer = AudioOutputConsumer;
    type Producer = AudioOutputProducer;

    fn write(&mut self, _consumer: Self::Consumer, input: [f32; 32]) {
        *self.buffer.borrow_mut() = input;
    }
}

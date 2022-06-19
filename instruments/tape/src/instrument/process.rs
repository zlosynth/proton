use dasp::{signal, Signal};

use super::Instrument;

impl Instrument {
    pub fn process(&mut self, block: &mut [f32; 32]) {
        let block_copy = *block;

        let mut input_signal = signal::from_iter(block_copy.iter().cloned());

        block.iter_mut().for_each(|f| {
            *f = input_signal.next();
        });
    }
}

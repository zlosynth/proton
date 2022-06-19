use dasp::{signal, Frame, Signal};

use super::Instrument;

impl Instrument {
    pub fn process<F>(&mut self, block: &mut [F; 32])
    where
        F: Frame + Default,
    {
        let block_copy = *block;

        let mut input_signal = signal::from_iter(block_copy.iter().cloned());

        block.iter_mut().for_each(|f| {
            *f = input_signal.next();
        });
    }
}

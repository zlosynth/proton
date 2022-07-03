use dasp::{Frame, Signal};

use super::coefficients::COEFFICIENTS_2;
use crate::ring_buffer::RingBuffer;

pub trait SignalUpsampler: Signal {
    fn upsampled_2(
        self,
    ) -> Upsampler<Self, { COEFFICIENTS_2.len() }, { COEFFICIENTS_2.len() / 2 + 1 }>
    where
        Self: Sized,
    {
        Upsampler {
            source: self,
            factor: 2,
            coefficients: &COEFFICIENTS_2,
            buffer: RingBuffer::new(),
            coefficients_offset: 0,
        }
    }
}

impl<T> SignalUpsampler for T where T: Signal {}

pub struct Upsampler<S, const N: usize, const M: usize> {
    source: S,
    factor: usize,
    coefficients: &'static [f32; N],
    buffer: RingBuffer<M>,
    coefficients_offset: usize,
}

impl<S, const N: usize, const M: usize> Signal for Upsampler<S, N, M>
where
    S: Signal<Frame = f32>,
{
    type Frame = S::Frame;

    fn next(&mut self) -> Self::Frame {
        if self.coefficients_offset == 0 {
            self.buffer.write(self.source.next());
        }

        let mut output = Self::Frame::EQUILIBRIUM;
        let mut coefficients_index = self.coefficients_offset;

        while coefficients_index < self.coefficients.len() {
            let past_value_index = -(coefficients_index as i32) / self.factor as i32;
            let past_value = self.buffer.peek(past_value_index);
            let amplification = self.coefficients[coefficients_index];
            output = output.offset_amp(past_value * amplification);

            coefficients_index += self.factor;
        }

        self.coefficients_offset += 1;
        self.coefficients_offset %= self.factor;

        output
    }
}

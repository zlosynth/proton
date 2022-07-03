use dasp::{Frame, Signal};

use super::coefficients::COEFFICIENTS_2;
use crate::ring_buffer::RingBuffer;

pub trait SignalDownsampler: Signal {
    fn downsampled_2(self) -> Downsampler<Self, { COEFFICIENTS_2.len() }>
    where
        Self: Sized,
    {
        Downsampler {
            source: self,
            factor: 2,
            coefficients: &COEFFICIENTS_2,
            buffer: RingBuffer::new(),
        }
    }
}

impl<T> SignalDownsampler for T where T: Signal {}

pub struct Downsampler<S, const N: usize> {
    source: S,
    factor: usize,
    coefficients: &'static [f32; N],
    buffer: RingBuffer<N>,
}

impl<S, const N: usize> Signal for Downsampler<S, N>
where
    S: Signal<Frame = f32>,
{
    type Frame = S::Frame;

    fn next(&mut self) -> Self::Frame {
        (0..self.factor).for_each(|_| self.buffer.write(self.source.next()));

        let mut output = Self::Frame::EQUILIBRIUM;

        for (i, coefficient) in self.coefficients.iter().enumerate() {
            let past_value_index = -(i as i32);
            let past_value = self.buffer.peek(past_value_index);
            output = output.offset_amp(past_value * coefficient * self.factor as f32 * 0.9);
        }

        output
    }
}

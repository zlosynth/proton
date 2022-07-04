use dasp::{Frame, Signal};

use super::coefficients::COEFFICIENTS_2;
use crate::ring_buffer::RingBuffer;

pub struct Upsampler<const N: usize, const M: usize> {
    factor: usize,
    coefficients: &'static [f32; N],
    buffer: RingBuffer<M>,
    coefficients_offset: usize,
}

pub type Upsampler2 = Upsampler<{ COEFFICIENTS_2.len() }, { COEFFICIENTS_2.len() / 2 + 1 }>;

impl Upsampler2 {
    pub fn new() -> Self {
        Self {
            factor: 2,
            coefficients: &COEFFICIENTS_2,
            buffer: RingBuffer::new(),
            coefficients_offset: 0,
        }
    }
}

impl Default for Upsampler2 {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SignalUpsample: Signal {
    fn upsample<const N: usize, const M: usize>(
        self,
        upsampler: &mut Upsampler<N, M>,
    ) -> Upsample<Self, N, M>
    where
        Self: Sized,
    {
        Upsample {
            source: self,
            upsampler,
        }
    }
}

impl<T> SignalUpsample for T where T: Signal {}

pub struct Upsample<'a, S, const N: usize, const M: usize> {
    source: S,
    upsampler: &'a mut Upsampler<N, M>,
}

impl<'a, S, const N: usize, const M: usize> Signal for Upsample<'a, S, N, M>
where
    S: Signal<Frame = f32>,
{
    type Frame = S::Frame;

    fn next(&mut self) -> Self::Frame {
        let upsampler = &mut self.upsampler;

        if upsampler.coefficients_offset == 0 {
            upsampler.buffer.write(self.source.next());
        }

        let mut output = Self::Frame::EQUILIBRIUM;
        let mut coefficients_index = upsampler.coefficients_offset;

        while coefficients_index < upsampler.coefficients.len() {
            let past_value_index = -(coefficients_index as i32) / upsampler.factor as i32;
            let past_value = upsampler.buffer.peek(past_value_index);
            let amplification = upsampler.coefficients[coefficients_index];
            output = output.offset_amp(past_value * amplification);

            coefficients_index += upsampler.factor;
        }

        upsampler.coefficients_offset += 1;
        upsampler.coefficients_offset %= upsampler.factor;

        output
    }
}

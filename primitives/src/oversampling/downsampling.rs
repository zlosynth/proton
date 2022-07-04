use dasp::{Frame, Signal};

use super::coefficients::COEFFICIENTS_2;
use crate::ring_buffer::RingBuffer;

pub struct Downsampler<const N: usize> {
    factor: usize,
    coefficients: &'static [f32; N],
    buffer: RingBuffer<N>,
}

pub type Downsampler2 = Downsampler<{ COEFFICIENTS_2.len() }>;

impl Downsampler2 {
    pub fn new() -> Self {
        Self {
            factor: 2,
            coefficients: &COEFFICIENTS_2,
            buffer: RingBuffer::new(),
        }
    }
}

impl Default for Downsampler2 {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SignalDownsample: Signal {
    fn downsample<const N: usize>(self, downsampler: &mut Downsampler<N>) -> Downsample<Self, N>
    where
        Self: Sized,
    {
        Downsample {
            source: self,
            downsampler,
        }
    }
}

impl<T> SignalDownsample for T where T: Signal {}

pub struct Downsample<'a, S, const N: usize> {
    source: S,
    downsampler: &'a mut Downsampler<N>,
}

impl<'a, S, const N: usize> Signal for Downsample<'a, S, N>
where
    S: Signal<Frame = f32>,
{
    type Frame = S::Frame;

    fn next(&mut self) -> Self::Frame {
        let downsampler = &mut self.downsampler;

        (0..downsampler.factor).for_each(|_| downsampler.buffer.write(self.source.next()));

        let mut output = Self::Frame::EQUILIBRIUM;

        for (i, coefficient) in downsampler.coefficients.iter().enumerate() {
            let past_value_index = -(i as i32);
            let past_value = downsampler.buffer.peek(past_value_index);
            output = output.offset_amp(past_value * coefficient * downsampler.factor as f32 * 0.9);
        }

        output
    }
}

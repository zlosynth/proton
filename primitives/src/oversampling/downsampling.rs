use core::fmt;

use dasp::{Frame, Signal};

use super::coefficients::*;
use crate::ring_buffer::RingBuffer;

pub struct Downsampler<const N: usize> {
    factor: usize,
    coefficients: &'static [f32; N],
    buffer: RingBuffer<N>,
}

impl<const N: usize> fmt::Debug for Downsampler<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Downsampler")
            .field("factor", &self.factor)
            .finish()
    }
}

pub type Downsampler2 = Downsampler<{ COEFFICIENTS_2.len() }>;

impl Downsampler2 {
    pub fn new_2() -> Self {
        Self {
            factor: 2,
            coefficients: &COEFFICIENTS_2,
            buffer: RingBuffer::new(),
        }
    }
}

pub type Downsampler8 = Downsampler<{ COEFFICIENTS_8.len() }>;

impl Downsampler8 {
    pub fn new_8() -> Self {
        Self {
            factor: 8,
            coefficients: &COEFFICIENTS_8,
            buffer: RingBuffer::new(),
        }
    }
}

pub type Downsampler16 = Downsampler<{ COEFFICIENTS_16.len() }>;

impl Downsampler16 {
    pub fn new_16() -> Self {
        Self {
            factor: 16,
            coefficients: &COEFFICIENTS_16,
            buffer: RingBuffer::new(),
        }
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

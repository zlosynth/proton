pub struct RingBuffer<const N: usize> {
    buffer: [f32; N],
    write_index: usize,
}

impl<const N: usize> RingBuffer<N> {
    pub fn new() -> Self {
        let buffer = {
            let mut data: [core::mem::MaybeUninit<f32>; N] =
                unsafe { core::mem::MaybeUninit::uninit().assume_init() };
            for elem in &mut data[..] {
                unsafe {
                    core::ptr::write(elem.as_mut_ptr(), 0.0);
                }
            }
            unsafe { core::mem::transmute_copy(&data) }
        };

        Self {
            buffer,
            write_index: 0,
        }
    }

    pub fn write(&mut self, value: f32) {
        self.write_index %= N;
        self.buffer[self.write_index] = value;
        self.write_index += 1;
    }

    pub fn peek(&self, relative_index: i32) -> f32 {
        let index =
            (self.write_index as i32 + relative_index - 1).wrapping_rem_euclid(N as i32) as usize;
        self.buffer[index]
    }
}

impl<const N: usize> Default for RingBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_buffer() {
        let _buffer = RingBuffer::<2>::new();
    }

    #[test]
    fn write_to_buffer() {
        let mut buffer = RingBuffer::<2>::new();

        buffer.write(1.0);
    }

    #[test]
    fn read_from_buffer() {
        let mut buffer = RingBuffer::<3>::new();
        buffer.write(1.0);
        buffer.write(2.0);
        buffer.write(3.0);

        assert_eq!(buffer.peek(0), 3.0);
        assert_eq!(buffer.peek(-1), 2.0);
        assert_eq!(buffer.peek(-2), 1.0);
    }

    #[test]
    fn cross_buffer_end_while_reading() {
        let mut buffer = RingBuffer::<101>::new();
        for x in 0..=100 {
            buffer.write(x as f32);
        }

        assert_eq!(buffer.peek(0) as usize, 100);
        assert_eq!(buffer.peek(-1) as usize, 100 - 1);
    }
}

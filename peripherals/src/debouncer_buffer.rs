#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DebounceBuffer<const N: usize> {
    buffer: [bool; N],
    pointer: usize,
}

impl<const N: usize> DebounceBuffer<N> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            buffer: [false; N],
            pointer: 0,
        }
    }

    pub fn write(&mut self, value: bool) {
        self.buffer[self.pointer] = value;
        self.pointer = (self.pointer + 1) % N;
    }

    pub fn read(&self) -> bool {
        let up: usize = self.buffer.iter().filter(|i| **i).count();
        up > N / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_input_fluctuates_it_does_not_change_output() {
        let mut buffer = DebounceBuffer::<4>::new();

        for _ in 0..4 {
            buffer.write(true);
        }
        let original = buffer.read();

        buffer.write(true);
        buffer.write(true);
        buffer.write(false);
        buffer.write(true);

        assert_eq!(buffer.read(), original);
    }

    #[test]
    fn when_input_is_stable_it_is_reflected_in_output() {
        let mut buffer = DebounceBuffer::<4>::new();

        for _ in 0..4 {
            buffer.write(true);
        }
        assert!(buffer.read());

        for _ in 0..4 {
            buffer.write(false);
        }
        assert!(!buffer.read());
    }
}

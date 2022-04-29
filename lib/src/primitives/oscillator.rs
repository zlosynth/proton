#[allow(unused_imports)]
use micromath::F32Ext;

pub struct Oscillator {
    sample_rate: f32,
    phase: f32,
    pub frequency: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            phase: 0.0,
            frequency: 0.0,
        }
    }

    pub fn populate(&mut self, buffer: &mut [f32]) {
        for x in buffer {
            *x = if self.phase < 0.5 {
                self.phase * 2.0
            } else {
                -1.0 + (self.phase - 0.5) * 2.0
            };
            self.phase += self.frequency / self.sample_rate;
            self.phase = self.phase.fract();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_initialized() {
        const SAMPLE_RATE: f32 = 64.0;
        let _oscillator = Oscillator::new(SAMPLE_RATE);
    }

    fn assert_saw_cycle(buffer: &[f32], start: usize, stop: usize) {
        assert_relative_eq!(buffer[start], 0.0, epsilon = 0.1);
        assert_relative_eq!(buffer[stop], 0.0, epsilon = 0.1);

        let mut value = buffer[start];
        let mut middle_index = None;

        for i in start + 1..=stop {
            let new_value = buffer[i];

            if new_value < value {
                if middle_index.is_some() {
                    panic!("Found second drop");
                } else {
                    middle_index = Some(i);
                }
            }

            value = new_value;
        }

        let middle_index = middle_index.expect("Did not find waveform drop");
        let expected_middle = (start + stop) / 2;
        let toleration = (stop - start) / 16;
        assert!(
            middle_index > expected_middle - toleration
                && middle_index < expected_middle + toleration,
            "Drop of the waform was not in the center"
        );
    }

    #[test]
    fn when_set_frequency_1hz_output_matches() {
        const SAMPLE_RATE: f32 = 64.0;
        let mut oscillator = Oscillator::new(SAMPLE_RATE);
        oscillator.frequency = 1.0;

        let mut buffer = [0.0; 64];
        oscillator.populate(&mut buffer);

        assert_saw_cycle(&buffer, 0, 63);
    }

    #[test]
    fn when_set_frequency_over_1hz_output_matches() {
        const SAMPLE_RATE: f32 = 64.0;
        let mut oscillator = Oscillator::new(SAMPLE_RATE);
        oscillator.frequency = 2.0;

        let mut buffer = [0.0; 64];
        oscillator.populate(&mut buffer);

        assert_saw_cycle(&buffer, 0, 32);
        assert_saw_cycle(&buffer, 33, 63);
    }

    #[test]
    fn when_set_frequency_bellow_1hz_output_matches() {
        const SAMPLE_RATE: f32 = 64.0;
        let mut oscillator = Oscillator::new(SAMPLE_RATE);
        oscillator.frequency = 0.5;

        let mut buffer = [0.0; 128];
        oscillator.populate(&mut buffer);

        assert_saw_cycle(&buffer, 0, 127);
    }
}

use dasp::signal::{self, Noise};

pub struct WhiteNoise {
    noise: Noise,
}

impl Default for WhiteNoise {
    fn default() -> Self {
        Self::new()
    }
}

impl WhiteNoise {
    pub fn new() -> Self {
        Self {
            noise: signal::noise(0),
        }
    }

    pub fn pop(&mut self) -> f32 {
        self.noise.next_sample() as f32
    }

    pub fn populate(&mut self, buffer: &mut [f32]) {
        for i in buffer.iter_mut() {
            *i = self.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_white_noise_array() -> [f32; 0xfff] {
        let mut white_noise = WhiteNoise::new();
        let mut buffer = [0.0; 0xfff];
        white_noise.populate(&mut buffer);
        buffer
    }

    #[test]
    fn noise_average_is_around_zero() {
        let noise = new_white_noise_array();
        let average = noise.iter().sum::<f32>() / noise.len() as f32;
        assert!(average.abs() < 0.1);
    }

    #[test]
    fn noise_gets_close_to_max() {
        let noise = new_white_noise_array();
        let max: f32 = noise.iter().fold(0.0, |a, b| a.max(*b));
        assert!(max > 0.9);
    }

    #[test]
    fn noise_gets_close_to_min() {
        let noise = new_white_noise_array();
        let max: f32 = noise.iter().fold(0.0, |a, b| a.min(*b));
        assert!(max < -0.9);
    }
}

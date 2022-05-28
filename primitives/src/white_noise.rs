use dasp::signal;

pub fn pop() -> f32 {
    let mut noise = signal::noise(0);
    noise.next_sample() as f32
}

pub fn populate(buffer: &mut [f32]) {
    for i in buffer.iter_mut() {
        *i = pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_white_noise_array() -> [f32; 0xfff] {
        let mut buffer = [0.0; 0xfff];
        for x in buffer.iter_mut() {
            *x = pop();
        }
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

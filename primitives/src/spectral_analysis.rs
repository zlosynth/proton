use heapless::Vec;

const N: usize = 1024;

pub struct SpectralAnalysis {
    bins: Vec<f32, N>,
    bin_width: f32,
}

impl SpectralAnalysis {
    pub fn analyze(signal: &[f32; N], sample_rate: u32) -> Self {
        let magnitude = fft_magnitude(&signal);
        let bins_length = f32::ceil(signal.len() as f32 / 2.0) as usize;
        let bins: Vec<f32, N> = magnitude.iter().take(bins_length).copied().collect();
        let bin_width = sample_rate as f32 / signal.len() as f32;
        Self { bins, bin_width }
    }

    pub fn magnitude(&self, frequency: f32) -> f32 {
        let bin_index = self.index(frequency);

        if bin_index > self.bins.len() {
            0.0
        } else {
            self.bins[bin_index]
        }
    }

    pub fn mean_magnitude(&self, bottom_frequency: f32, top_frequency: f32) -> f32 {
        assert!(bottom_frequency < top_frequency);

        let bottom_index = self.index(bottom_frequency);
        if bottom_index > self.bins.len() {
            return 0.0;
        }

        let requested_top_index = self.index(top_frequency);
        let actual_top_index = usize::min(requested_top_index, self.bins.len() - 1);

        let sum = (bottom_index..=actual_top_index).fold(0.0, |sum, i| sum + self.bins[i]);

        sum / (requested_top_index - bottom_index) as f32
    }

    pub fn lowest_peak(&self, relative_treshold: f32) -> f32 {
        let peak_index = lowest_peak_index(&self.bins, relative_treshold);
        self.frequency(peak_index)
    }

    pub fn trash_range(&mut self, bottom_frequency: f32, top_frequency: f32) {
        assert!(bottom_frequency < top_frequency);
        assert!(bottom_frequency >= 0.0);

        let bottom_index = self.index(bottom_frequency);

        let requested_top_index = self.index(top_frequency);
        let actual_top_index = usize::min(requested_top_index, self.bins.len() - 1);

        (bottom_index..=actual_top_index).for_each(|i| self.bins[i] = 0.0);
    }

    fn index(&self, frequency: f32) -> usize {
        (frequency / self.bin_width) as usize
    }

    fn frequency(&self, index: usize) -> f32 {
        index as f32 * self.bin_width
    }
}

fn lowest_peak_index(data: &[f32], relative_treshold: f32) -> usize {
    assert!(relative_treshold > 0.0 && relative_treshold <= 1.0);

    let treshold = {
        let maximal_peak = data.iter().fold(0.0, |max, x| f32::max(max, *x));
        maximal_peak * relative_treshold
    };

    let treshold_finder = data.iter().enumerate();
    let peak_finder = treshold_finder.skip_while(|(_, x)| **x < treshold);
    let peak_index = {
        let mut index = 0;
        let mut current_peak = 0.0;
        for (i, x) in peak_finder {
            if *x > current_peak {
                index = i;
                current_peak = *x;
            } else {
                break;
            }
        }
        index
    };

    peak_index
}

fn fft_magnitude(signal: &[f32; N]) -> Vec<f32, N> {
    if signal.is_empty() {
        return Vec::new();
    }

    let mut signal = *signal;
    let spectrum = microfft::real::rfft_1024(&mut signal);
    spectrum[0].im = 0.0;

    let amplitudes: Vec<_, N> = spectrum.iter().map(|c| c.norm_sqr()).collect();

    amplitudes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::white_noise::WhiteNoise;
    use core::f32::consts::PI;

    #[test]
    fn fft_magnitude_check() {
        let mut signal = [0.0; N];
        signal[0] = 1.0;

        let magnitude = fft_magnitude(&signal);

        for i in 0..magnitude.len() {
            assert_relative_eq!(magnitude[i], 1.0);
        }
    }

    #[test]
    fn initialize_analyzer() {
        const SAMPLE_RATE: u32 = 44100;
        let _analysis = SpectralAnalysis::analyze(&[1.0; N], SAMPLE_RATE);
    }

    fn write_sine(buffer: &mut [f32], frequency: f32, sample_rate: u32) {
        for (i, x) in buffer.iter_mut().enumerate() {
            *x = f32::sin((i as f32 / sample_rate as f32) * frequency * 2.0 * PI);
        }
    }

    #[test]
    fn analyze_magnitude_simple_sinusoid() {
        const SAMPLE_RATE: u32 = 11;
        let frequency = 3.0;
        let signal = {
            let mut signal = [0.0; N];
            write_sine(&mut signal, frequency, SAMPLE_RATE);
            signal
        };

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let magnitude = analysis.magnitude(frequency);

        assert!(magnitude > 10.0);
        assert!(analysis.magnitude(frequency - 1.0) < magnitude);
        assert!(analysis.magnitude(frequency + 1.0) < magnitude);
    }

    #[test]
    fn analyze_mean_magnitude_in_range() {
        const SAMPLE_RATE: u32 = 11;
        let ringing_frequency = 3.0;
        let silent_frequency = 1.0;
        let signal = {
            let mut signal = [0.0; N];
            write_sine(&mut signal, ringing_frequency, SAMPLE_RATE);
            signal
        };

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let mean_ringing_magnitude =
            analysis.mean_magnitude(ringing_frequency - 0.5, ringing_frequency + 0.5);
        let mean_silent_magnitude =
            analysis.mean_magnitude(silent_frequency - 0.5, silent_frequency + 0.5);

        assert!(mean_ringing_magnitude > 5.0);
        assert!(mean_ringing_magnitude > mean_silent_magnitude);
    }

    #[test]
    fn analyze_mean_magnitude_with_bottom_over_the_range() {
        const SAMPLE_RATE: u32 = 3;
        let mut signal = [0.0; N];
        signal[0] = 1.0;

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let mean_magnitude = analysis.mean_magnitude(20.0, 21.0);

        assert_relative_eq!(mean_magnitude, 0.0);
    }

    #[test]
    fn analyze_mean_magnitude_with_top_over_the_range() {
        const SAMPLE_RATE: u32 = 3;
        let mut signal = [0.0; N];
        signal[0] = 1.0;

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let mean_magnitude = analysis.mean_magnitude(1.0, 21.0);

        assert!(mean_magnitude > 0.01);
    }

    #[test]
    fn analyze_mean_magnitude_of_white_noise() {
        const SAMPLE_RATE: u32 = 1200;

        let mut signal = [0.0; N];
        WhiteNoise::new().populate(&mut signal);

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let low_mean_magnitude = analysis.mean_magnitude(0.0, 200.0);
        let middle_mean_magnitude = analysis.mean_magnitude(200.0, 400.0);
        let high_mean_magnitude = analysis.mean_magnitude(400.0, 600.0);

        assert_relative_eq!(
            low_mean_magnitude,
            middle_mean_magnitude,
            max_relative = 1.0
        );
        assert_relative_eq!(
            middle_mean_magnitude,
            high_mean_magnitude,
            max_relative = 1.0
        );
    }

    #[test]
    fn find_the_tip_of_the_lowest_peak() {
        const SAMPLE_RATE: u32 = 1000;
        let frequency = 29.0;
        let signal = {
            let mut signal = [0.0; N];
            write_sine(&mut signal, frequency, SAMPLE_RATE);
            signal
        };

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let low_magnitude = analysis.magnitude(frequency - 1.0);
        let center_magnitude = analysis.magnitude(frequency);
        assert!(center_magnitude > low_magnitude);

        let treshold = low_magnitude / center_magnitude;

        let lowest_peak = analysis.lowest_peak(treshold);
        assert_relative_eq!(lowest_peak, frequency, epsilon = 0.3);
    }

    #[test]
    fn find_lowest_peak_index_without_slope() {
        let mut data = [0.0; N];
        data[N / 2] = 9.0;
        data[3 * N / 4] = 10.0;
        let peak_index = lowest_peak_index(&data, 0.5);
        assert_eq!(peak_index, N / 2);
    }

    #[test]
    fn find_lowest_peak_index_with_slope() {
        let mut data = [0.0; N];
        data[N / 2 - 2] = 3.0;
        data[N / 2 - 1] = 6.0;
        data[N / 2] = 9.0;
        data[3 * N / 4] = 10.0;
        let peak_index = lowest_peak_index(&data, 0.5);
        assert_eq!(peak_index, N / 2);
    }

    #[test]
    #[should_panic]
    fn panic_lowest_peak_index_treshold_below_zero() {
        let data = [0.0; N];
        let _peak_index = lowest_peak_index(&data, -1.0);
    }

    #[test]
    #[should_panic]
    fn panic_lowest_peak_index_treshold_over_one() {
        let data = [0.0, 1.0];
        let _peak_index = lowest_peak_index(&data, 2.0);
    }

    #[test]
    fn trash_records() {
        const SAMPLE_RATE: u32 = 100;

        let mut signal = [0.0; N];
        WhiteNoise::new().populate(&mut signal);

        let mut analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        analysis.trash_range(0.0, 50.0);

        let mean_magnitude = analysis.mean_magnitude(0.0, 50.0);
        assert_relative_eq!(mean_magnitude, 0.0);
    }

    #[test]
    #[should_panic]
    fn trash_panics_when_bottom_is_below_zero() {
        const SAMPLE_RATE: u32 = 100;
        let signal = [0.0; N];

        let mut analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);

        analysis.trash_range(-1.0, 50.0);
    }

    #[test]
    #[should_panic]
    fn trash_panics_when_bottom_is_above_top() {
        const SAMPLE_RATE: u32 = 100;
        let signal = [0.0; N];

        let mut analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);

        analysis.trash_range(10.0, 5.0);
    }
}

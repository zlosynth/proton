#[allow(unused_imports)]
use micromath::F32Ext;

use core::f32::consts::PI;

pub struct StateVariableFilter {
    sample_rate: u32,
    bandform: Bandform,
    f: f32,
    q: f32,
    delay_1: f32,
    delay_2: f32,
}

impl StateVariableFilter {
    pub fn new(sample_rate: u32) -> Self {
        let mut filter = Self {
            sample_rate,
            bandform: BandPass,
            f: 0.0,
            q: 0.0,
            delay_1: 0.0,
            delay_2: 0.0,
        };
        filter.set_q_factor(0.7);
        filter.set_frequency(0.0);
        filter
    }

    pub fn set_bandform(&mut self, bandform: Bandform) -> &mut Self {
        self.bandform = bandform;
        self
    }

    pub fn set_frequency(&mut self, frequency: f32) -> &mut Self {
        self.f = 2.0 * f32::sin((PI * frequency) / self.sample_rate as f32);
        self
    }

    pub fn set_q_factor(&mut self, q_factor: f32) -> &mut Self {
        self.q = 1.0 / f32::max(q_factor, 0.5);
        self
    }

    pub fn process(&mut self, signal: &mut [f32]) {
        for x in signal.iter_mut() {
            *x = self.tick(*x);
        }
    }

    pub fn pass(&mut self, signal: &[f32]) {
        for x in signal.iter() {
            self.tick(*x);
        }
    }

    // https://www.earlevel.com/main/2003/03/02/the-digital-state-variable-filter/
    //
    //             +----------------------------------------------------------+
    //             |                                                          |
    //             +-->[high pass]      +-->[band pass]                    [sum 4]-->[band reject]
    //             |                    |                                     |
    // -->[sum 1]--+--[mul f]--[sum 2]--+->[delay 1]--+--[mul f]--[sum 3]--+--+----+-->[low pass]
    //    - A  A -                A                   |              A     |       |
    //      |   \                 |                   |              |  [delay 2]  |
    //      |    \                +-------------------+              |     |       |
    //      |     \                                   |              +-----+       |
    //      |      \---[mut q]------------------------+                            |
    //      |                                                                      |
    //      +----------------------------------------------------------------------+
    //
    pub fn tick(&mut self, value: f32) -> f32 {
        let sum_3 = self.delay_1 * self.f + self.delay_2;
        let sum_1 = value - sum_3 - self.delay_1 * self.q;
        let sum_2 = sum_1 * self.f + self.delay_1;

        let value = match self.bandform {
            LowPass => sum_3,
            HighPass => sum_1,
            BandPass => sum_2,
            BandReject => {
                #[allow(clippy::let_and_return)]
                let sum_4 = sum_1 + sum_3;
                sum_4
            }
        };

        self.delay_1 = sum_2;
        self.delay_2 = sum_3;

        f32::max(f32::min(value, 1.0), -1.0)
    }
}

pub enum Bandform {
    LowPass,
    HighPass,
    BandPass,
    BandReject,
}

pub use Bandform::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectral_analysis::SpectralAnalysis;
    use crate::white_noise;

    #[test]
    fn initialize_filter() {
        const SAMPLE_RATE: u32 = 1;
        let _filter = StateVariableFilter::new(SAMPLE_RATE);
    }

    #[test]
    fn low_pass() {
        const SAMPLE_RATE: u32 = 1024;

        let mut signal = [0.0; 1024];
        white_noise::populate(&mut signal);

        let mut filter = StateVariableFilter::new(SAMPLE_RATE);
        filter.set_bandform(LowPass).set_frequency(100.0);
        filter.process(&mut signal);

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let low_mean_magnitude = analysis.mean_magnitude(0.0, 100.0);
        let high_mean_magnitude = analysis.mean_magnitude(100.0, 600.0);

        assert!(low_mean_magnitude / high_mean_magnitude > 3.0);
    }

    #[test]
    fn high_pass() {
        const SAMPLE_RATE: u32 = 1024;

        let mut signal = [0.0; 1024];
        white_noise::populate(&mut signal);

        let mut filter = StateVariableFilter::new(SAMPLE_RATE);
        filter.set_bandform(HighPass).set_frequency(100.0);
        filter.process(&mut signal);

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let low_mean_magnitude = analysis.mean_magnitude(0.0, 100.0);
        let high_mean_magnitude = analysis.mean_magnitude(100.0, 600.0);

        assert!(high_mean_magnitude / low_mean_magnitude > 3.0);
    }

    #[test]
    fn band_pass() {
        const SAMPLE_RATE: u32 = 1024;

        let mut signal = [0.0; 1024];
        white_noise::populate(&mut signal);

        let mut filter = StateVariableFilter::new(SAMPLE_RATE);
        filter
            .set_bandform(BandPass)
            .set_frequency(300.0)
            .set_q_factor(10.0);
        filter.process(&mut signal);

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let low_mean_magnitude = analysis.mean_magnitude(0.0, 250.0);
        let center_mean_magnitude = analysis.mean_magnitude(250.0, 350.0);
        let high_mean_magnitude = analysis.mean_magnitude(400.0, 600.0);

        assert!(center_mean_magnitude / low_mean_magnitude > 3.0);
        assert!(center_mean_magnitude / high_mean_magnitude > 3.0);
    }

    #[test]
    fn band_reject() {
        const SAMPLE_RATE: u32 = 1024;

        let mut signal = [0.0; 1024];
        white_noise::populate(&mut signal);

        let mut filter = StateVariableFilter::new(SAMPLE_RATE);
        filter
            .set_bandform(BandReject)
            .set_frequency(300.0)
            .set_q_factor(1.0);
        filter.process(&mut signal);

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let low_mean_magnitude = analysis.mean_magnitude(0.0, 250.0);
        let center_mean_magnitude = analysis.mean_magnitude(250.0, 350.0);
        let high_mean_magnitude = analysis.mean_magnitude(400.0, 600.0);

        assert!(low_mean_magnitude / center_mean_magnitude > 2.0);
        assert!(high_mean_magnitude / center_mean_magnitude > 2.0);
    }

    #[test]
    fn dry_pass_does_not_change_signal() {
        const SAMPLE_RATE: u32 = 1024;

        let mut signal = [0.0; 1024];
        white_noise::populate(&mut signal);

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let original_mean_magnitude = analysis.mean_magnitude(0.0, 200.0);

        let mut filter = StateVariableFilter::new(SAMPLE_RATE);
        filter.set_bandform(LowPass).set_frequency(100.0);
        filter.pass(&mut signal);

        let analysis = SpectralAnalysis::analyze(&signal, SAMPLE_RATE);
        let updated_mean_magnitude = analysis.mean_magnitude(0.0, 200.0);

        assert_relative_eq!(original_mean_magnitude, updated_mean_magnitude);
    }
}

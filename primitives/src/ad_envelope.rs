// Thanks to Nigel Redmon for his series about ADSR
// http://www.earlevel.com/main/category/envelope-generators/

#[allow(unused_imports)]
use micromath::F32Ext;

pub struct Ad {
    sample_rate: f32,
    config: Config,
    value: f32,
    state: State,
    cache: Cache,
}

impl Ad {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            config: Config::new(),
            value: 0.0,
            state: State::Dormant,
            cache: Cache::default(),
        }
    }

    pub fn pop(&mut self) -> f32 {
        let (state, value) = match self.state {
            State::Attack(i) => self.next_attack(i),
            State::Decay(i) => self.next_decay(i),
            State::Dormant => (State::Dormant, 0.0),
        };
        self.state = state;
        self.value = value;
        self.value
    }

    fn next_attack(&self, previous_rate: u32) -> (State, f32) {
        let new_rate = previous_rate + 1;

        let new_value = {
            let mut new_value = self.cache.attack_base + self.value * self.cache.attack_coefficient;

            // f32 may not be accurate enough to converge when the difference is
            // small. To overcome this, switch to linear mode.
            if (new_value - self.value).abs() < f32::EPSILON {
                new_value = f32::max(new_rate as f32 / self.cache.attack_rate as f32, new_value);
            }

            new_value
        };

        if new_value >= 1.0 {
            (State::Decay(0), 1.0)
        } else {
            (State::Attack(previous_rate + 1), new_value)
        }
    }

    fn next_decay(&self, previous_rate: u32) -> (State, f32) {
        let new_rate = previous_rate + 1;

        let new_value = {
            let mut new_value = self.cache.decay_base + self.value * self.cache.decay_coefficient;

            // f32 may not be accurate enough to converge when the difference is
            // small. To overcome this, switch to linear mode.
            if new_value == self.value {
                new_value = 1.0 - new_rate as f32 / self.cache.decay_rate as f32;
            }

            new_value
        };

        if new_value <= 0.0 {
            (State::Dormant, 0.0)
        } else {
            (State::Decay(previous_rate + 1), new_value)
        }
    }

    pub fn trigger(&mut self, mut config: Config) {
        config.attack_ratio = config.attack_ratio.clamp(0.000000001, 100000.0);
        let attack_rate = (config.attack_time * self.sample_rate) as u32;
        let attack_coefficient = Self::calculate_coefficient(attack_rate, config.attack_ratio);
        let attack_base = (1.0 + config.attack_ratio) * (1.0 - attack_coefficient);

        config.decay_ratio = config.decay_ratio.clamp(0.000000001, 100000.0);
        let decay_rate = (config.decay_time * self.sample_rate) as u32;
        let decay_coefficient = Self::calculate_coefficient(decay_rate, config.decay_ratio);
        let decay_base = -config.decay_ratio * (1.0 - decay_coefficient);

        self.state = State::Attack(0);
        self.config = config;
        self.cache = Cache {
            attack_rate,
            attack_coefficient,
            attack_base,
            decay_rate,
            decay_coefficient,
            decay_base,
        };
    }

    fn calculate_coefficient(rate: u32, ratio: f32) -> f32 {
        f32::exp(-f32::ln((1.0 + ratio) / ratio) / rate as f32)
    }
}

enum State {
    Attack(u32),
    Decay(u32),
    Dormant,
}

#[derive(Clone, Copy)]
pub struct Config {
    pub attack_time: f32,
    pub attack_ratio: f32,
    pub decay_time: f32,
    pub decay_ratio: f32,
}

impl Config {
    pub fn new() -> Self {
        Config {
            attack_time: 0.0,
            attack_ratio: 1.0,
            decay_time: 0.0,
            decay_ratio: 1.0,
        }
    }

    pub fn with_attack_time(mut self, attack_time: f32) -> Self {
        self.attack_time = attack_time;
        self
    }

    pub fn with_attack_ratio(mut self, attack_ratio: f32) -> Self {
        self.attack_ratio = attack_ratio;
        self
    }

    pub fn with_decay_time(mut self, attack_time: f32) -> Self {
        self.decay_time = attack_time;
        self
    }

    pub fn with_decay_ratio(mut self, attack_ratio: f32) -> Self {
        self.decay_ratio = attack_ratio;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
struct Cache {
    pub attack_rate: u32,
    pub attack_coefficient: f32,
    pub attack_base: f32,
    pub decay_rate: u32,
    pub decay_coefficient: f32,
    pub decay_base: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use test_case::test_case;

    #[test]
    fn envelope_should_get_initialized() {
        const SAMPLE_RATE: f32 = 10.0;
        let _ad = Ad::new(SAMPLE_RATE);
    }

    #[test]
    fn untriggered_envelope_should_stay_silent() {
        const SAMPLE_RATE: f32 = 10.0;
        let mut ad = Ad::new(SAMPLE_RATE);

        for _ in 0..10 {
            assert_relative_eq!(ad.pop(), 0.0);
        }
    }

    #[test_case(1.0, 1.0, 0.0, 1.0; "with no decay")]
    #[test_case(0.0, 1.0, 1.0, 1.0; "with no attack")]
    #[test_case(0.01, 1.0, 0.01, 1.0; "with short attack and decay")]
    #[test_case(1.0, 1.0, 1.0, 1.0; "with medium attack and decay")]
    #[test_case(600.0, 1.0, 600.0, 1.0; "inconclusive: with very long attack and decay")]
    #[test_case(0.01, f32::MAX, 0.01, f32::MAX; "with short flattest curves")]
    #[test_case(0.01, 0.0, 0.01, 0.0; "with short most exponential curves")]
    #[test_case(1.0, f32::MAX, 1.0, f32::MAX; "with medium flattest curves")]
    #[test_case(1.0, 0.0, 1.0, 0.0; "with medium most exponential curves")]
    #[test_case(600.0, f32::MAX, 600.0, f32::MAX; "inconclusive: with very long flattest curves")]
    #[test_case(600.0, 0.0, 600.0, 0.0; "inconclusive: with very long most exponential curves")]
    fn triggered_envelope_should_rise_and_fall_with_audio_sample_rate(
        attack_rate: f32,
        attack_ratio: f32,
        decay_rate: f32,
        decay_ratio: f32,
    ) {
        const SAMPLE_RATE: f32 = 48000.0;
        let config = Config::new()
            .with_attack_time(attack_rate)
            .with_attack_ratio(attack_ratio)
            .with_decay_time(decay_rate)
            .with_decay_ratio(decay_ratio);
        assert_rising(SAMPLE_RATE, config);
        assert_falling(SAMPLE_RATE, config);
    }

    #[test_case(1.0, 1.0, 0.0, 1.0; "with no decay")]
    #[test_case(0.0, 1.0, 1.0, 1.0; "with no attack")]
    #[test_case(0.01, 1.0, 0.01, 1.0; "with short attack and decay")]
    #[test_case(1.0, 1.0, 1.0, 1.0; "with medium attack and decay")]
    #[test_case(600.0, 1.0, 600.0, 1.0; "inconclusive: with very long attack and decay")]
    #[test_case(0.01, f32::MAX, 0.01, f32::MAX; "with short flattest curves")]
    #[test_case(0.01, 0.0, 0.01, 0.0; "with short most exponential curves")]
    #[test_case(1.0, f32::MAX, 1.0, f32::MAX; "with medium flattest curves")]
    #[test_case(1.0, 0.0, 1.0, 0.0; "with medium most exponential curves")]
    #[test_case(600.0, f32::MAX, 600.0, f32::MAX; "inconclusive: with very long flattest curves")]
    #[test_case(600.0, 0.0, 600.0, 0.0; "inconclusive: with very long most exponential curves")]
    fn triggered_envelope_should_rise_and_fall_with_cv_sample_rate(
        attack_rate: f32,
        attack_ratio: f32,
        decay_rate: f32,
        decay_ratio: f32,
    ) {
        const SAMPLE_RATE: f32 = 500.0;
        let config = Config::new()
            .with_attack_time(attack_rate)
            .with_attack_ratio(attack_ratio)
            .with_decay_time(decay_rate)
            .with_decay_ratio(decay_ratio);
        assert_rising(SAMPLE_RATE, config);
        assert_falling(SAMPLE_RATE, config);
    }

    #[test]
    fn envelope_should_eventually_become_silent() {
        const SAMPLE_RATE: f32 = 10.0;
        let mut ad = Ad::new(SAMPLE_RATE);

        ad.trigger(Config::new().with_attack_time(1.0));

        // Empty the envelope
        for _ in 0..10 {
            ad.pop();
        }

        assert_relative_eq!(ad.pop(), 0.0);
    }

    #[test]
    fn complete_envelope_should_be_able_to_start_again() {
        const SAMPLE_RATE: f32 = 10.0;
        let mut ad = Ad::new(SAMPLE_RATE);
        let config = Config::new().with_attack_time(1.0);

        ad.trigger(config);

        // Empty the envelope
        for _ in 0..11 {
            ad.pop();
        }

        ad.trigger(config);

        // Should be rising again
        let mut previous = ad.pop();
        for _ in 0..5 {
            let new = ad.pop();
            assert!(new > previous, "{} !> {}", new, previous);
            previous = new;
        }
    }

    #[test]
    fn rising_envelope_should_keep_rising_when_retriggered() {
        const SAMPLE_RATE: f32 = 10.0;
        let mut ad = Ad::new(SAMPLE_RATE);
        let config = Config::new().with_attack_time(1.0).with_decay_time(1.0);

        ad.trigger(config);

        // Empty half of the rising edge
        for _ in 0..4 {
            ad.pop();
        }

        // Should continue rising without getting reset to zero
        let mut previous = ad.pop();
        ad.trigger(config);
        for _ in 0..5 {
            let new = ad.pop();
            assert!(new > previous, "{} !> {}", new, previous);
            previous = new;
        }
    }

    #[test]
    fn falling_envelope_should_immediately_start_rising_when_triggered() {
        const SAMPLE_RATE: f32 = 10.0;
        let mut ad = Ad::new(SAMPLE_RATE);
        let config = Config::new().with_attack_time(1.0);

        ad.trigger(config);

        // Empty the envelope up to half of the falling edge
        for _ in 0..14 {
            ad.pop();
        }

        // Should start rising again without getting reset to zero
        let mut previous = ad.pop();
        ad.trigger(config);
        for _ in 0..5 {
            let new = ad.pop();
            assert!(new > previous, "{} !> {}", new, previous);
            previous = new;
        }
    }

    #[test]
    fn envelope_set_to_lineary_should_have_roughly_linear_progress() {
        const SAMPLE_RATE: f32 = 10.0;
        let mut ad = Ad::new(SAMPLE_RATE);

        ad.trigger(
            Config::new()
                .with_attack_time(1.0)
                .with_attack_ratio(100000.0)
                .with_decay_time(2.0)
                .with_decay_ratio(100000.0),
        );

        // Should be going up in equal steps
        let mut previous = ad.pop();
        let step = previous;
        for _ in 0..9 {
            let new = ad.pop();
            assert_relative_eq!(new - previous, step, epsilon = 0.03);
            previous = new;
        }

        // Confirm it reached the top
        assert_relative_eq!(previous, 1.0, epsilon = 0.0001);

        // Should be going down in equal steps
        let mut previous = ad.pop();
        let step = previous - 1.0;
        for _ in 0..20 {
            let new = ad.pop();
            assert_relative_eq!(new - previous, step, epsilon = 0.03);
            previous = new;
        }

        // Confirm it reached the bottom
        assert_relative_eq!(previous, 0.0, epsilon = 0.0001);
    }

    #[test]
    fn envelope_set_to_logarithmic_should_have_roughly_logarithmic_progress() {
        const SAMPLE_RATE: f32 = 10.0;
        let mut ad = Ad::new(SAMPLE_RATE);

        ad.trigger(
            Config::new()
                .with_attack_time(1.0)
                .with_attack_ratio(0.000000001)
                .with_decay_time(2.0)
                .with_decay_ratio(0.000000001),
        );

        // Should be going up in decreasing steps
        let mut previous = ad.pop();
        let mut step = previous;
        for _ in 0..8 {
            let new = ad.pop();
            let new_step = new - previous;
            assert!(new_step < step / 2.0);
            previous = new;
            step = new_step;
        }

        // Confirm it reached the top
        assert_relative_eq!(previous, 1.0, epsilon = 0.0001);

        // Should be going down in increasing steps
        let mut previous = ad.pop();
        let mut step = previous - 1.0;
        for _ in 0..20 {
            let new = ad.pop();
            let new_step = new - previous;
            assert!(new_step > step * 2.0);
            previous = new;
            step = new_step;
        }

        // Confirm it reached the bottom
        assert_relative_eq!(previous, 0.0, epsilon = 0.0001);
    }

    proptest! {
        #[ignore]
        #[test]
        fn triggered_envelope_should_rise_and_fall_in_proptest(
            attack_rate in 0.001f32..600.0,
            attack_ratio in 0.000000001f32..100000.0,
            decay_rate in 0.001f32..600.0,
            decay_ratio in 0.000000001f32..100000.0,
        ) {
            const SAMPLE_RATE: f32 = 48000.0;
            let config = Config::new()
                .with_attack_time(attack_rate)
                .with_attack_ratio(attack_ratio)
                .with_decay_time(decay_rate)
                .with_decay_ratio(decay_ratio);
            assert_rising(SAMPLE_RATE, config);
            assert_falling(SAMPLE_RATE, config);
        }
    }

    fn assert_rising(sample_rate: f32, config: Config) {
        const RELATIVE_TOLERATION: f32 = 0.02;
        const ABSOLUTE_TOLERATION: usize = 2;

        let samples_rising = sample_rate * config.attack_time;

        let mut ad = Ad::new(sample_rate);
        ad.trigger(config);

        let leading_period = {
            let mut leading_period = (samples_rising * (1.0 - RELATIVE_TOLERATION / 2.0)) as usize;
            if leading_period >= 1 {
                leading_period -= ABSOLUTE_TOLERATION;
            }
            leading_period
        };
        (0..leading_period).for_each(|_| {
            ad.pop();
        });

        let considered_period =
            (samples_rising * RELATIVE_TOLERATION) as usize + ABSOLUTE_TOLERATION;
        let top = (0..=considered_period).find(|_| relative_eq!(ad.pop(), 1.0));
        assert!(
            top.is_some(),
            "Envelope has not reached the top when expected, last sample: {}",
            ad.pop()
        );
    }

    fn assert_falling(sample_rate: f32, config: Config) {
        const RELATIVE_TOLERATION: f32 = 0.08;
        const ABSOLUTE_TOLERATION: usize = 2;

        let samples_rising = sample_rate * config.attack_time;
        let samples_falling = sample_rate * config.decay_time;

        let mut ad = Ad::new(sample_rate);
        ad.trigger(config);

        let increasing_period =
            (samples_rising * (1.0 + RELATIVE_TOLERATION)) as usize + ABSOLUTE_TOLERATION;
        let top = (0..=increasing_period).find(|_| relative_eq!(ad.pop(), 1.0));
        assert!(
            top.is_some(),
            "Has not reached the top, last sample: {}",
            ad.pop()
        );

        let leading_period = (samples_falling * (1.0 - RELATIVE_TOLERATION / 2.0)) as usize;
        (0..leading_period).for_each(|_| {
            ad.pop();
        });

        let considered_period =
            (samples_falling * RELATIVE_TOLERATION) as usize + ABSOLUTE_TOLERATION;
        let bottom = (0..=considered_period).find(|_| relative_eq!(ad.pop(), 0.0));
        assert!(
            bottom.is_some(),
            "Envelope has not reached the bottom when expected, last sample: {}",
            ad.pop()
        );
    }
}

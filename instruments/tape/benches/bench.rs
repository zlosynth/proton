use criterion::{black_box, criterion_group, criterion_main, Criterion};

use proton_control::input_snapshot::{Cv as CvSnapshot, InputSnapshot};
use proton_instruments_tape::{Instrument, Rand};

const SAMPLE_RATE: u32 = 44_100;

struct ThreadRand;

impl Rand for ThreadRand {
    fn generate(&mut self) -> u16 {
        use rand::Rng;
        rand::thread_rng().gen()
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("instrument", |b| {
        let mut instrument = Instrument::new(SAMPLE_RATE);
        let mut buffer = [0.0; 32];
        b.iter(|| {
            instrument.update_control(InputSnapshot {
                cv: [
                    CvSnapshot { value: 0.5 },
                    CvSnapshot { value: 0.5 },
                    CvSnapshot { value: 0.5 },
                    CvSnapshot { value: 0.5 },
                    CvSnapshot { value: 0.5 },
                ],
            });
            instrument.process(black_box(&mut buffer), &mut ThreadRand);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

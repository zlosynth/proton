use proton_peripherals::cv_input::CvInput;

use crate::input_snapshot::{Cv as CvSnapshot, InputSnapshot, Pot as PotSnapshot};

pub struct InputProcessor<A1, A2, P, CI1, CI2, CI3, CI4, CI5> {
    pot: P,
    cv_input_1: CI1,
    cv_input_2: CI2,
    cv_input_3: CI3,
    cv_input_4: CI4,
    cv_input_5: CI5,
    adc_1: A1,
    adc_2: A2,
}

impl<A1, A2, P, CI1, CI2, CI3, CI4, CI5> InputProcessor<A1, A2, P, CI1, CI2, CI3, CI4, CI5>
where
    P: CvInput<Adc = A1>,
    CI1: CvInput<Adc = A2>,
    CI2: CvInput<Adc = A1>,
    CI3: CvInput<Adc = A2>,
    CI4: CvInput<Adc = A1>,
    CI5: CvInput<Adc = A2>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adc_1: A1,
        adc_2: A2,
        pot: P,
        cv_input_1: CI1,
        cv_input_2: CI2,
        cv_input_3: CI3,
        cv_input_4: CI4,
        cv_input_5: CI5,
    ) -> Self {
        Self {
            adc_1,
            adc_2,
            pot,
            cv_input_1,
            cv_input_2,
            cv_input_3,
            cv_input_4,
            cv_input_5,
        }
    }

    pub fn update(&mut self) -> InputSnapshot {
        self.sample();
        self.snapshot()
    }

    fn sample(&mut self) {
        self.pot.start_sampling(&mut self.adc_1);
        self.cv_input_1.start_sampling(&mut self.adc_2);
        self.pot.finish_sampling(&mut self.adc_1);
        self.cv_input_1.finish_sampling(&mut self.adc_2);

        self.cv_input_2.start_sampling(&mut self.adc_1);
        self.cv_input_3.start_sampling(&mut self.adc_2);
        self.cv_input_2.finish_sampling(&mut self.adc_1);
        self.cv_input_3.finish_sampling(&mut self.adc_2);

        self.cv_input_4.start_sampling(&mut self.adc_1);
        self.cv_input_5.start_sampling(&mut self.adc_2);
        self.cv_input_4.finish_sampling(&mut self.adc_1);
        self.cv_input_5.finish_sampling(&mut self.adc_2);
    }

    fn snapshot(&mut self) -> InputSnapshot {
        InputSnapshot {
            pot: PotSnapshot {
                value: self.pot.value(),
            },
            cv: [
                CvSnapshot {
                    value: self.cv_input_1.value(),
                },
                CvSnapshot {
                    value: self.cv_input_2.value(),
                },
                CvSnapshot {
                    value: self.cv_input_3.value(),
                },
                CvSnapshot {
                    value: self.cv_input_4.value(),
                },
                CvSnapshot {
                    value: self.cv_input_5.value(),
                },
            ],
        }
    }
}

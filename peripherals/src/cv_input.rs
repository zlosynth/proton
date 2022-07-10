pub trait CvInput {
    type Adc;
    fn start_sampling(&mut self, adc: &mut Self::Adc);
    fn finish_sampling(&mut self, adc: &mut Self::Adc);
    fn value(&self) -> f32;
}

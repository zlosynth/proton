mod attributes;
mod command;
mod execute;
mod populate;
mod state;

pub struct Instrument {
    pub(crate) post_gain: f32,
}

impl Instrument {
    pub fn new(_sample_rate: u32) -> Self {
        Self { post_gain: 1.0 }
    }
}

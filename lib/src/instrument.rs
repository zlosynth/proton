use crate::core::engine::Engine;

pub struct Instrument {
    engine: Engine,
}

#[allow(clippy::new_without_default)]
impl Instrument {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    pub fn set_control(&mut self, value: f32) {
        self.engine.set_control(value);
    }

    pub fn tick(&mut self) {
        self.engine.tick();
    }

    pub fn get_audio(&self) -> [f32; 32] {
        self.engine.get_audio()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_initialized() {
        let _instrument = Instrument::new();
    }

    #[test]
    fn set_arbitrary_control_tick_and_get() {
        let mut instrument = Instrument::new();
        instrument.set_control(0.5);
        instrument.tick();
        let _audio: [f32; 32] = instrument.get_audio();
    }
}

use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::BinaryColor;

use crate::core::engine::Engine;
use crate::display::Display;

pub struct Instrument<D> {
    engine: Engine,
    display: Option<Display<D>>,
}

#[allow(clippy::new_without_default)]
impl<D> Instrument<D> {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
            display: None,
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

#[allow(clippy::new_without_default)]
impl<D> Instrument<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    pub fn register_display(&mut self, display: D) {
        self.display = Some(Display::new(display));
    }

    pub fn update_display(&mut self) {
        self.display.as_mut().unwrap().update();
    }

    pub fn mut_display(&mut self) -> &mut D {
        &mut self.display.as_mut().unwrap().display
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_initialized() {
        let _instrument: Instrument<()> = Instrument::new();
    }

    #[test]
    fn set_arbitrary_control_tick_and_get() {
        let mut instrument: Instrument<()> = Instrument::new();
        instrument.set_control(0.5);
        instrument.tick();
        let _audio: [f32; 32] = instrument.get_audio();
    }
}

use alloc::vec;

use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::BinaryColor;

use crate::core::engine::Engine;
use crate::display::Display;
use crate::model::store::{Attribute, Module, Store};

pub struct Instrument<D> {
    engine: Engine,
    display: Option<Display<D>>,
    store: Store,
}

#[allow(clippy::new_without_default)]
impl<D> Instrument<D> {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
            display: None,
            store: Store {
                modules: vec![
                    Module {
                        name: "ENV",
                        index: 2,
                        attributes: vec![
                            Attribute {
                                name: "A",
                                connected: true,
                                value: "100% +0.50 ",
                            },
                            Attribute {
                                name: "B",
                                connected: false,
                                value: "0.58       ",
                            },
                        ],
                        selected_attribute: 1,
                    },
                    Module {
                        name: "MIX",
                        index: 1,
                        attributes: vec![],
                        selected_attribute: 0,
                    },
                    Module {
                        name: "OSC",
                        index: 3,
                        attributes: vec![],
                        selected_attribute: 0,
                    },
                    Module {
                        name: ">CV",
                        index: 9,
                        attributes: vec![],
                        selected_attribute: 0,
                    },
                    Module {
                        name: "<AO",
                        index: 1,
                        attributes: vec![],
                        selected_attribute: 0,
                    },
                    Module {
                        name: "FOL",
                        index: 3,
                        attributes: vec![],
                        selected_attribute: 0,
                    },
                    Module {
                        name: "DIS",
                        index: 3,
                        attributes: vec![],
                        selected_attribute: 0,
                    },
                ],
                selected_module: 0,
            },
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
        self.display.as_mut().unwrap().update(&self.store);
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

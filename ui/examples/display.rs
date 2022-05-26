use std::{thread, time::Duration};

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use heapless::Vec;

use proton_ui::action::Action;
use proton_ui::display::*;
use proton_ui::reducer;
use proton_ui::state::*;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Fonts", &output_settings);

    let mut state = State {
        title: "Proton",
        attributes: Vec::from_slice(&[
            Attribute {
                name: "scale",
                value: Value::Select(ValueSelect {
                    available: Vec::from_slice(&["major", "minor"]).unwrap(),
                    selected: 0,
                }),
            },
            Attribute {
                name: "root",
                value: Value::Select(ValueSelect {
                    available: Vec::from_slice(&["c", "c#"]).unwrap(),
                    selected: 1,
                }),
            },
            Attribute {
                name: "speed",
                value: Value::F32(ValueF32 {
                    value: 0.1,
                    step: 0.01,
                }),
            },
        ])
        .unwrap(),
        selected_attribute: 1,
    };

    let view = (&state).into();
    draw(&mut display, &view)?;
    window.update(&display);

    'running: loop {
        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running Ok(()),
                SimulatorEvent::KeyDown { keycode, .. } => {
                    let action = match keycode {
                        Keycode::Left => Some(Action::BetaDown),
                        Keycode::Right => Some(Action::BetaUp),
                        Keycode::Up => Some(Action::AlphaUp),
                        Keycode::Down => Some(Action::AlphaDown),
                        _ => None,
                    };
                    if let Some(action) = action {
                        reducer::reduce(action, &mut state);
                        let view = (&state).into();
                        draw(&mut display, &view)?;
                    }
                }
                _ => {}
            }
        }

        window.update(&display);

        thread::sleep(Duration::from_millis(50));
    }
}

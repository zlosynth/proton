use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use proton_ui::display::*;
use proton_ui::view::*;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));

    let state = View {
        title: "Protonq",
        attributes: [
            Some(Attribute {
                name: "scale",
                value: Value::Str("phrygian"),
            }),
            Some(Attribute {
                name: "root",
                value: Value::Str("c#"),
            }),
            Some(Attribute {
                name: "speed",
                value: Value::F32(1.0),
            }),
            None,
        ],
        selected_attribute: 1,
    };
    draw(&mut display, &state)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}

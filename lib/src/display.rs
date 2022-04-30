use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::Text,
};
use embedded_graphics_core::draw_target::DrawTarget;

pub struct Display<D> {
    pub display: D,
}

impl<D> Display<D>
where
    D: DrawTarget<Color = BinaryColor>,
{
    pub fn new(display: D) -> Self {
        Self { display }
    }

    pub fn update(&mut self) {
        Line::new(Point::new(0, 0), Point::new(0, 32))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut self.display)
            .ok()
            .unwrap();

        for (i, x) in [" ENV1 ", " MIX2 ", " OSC1 ", " CV4 ", " AUD5 "]
            .iter()
            .enumerate()
        {
            if i == 1 {
                let style = MonoTextStyleBuilder::new()
                    .font(&FONT_6X12)
                    .text_color(BinaryColor::Off)
                    .background_color(BinaryColor::On)
                    .build();
                Text::new(x, Point::new(5, 12 * (i + 1) as i32 - 1), style)
                    .draw(&mut self.display)
                    .ok()
                    .unwrap();
            } else {
                Text::new(
                    x,
                    Point::new(5, 12 * (i + 1) as i32 - 1),
                    MonoTextStyle::new(&FONT_6X12, BinaryColor::On),
                )
                .draw(&mut self.display)
                .ok()
                .unwrap();
            }
        }

        for (i, x) in [
            " >A 0.01 ",
            "  D 0.0 ",
            "  S 0.0      ",
            " >R 2.0 ",
            " <O 100% ",
        ]
        .iter()
        .enumerate()
        {
            if i == 2 {
                let style = MonoTextStyleBuilder::new()
                    .font(&FONT_6X12)
                    .text_color(BinaryColor::Off)
                    .background_color(BinaryColor::On)
                    .build();
                Text::new(x, Point::new(45, 12 * (i + 1) as i32 - 1), style)
                    .draw(&mut self.display)
                    .ok()
                    .unwrap();
            } else {
                Text::new(
                    x,
                    Point::new(45, 12 * (i + 1) as i32 - 1),
                    MonoTextStyle::new(&FONT_6X12, BinaryColor::On),
                )
                .draw(&mut self.display)
                .ok()
                .unwrap();
            }
        }
    }
}

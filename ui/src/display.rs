use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};

use crate::view::*;

pub const DISPLAY_WIDTH: u32 = 128;
pub const DISPLAY_HEIGHT: u32 = 64;
const HEADER_HEIGHT: u32 = 15;
const HEADER_LINE: u32 = 1;
const ATTRIBUTE_HEIGHT: u32 = 12;
const ATTRIBUTE_PADDING: u32 = 5;
const FONT_WIDTH: u32 = 6;
const FONT_HEIGHT_ABOVE_LINE: u32 = 8;

const USIZE_TO_STR: [&str; 101] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
    "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32",
    "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48",
    "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64",
    "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80",
    "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96",
    "97", "98", "99", "100",
];

pub fn draw<D>(target: &mut D, view: &View) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    reset_screen(target)?;

    draw_status_bar(target, view.title)?;

    for (i, attribute) in view
        .attributes
        .iter()
        .filter_map(|a| a.as_ref())
        .enumerate()
    {
        let highlighted = i == view.selected_attribute;
        draw_attribute(target, attribute, highlighted, i)?;
    }

    Ok(())
}

fn reset_screen<D>(target: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    draw_rectangle(
        target,
        Point::new(0, 0),
        Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT),
        BinaryColor::Off,
    )
}

fn draw_status_bar<D>(target: &mut D, text: &'static str) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    draw_rectangle(
        target,
        Point::new(0, HEADER_HEIGHT as i32 - HEADER_LINE as i32),
        Size::new(DISPLAY_WIDTH, HEADER_LINE),
        BinaryColor::On,
    )?;

    let x = x_for_centered_text(text);
    let y = FONT_HEIGHT_ABOVE_LINE as i32;
    draw_text(target, text, Point::new(x, y), BinaryColor::On)?;

    Ok(())
}

fn draw_attribute<D>(
    target: &mut D,
    attribute: &Attribute,
    highlighted: bool,
    position: usize,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let (text, background) = if highlighted {
        (BinaryColor::Off, BinaryColor::On)
    } else {
        (BinaryColor::On, BinaryColor::Off)
    };

    let y = position as i32 * ATTRIBUTE_HEIGHT as i32 + HEADER_HEIGHT as i32;

    draw_rectangle(
        target,
        Point::new(0, y),
        Size::new(DISPLAY_WIDTH, ATTRIBUTE_HEIGHT),
        background,
    )?;
    draw_text(
        target,
        attribute.name,
        Point::new(ATTRIBUTE_PADDING as i32, y + FONT_HEIGHT_ABOVE_LINE as i32),
        text,
    )?;

    match &attribute.value {
        Value::Str(value) => {
            let x = x_for_right_justified_text_with_offset(value, 0);
            draw_text(
                target,
                value,
                Point::new(x, y + FONT_HEIGHT_ABOVE_LINE as i32),
                text,
            )?;
        }
        Value::F32(value) => {
            let number = (value * 100.0) as usize;
            let number = USIZE_TO_STR[number];

            let y = y + FONT_HEIGHT_ABOVE_LINE as i32;

            let x = x_for_right_justified_text_with_offset(number, 1);
            draw_text(target, number, Point::new(x, y), text)?;

            let x = x_for_right_justified_text_with_offset("%", 0);
            draw_text(target, "%", Point::new(x, y), text)?;
        }
    }

    Ok(())
}

fn x_for_right_justified_text_with_offset(text: &'static str, offset: usize) -> i32 {
    let text_width = (text.len() + offset) as i32 * FONT_WIDTH as i32;
    DISPLAY_WIDTH as i32 - text_width - ATTRIBUTE_PADDING as i32
}

fn draw_text<D>(
    target: &mut D,
    text: &'static str,
    position: Point,
    color: BinaryColor,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    Text::new(text, position, MonoTextStyle::new(&FONT_6X12, color)).draw(target)?;
    Ok(())
}

fn draw_rectangle<D>(
    target: &mut D,
    start: Point,
    size: Size,
    color: BinaryColor,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    Rectangle::new(start, size)
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(target)
}

fn x_for_centered_text(text: &'static str) -> i32 {
    let text_width = text.len() as i32 * FONT_WIDTH as i32;
    DISPLAY_WIDTH as i32 / 2 - text_width / 2
}

#[allow(unused_imports)]
use micromath::F32Ext;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::Text,
};
use embedded_graphics_core::draw_target::DrawTarget;

use crate::store::{Attribute, Module, Store};

const PADDING_LEFT: i32 = 5;
const FONT_HEIGHT: i32 = 12;
const FONT_WIDTH: i32 = 6;
const MODULES_PER_PAGE: usize = 5;
const ATTRIBUTES_PER_PAGE: usize = 5;
const DISPLAY_WIDTH: i32 = 128;
const DISPLAY_HEIGHT: i32 = 64;

const I32_TO_STR: [&str; 100] = [
    "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15",
    "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
    "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47",
    "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63",
    "64", "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79",
    "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95",
    "96", "97", "98", "99",
];

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

    pub fn update(&mut self, store: &Store) {
        let modules_page = selected_module_to_page(store.selected_module);

        let (list_start, list_stop) = range_for_modules_page(&store.modules, modules_page);
        for (i, module) in store.modules[list_start..=list_stop].iter().enumerate() {
            let selected = list_start + i == store.selected_module;
            draw_module(module, i, selected, &mut self.display);

            if selected {
                let attributes_page = selected_attribute_to_page(module.selected_attribute);
                let (list_start, list_stop) =
                    range_for_attributes_page(&module.attributes, attributes_page);
                for (i, attribute) in module.attributes[list_start..=list_stop].iter().enumerate() {
                    let selected = list_start + i == module.selected_attribute;
                    draw_attribute(attribute, i, selected, &mut self.display);
                }
                draw_attributes_scroll_bar(&module.attributes, attributes_page, &mut self.display);
            }
        }

        draw_modules_scroll_bar(&store.modules, modules_page, &mut self.display);
    }
}

fn range_for_modules_page(modules: &[Module], modules_page: usize) -> (usize, usize) {
    let list_start = modules_page * MODULES_PER_PAGE;
    let list_stop = usize::min((modules_page + 1) * MODULES_PER_PAGE, modules.len()) - 1;
    (list_start, list_stop)
}

fn draw_module<D: DrawTarget<Color = BinaryColor>>(
    module: &Module,
    index: usize,
    selected: bool,
    display: &mut D,
) {
    let highlight = if selected {
        Highlight::Yes
    } else {
        Highlight::No
    };

    let x = PADDING_LEFT;
    let y = FONT_HEIGHT * (index + 1) as i32 - 1;
    draw_text(module.name, x, y, highlight, display);

    let name_width = FONT_WIDTH * 3;
    let x = x + name_width;
    draw_text(I32_TO_STR[module.index], x, y, highlight, display);
}

fn draw_modules_scroll_bar<D: DrawTarget<Color = BinaryColor>>(
    modules: &[Module],
    modules_page: usize,
    display: &mut D,
) {
    let sections = (modules.len() as f32 / MODULES_PER_PAGE as f32).ceil() as i32;
    let section_height = DISPLAY_HEIGHT / sections;

    let line_start = modules_page as i32 * section_height;

    let is_last_section = modules_page == sections as usize - 1;
    let line_stop = if is_last_section {
        DISPLAY_HEIGHT
    } else {
        (modules_page + 1) as i32 * section_height - 1
    };

    Line::new(Point::new(0, line_start), Point::new(0, line_stop))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok()
        .unwrap();
}

fn selected_module_to_page(selected_module: usize) -> usize {
    (selected_module as f32 / MODULES_PER_PAGE as f32).floor() as usize
}

fn draw_attribute<D: DrawTarget<Color = BinaryColor>>(
    attribute: &Attribute,
    index: usize,
    selected: bool,
    display: &mut D,
) {
    let highlight = if selected {
        Highlight::Yes
    } else {
        Highlight::No
    };

    let x = PADDING_LEFT + 34;
    let y = FONT_HEIGHT * (index + 1) as i32 - 1;

    if attribute.connected {
        draw_text(">", x, y, highlight, display);
    } else {
        draw_text(" ", x, y, highlight, display);
    }

    let x = x + FONT_WIDTH;
    draw_text(attribute.name, x, y, highlight, display);

    let x = x + FONT_WIDTH;
    draw_text(" ", x, y, highlight, display);

    let x = x + FONT_WIDTH;
    draw_text(attribute.value, x, y, highlight, display);
}

fn selected_attribute_to_page(selected_attribute: usize) -> usize {
    (selected_attribute as f32 / ATTRIBUTES_PER_PAGE as f32).floor() as usize
}

fn range_for_attributes_page(attributes: &[Attribute], attributes_page: usize) -> (usize, usize) {
    let list_start = attributes_page * ATTRIBUTES_PER_PAGE;
    let list_stop = usize::min(
        (attributes_page + 1) * ATTRIBUTES_PER_PAGE,
        attributes.len(),
    ) - 1;
    (list_start, list_stop)
}

fn draw_attributes_scroll_bar<D: DrawTarget<Color = BinaryColor>>(
    attributes: &[Attribute],
    attributes_page: usize,
    display: &mut D,
) {
    let sections = (attributes.len() as f32 / ATTRIBUTES_PER_PAGE as f32).ceil() as i32;
    let section_height = DISPLAY_HEIGHT / sections;

    let line_start = attributes_page as i32 * section_height;

    let is_last_section = attributes_page == sections as usize - 1;
    let line_stop = if is_last_section {
        DISPLAY_HEIGHT
    } else {
        (attributes_page + 1) as i32 * section_height - 1
    };

    let x = DISPLAY_WIDTH - 1;
    Line::new(Point::new(x, line_start), Point::new(x, line_stop))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok()
        .unwrap();
}

#[derive(Clone, Copy)]
enum Highlight {
    Yes,
    No,
}

fn draw_text<D: DrawTarget<Color = BinaryColor>>(
    text: &'static str,
    x: i32,
    y: i32,
    highlighted: Highlight,
    display: &mut D,
) {
    match highlighted {
        Highlight::Yes => {
            let style = MonoTextStyleBuilder::new()
                .font(&FONT_6X12)
                .text_color(BinaryColor::Off)
                .background_color(BinaryColor::On)
                .build();
            Text::new(text, Point::new(x, y), style)
                .draw(display)
                .ok()
                .unwrap();
        }
        Highlight::No => {
            Text::new(
                text,
                Point::new(x, y),
                MonoTextStyle::new(&FONT_6X12, BinaryColor::On),
            )
            .draw(display)
            .ok()
            .unwrap();
        }
    }
}

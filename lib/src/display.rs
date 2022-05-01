#[allow(unused_imports)]
use micromath::F32Ext;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_graphics_core::draw_target::DrawTarget;

use crate::model::state::{Attribute, Module, Patch, State, View};

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

    pub fn update<NI, CI, PI>(&mut self, state: &State<NI, CI, PI>) {
        draw_blank(&mut self.display);

        match state.view {
            View::Modules => self.update_modules(state),
            View::Patches => self.update_patches(state),
        }
    }

    fn update_modules<NI, CI, PI>(&mut self, state: &State<NI, CI, PI>) {
        let modules_page = selected_module_to_page(state.selected_module);

        let (list_start, list_stop) = range_for_modules_page(&state.modules, modules_page);
        for (i, module) in state.modules[list_start..=list_stop].iter().enumerate() {
            let selected = list_start + i == state.selected_module;
            draw_module(module, i, selected, &mut self.display);

            if selected && !module.attributes.is_empty() {
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

        draw_modules_scroll_bar(&state.modules, modules_page, &mut self.display);
    }

    fn update_patches<NI, CI, PI>(&mut self, state: &State<NI, CI, PI>) {
        for (i, patch) in state.patches.iter().enumerate() {
            draw_patch(patch, i, &mut self.display);
        }
    }
}

fn range_for_modules_page<NI, CI, PI>(
    modules: &[Module<NI, CI, PI>],
    modules_page: usize,
) -> (usize, usize) {
    let list_start = modules_page * MODULES_PER_PAGE;
    let list_stop = usize::min((modules_page + 1) * MODULES_PER_PAGE, modules.len()) - 1;
    (list_start, list_stop)
}

fn draw_module<NI, CI, PI, D: DrawTarget<Color = BinaryColor>>(
    module: &Module<NI, CI, PI>,
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

fn draw_modules_scroll_bar<NI, CI, PI, D: DrawTarget<Color = BinaryColor>>(
    modules: &[Module<NI, CI, PI>],
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

fn draw_attribute<CI, PI, D: DrawTarget<Color = BinaryColor>>(
    attribute: &Attribute<CI, PI>,
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

    let x = x + FONT_WIDTH * attribute.name.len() as i32;
    draw_text(" ", x, y, highlight, display);

    let x = x + FONT_WIDTH;
    draw_text(attribute.value, x, y, highlight, display);
}

fn selected_attribute_to_page(selected_attribute: usize) -> usize {
    (selected_attribute as f32 / ATTRIBUTES_PER_PAGE as f32).floor() as usize
}

fn range_for_attributes_page<CI, PI>(
    attributes: &[Attribute<CI, PI>],
    attributes_page: usize,
) -> (usize, usize) {
    let list_start = attributes_page * ATTRIBUTES_PER_PAGE;
    let list_stop = usize::min(
        (attributes_page + 1) * ATTRIBUTES_PER_PAGE,
        attributes.len(),
    ) - 1;
    (list_start, list_stop)
}

fn draw_attributes_scroll_bar<CI, PI, D: DrawTarget<Color = BinaryColor>>(
    attributes: &[Attribute<CI, PI>],
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

fn draw_patch<CI, PI, D: DrawTarget<Color = BinaryColor>>(
    patch: &Patch<CI, PI>,
    index: usize,
    display: &mut D,
) {
    let x = PADDING_LEFT;
    let y = FONT_HEIGHT * (index + 1) as i32 - 1;

    draw_text(patch.source_module_name, x, y, Highlight::No, display);
    let x = x + FONT_WIDTH * patch.source_module_name.len() as i32;

    let index = I32_TO_STR[patch.source_module_index];
    draw_text(index, x, y, Highlight::No, display);
    let x = x + FONT_WIDTH * index.len() as i32;

    draw_text(".", x, y, Highlight::No, display);
    let x = x + FONT_WIDTH;

    draw_text(patch.source_attribute_name, x, y, Highlight::No, display);
    let x = x + FONT_WIDTH * patch.source_attribute_name.len() as i32;

    let x = x + 2;
    draw_text("-", x, y, Highlight::No, display);
    let x = x + FONT_WIDTH + 2;

    draw_text(patch.destination_module_name, x, y, Highlight::No, display);
    let x = x + FONT_WIDTH * patch.destination_module_name.len() as i32;

    let index = I32_TO_STR[patch.destination_module_index];
    draw_text(index, x, y, Highlight::No, display);
    let x = x + FONT_WIDTH * index.len() as i32;

    draw_text(".", x, y, Highlight::No, display);
    let x = x + FONT_WIDTH;

    draw_text(patch.destination_attribute_name, x, y, Highlight::No, display);
    let x = x + FONT_WIDTH * patch.destination_attribute_name.len() as i32;
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
    let style = match highlighted {
        Highlight::Yes => MonoTextStyleBuilder::new()
            .font(&FONT_6X12)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build(),
        Highlight::No => MonoTextStyleBuilder::new()
            .font(&FONT_6X12)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build(),
    };
    Text::new(text, Point::new(x, y), style)
        .draw(display)
        .ok()
        .unwrap();
}

fn draw_blank<D: DrawTarget<Color = BinaryColor>>(display: &mut D) {
    Rectangle::new(
        Point::new(0, 0),
        Size::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
    .draw(display)
    .ok()
    .unwrap();
}

#[allow(unused_imports)]
use micromath::F32Ext;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle, Triangle},
    text::Text,
};
use embedded_graphics_core::draw_target::DrawTarget;

use crate::model::state::{Attribute, Destination, Module, Patch, Socket, Source, State, View};

const PADDING: i32 = 5;
const FONT_HEIGHT: i32 = 12;
const FONT_WIDTH: i32 = 6;
const LINES_PER_PAGE: usize = 5;
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
            View::PatchEdit => self.update_patch_edit(state),
        }
    }

    fn update_modules<NI, CI, PI>(&mut self, state: &State<NI, CI, PI>) {
        let modules_page = selected_item_to_page(state.selected_module);
        let (list_start, list_stop) = range_for_items_page(state.modules.len(), modules_page);

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
                draw_scroll_bar(
                    Right,
                    module.attributes.len(),
                    attributes_page,
                    &mut self.display,
                );
            }
        }

        draw_scroll_bar(Left, state.modules.len(), modules_page, &mut self.display);
    }

    fn update_patches<NI, CI, PI>(&mut self, state: &State<NI, CI, PI>) {
        let patches_page = selected_item_to_page(state.selected_patch);
        let (list_start, list_stop) = range_for_items_page(state.patches.len(), patches_page);

        for (i, patch) in state.patches[list_start..=list_stop].iter().enumerate() {
            let selected = list_start + i == state.selected_patch;
            draw_patch(patch, i, selected, &mut self.display);
        }

        draw_scroll_bar(Left, state.patches.len(), patches_page, &mut self.display);
    }

    fn update_patch_edit<NI, CI, PI>(&mut self, state: &State<NI, CI, PI>) {
        let patch = &state.patches[state.selected_patch];
        draw_destination(&patch.destination, 0, false, &mut self.display);
        draw_arrow_left(0, false, &mut self.display);

        let sources_page = selected_item_to_page(state.patch_edit_selected_source);
        let (list_start, list_stop) = range_for_items_page(state.patch_edit_sources.len(), sources_page);

        for (i, source) in state.patch_edit_sources[list_start..=list_stop].iter().enumerate() {
            let selected = list_start + i == state.patch_edit_selected_source;
            draw_source(Some(source), i, selected, &mut self.display);
        }
    }
}

fn range_for_items_page(
    items_len: usize,
    items_page: usize,
) -> (usize, usize) {
    let list_start = items_page * LINES_PER_PAGE;
    let list_stop = usize::min((items_page + 1) * LINES_PER_PAGE, items_len) - 1;
    (list_start, list_stop)
}

fn draw_module<NI, CI, PI, D: DrawTarget<Color = BinaryColor>>(
    module: &Module<NI, CI, PI>,
    index: usize,
    selected: bool,
    display: &mut D,
) {
    let x = PADDING;
    let y = FONT_HEIGHT * (index + 1) as i32 - 1;

    let mut cursor = Cursor::new(x, y, display).with_highlight(selected);
    cursor.write(module.name);
    cursor.write(I32_TO_STR[module.index]);
}

enum Side {
    Left,
    Right,
}
use Side::*;

fn draw_scroll_bar<D: DrawTarget<Color = BinaryColor>>(
    side: Side,
    items_len: usize,
    items_page: usize,
    display: &mut D,
) {
    let sections = (items_len as f32 / LINES_PER_PAGE as f32).ceil() as i32;
    let section_height = DISPLAY_HEIGHT / sections;

    let line_start = items_page as i32 * section_height;

    let is_last_section = items_page == sections as usize - 1;
    let line_stop = if is_last_section {
        DISPLAY_HEIGHT
    } else {
        (items_page + 1) as i32 * section_height - 1
    };

    let x = match side {
        Left => 0,
        Right => DISPLAY_WIDTH - 1,
    };
    Line::new(Point::new(x, line_start), Point::new(x, line_stop))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
        .ok()
        .unwrap();
}

fn selected_item_to_page(selected_item: usize) -> usize {
    (selected_item as f32 / LINES_PER_PAGE as f32).floor() as usize
}

fn draw_attribute<CI, PI, D: DrawTarget<Color = BinaryColor>>(
    attribute: &Attribute<CI, PI>,
    index: usize,
    selected: bool,
    display: &mut D,
) {
    let x = PADDING + 34;
    let y = FONT_HEIGHT * (index + 1) as i32 - 1;

    let mut cursor = Cursor::new(x, y, display).with_highlight(selected);

    if attribute.connected {
        match attribute.socket {
            Socket::Consumer(_) => cursor.write(">"),
            Socket::Producer(_) => cursor.write("<"),
        }
    } else {
        cursor.write(" ");
    }

    cursor.write(attribute.name);
    cursor.space_until(DISPLAY_WIDTH - PADDING);
}

fn selected_attribute_to_page(selected_attribute: usize) -> usize {
    (selected_attribute as f32 / LINES_PER_PAGE as f32).floor() as usize
}

fn range_for_attributes_page<CI, PI>(
    attributes: &[Attribute<CI, PI>],
    attributes_page: usize,
) -> (usize, usize) {
    let list_start = attributes_page * LINES_PER_PAGE;
    let list_stop = usize::min((attributes_page + 1) * LINES_PER_PAGE, attributes.len()) - 1;
    (list_start, list_stop)
}

fn draw_patch<CI, PI, D: DrawTarget<Color = BinaryColor>>(
    patch: &Patch<CI, PI>,
    index: usize,
    selected: bool,
    display: &mut D,
) {
    draw_destination(&patch.destination, index, selected, display);
    draw_arrow_left(index, selected, display);
    draw_source(patch.source.as_ref(), index, selected, display);
}

fn draw_destination<CI, D: DrawTarget<Color = BinaryColor>>(
    destination: &Destination<CI>,
    index: usize,
    selected: bool,
    display: &mut D,
) {
    let x = PADDING;
    let y = FONT_HEIGHT * (index + 1) as i32 - 1;

    let mut cursor = Cursor::new(x, y, display).with_highlight(selected);

    cursor.write(destination.module_name);
    cursor.write(I32_TO_STR[destination.module_index]);
    cursor.write(".");
    cursor.write(destination.attribute_name);

    cursor.space_until(DISPLAY_WIDTH / 2 - FONT_WIDTH / 2 - 2);
}

fn draw_source<CI, D: DrawTarget<Color = BinaryColor>>(
    source: Option<&Source<CI>>,
    index: usize,
    selected: bool,
    display: &mut D,
) {
    let x = DISPLAY_WIDTH - PADDING - FONT_WIDTH * 9;
    let y = FONT_HEIGHT * (index + 1) as i32 - 1;

    let mut cursor = Cursor::new(x, y, display).with_highlight(selected);

    if let Some(source) = source.as_ref() {
        cursor.write(source.module_name);
        cursor.write(I32_TO_STR[source.module_index]);
        cursor.write(".");
        cursor.write(source.attribute_name);
    }

    cursor.space_until(DISPLAY_WIDTH - PADDING);
}

fn draw_arrow_left<D: DrawTarget<Color = BinaryColor>>(
    index: usize,
    selected: bool,
    display: &mut D,
) {
    let x = DISPLAY_WIDTH / 2 - FONT_WIDTH / 2 - 2;
    let y = FONT_HEIGHT * (index + 1) as i32 - 1;

    let mut cursor = Cursor::new(x, y, display).with_highlight(selected);

    cursor.arrow_left();
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

struct Cursor<'a, D> {
    x: i32,
    y: i32,
    display: &'a mut D,
    highlighted: bool,
}

impl<'a, D: DrawTarget<Color = BinaryColor>> Cursor<'a, D> {
    fn new(x: i32, y: i32, display: &'a mut D) -> Self {
        Self {
            x,
            y,
            display,
            highlighted: false,
        }
    }

    fn with_highlight(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }

    fn write(&mut self, value: &'static str) {
        draw_text(value, self.x, self.y, self.highlighted, self.display);
        self.x += FONT_WIDTH * value.len() as i32;
    }

    fn space(&mut self, distance: i32) {
        let color = if self.highlighted {
            BinaryColor::On
        } else {
            BinaryColor::Off
        };

        Rectangle::new(
            Point::new(self.x, self.y - 9),
            Size::new(distance as u32, FONT_HEIGHT as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(self.display)
        .ok()
        .unwrap();

        self.x += distance;
    }

    fn space_until(&mut self, x: i32) {
        let distance = x - self.x;
        self.space(distance);
    }

    fn arrow_left(&mut self) {
        const WIDTH: i32 = 10;

        let (background, foreground) = if self.highlighted {
            (BinaryColor::On, BinaryColor::Off)
        } else {
            (BinaryColor::Off, BinaryColor::On)
        };

        Rectangle::new(
            Point::new(self.x, self.y - 9),
            Size::new(WIDTH as u32, FONT_HEIGHT as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(background))
        .draw(self.display)
        .ok()
        .unwrap();

        Triangle::new(
            Point::new(self.x + 2, self.y - 3),
            Point::new(self.x + 4, self.y - 5),
            Point::new(self.x + 4, self.y - 1),
        )
        .into_styled(PrimitiveStyle::with_fill(foreground))
        .draw(self.display)
        .ok()
        .unwrap();

        Line::new(
            Point::new(self.x + 2, self.y - 3),
            Point::new(self.x + 7, self.y - 3),
        )
        .into_styled(PrimitiveStyle::with_stroke(foreground, 1))
        .draw(self.display)
        .ok()
        .unwrap();

        self.x += WIDTH;
    }
}

fn draw_text<D: DrawTarget<Color = BinaryColor>>(
    text: &'static str,
    x: i32,
    y: i32,
    highlighted: bool,
    display: &mut D,
) {
    let style = if highlighted {
        MonoTextStyleBuilder::new()
            .font(&FONT_6X12)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build()
    } else {
        MonoTextStyleBuilder::new()
            .font(&FONT_6X12)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build()
    };
    Text::new(text, Point::new(x, y), style)
        .draw(display)
        .ok()
        .unwrap();
}

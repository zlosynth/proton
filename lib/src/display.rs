#[allow(unused_imports)]
use micromath::F32Ext;

use alloc::vec;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X12, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::Text,
};
use embedded_graphics_core::draw_target::DrawTarget;

use crate::store::{Module, Store};

const PADDING_LEFT: i32 = 5;
const FONT_HEIGHT: i32 = 12;
const MODULES_PER_PAGE: usize = 5;
const DISPLAY_HEIGHT: i32 = 64;

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
        let store = Store {
            modules: vec![
                Module { name: "ENV" },
                Module { name: "MIX" },
                Module { name: "OSC" },
                Module { name: ">CV" },
                Module { name: "<AO" },
                Module { name: "FOL" },
                Module { name: "DIS" },
            ],
            modules_page: 0,
        };

        let (list_start, list_stop) = range_for_modules_page(&store.modules, store.modules_page);
        for (i, module) in store.modules[list_start..=list_stop].iter().enumerate() {
            draw_module(module, i, &mut self.display);
        }

        draw_modules_scroll_bar(&store.modules, store.modules_page, &mut self.display);
    }
}

fn range_for_modules_page(modules: &[Module], modules_page: usize) -> (usize, usize) {
    let list_start = modules_page * MODULES_PER_PAGE;
    let list_stop = usize::min((modules_page + 1) * MODULES_PER_PAGE, modules.len()) - 1;
    (list_start, list_stop)
}

fn draw_module<D: DrawTarget<Color = BinaryColor>>(module: &Module, index: usize, display: &mut D) {
    Text::new(
        module.name,
        Point::new(PADDING_LEFT, FONT_HEIGHT * (index + 1) as i32 - 1),
        MonoTextStyle::new(&FONT_6X12, BinaryColor::On),
    )
    .draw(display)
    .ok()
    .unwrap();
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

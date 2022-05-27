#![deny(clippy::transmute_ptr_to_ptr)]
#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate field_offset;

#[macro_use]
mod wrapper;

mod cstr;
mod log;

use std::os::raw::{c_int, c_void};

use embedded_graphics_core::geometry::Size;
use embedded_graphics_core::pixelcolor::BinaryColor;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use proton_ui::action::Action;
use proton_ui::display::draw as draw_display;
use proton_ui::reducer;
use proton_ui::state::*;

pub type Display = SimulatorDisplay<BinaryColor>;

static mut CLASS: Option<*mut pd_sys::_class> = None;
static mut STATE: Option<State> = None;
static mut WINDOW: Option<Window> = None;
static mut DISPLAY: Option<Display> = None;

#[repr(C)]
struct Class {
    pd_obj: pd_sys::t_object,
    signal_dummy: f32,
}

#[no_mangle]
pub unsafe extern "C" fn proton_tilde_setup() {
    let class = create_class();

    register_dsp_method!(
        class,
        receiver = Class,
        dummy_offset = offset_of!(Class => signal_dummy),
        number_of_inlets = 1,
        number_of_outlets = 1,
        callback = perform
    );

    let mut window = Window::new("", &OutputSettingsBuilder::new().scale(2).build());
    let mut display = SimulatorDisplay::new(Size::new(128, 64));

    let state = State::new("Proton")
        .with_attributes(&[
            Attribute::new("scale")
                .with_value_select(ValueSelect::new(&["major", "minor"]).unwrap()),
            Attribute::new("root").with_value_select(ValueSelect::new(&["c", "c#"]).unwrap()),
            Attribute::new("speed").with_value_f32(ValueF32::new(0.0)),
        ])
        .unwrap();
    let view = (&state).into();
    draw_display(&mut display, &view).unwrap();
    window.update(&display);

    register_float_method(class, "control1", set_control1);
    register_float_method(class, "control2", set_control2);
    register_symbol_method(class, "au", alpha_up);
    register_symbol_method(class, "ad", alpha_down);
    register_symbol_method(class, "ac", alpha_click);
    register_symbol_method(class, "bu", beta_up);
    register_symbol_method(class, "bd", beta_down);
    register_symbol_method(class, "bc", beta_click);

    CLASS = Some(class);
    STATE = Some(state);
    WINDOW = Some(window);
    DISPLAY = Some(display);
}

unsafe fn create_class() -> *mut pd_sys::_class {
    log::info("[proton~] initializing");

    pd_sys::class_new(
        pd_sys::gensym(cstr::cstr("proton~").as_ptr()),
        Some(std::mem::transmute::<
            unsafe extern "C" fn() -> *mut c_void,
            _,
        >(new)),
        None,
        std::mem::size_of::<Class>(),
        pd_sys::CLASS_DEFAULT as i32,
        0,
    )
}

unsafe extern "C" fn new() -> *mut c_void {
    let class = pd_sys::pd_new(CLASS.unwrap()) as *mut Class;

    pd_sys::outlet_new(&mut (*class).pd_obj, &mut pd_sys::s_signal);

    class as *mut c_void
}

unsafe fn register_float_method(
    class: *mut pd_sys::_class,
    symbol: &str,
    method: unsafe extern "C" fn(*mut Class, pd_sys::t_float),
) {
    pd_sys::class_addmethod(
        class,
        Some(std::mem::transmute::<
            unsafe extern "C" fn(*mut Class, pd_sys::t_float),
            _,
        >(method)),
        pd_sys::gensym(cstr::cstr(symbol).as_ptr()),
        pd_sys::t_atomtype::A_FLOAT,
        0,
    );
}

unsafe fn register_symbol_method(
    class: *mut pd_sys::_class,
    symbol: &str,
    method: unsafe extern "C" fn(*mut Class),
) {
    pd_sys::class_addmethod(
        class,
        Some(std::mem::transmute::<unsafe extern "C" fn(*mut Class), _>(
            method,
        )),
        pd_sys::gensym(cstr::cstr(symbol).as_ptr()),
        0,
    );
}

unsafe extern "C" fn set_control1(_class: *mut Class, _value: pd_sys::t_float) {
    // TODO
}

unsafe extern "C" fn set_control2(_class: *mut Class, _value: pd_sys::t_float) {
    // TODO
}

unsafe extern "C" fn alpha_up(_class: *mut Class) {
    reducer::reduce(Action::AlphaUp, STATE.as_mut().unwrap());
    update_display();
}

unsafe extern "C" fn alpha_down(_class: *mut Class) {
    reducer::reduce(Action::AlphaDown, STATE.as_mut().unwrap());
    update_display();
}

unsafe extern "C" fn alpha_click(_class: *mut Class) {
    reducer::reduce(Action::AlphaClick, STATE.as_mut().unwrap());
    update_display();
}

unsafe extern "C" fn beta_up(_class: *mut Class) {
    reducer::reduce(Action::BetaUp, STATE.as_mut().unwrap());
    update_display();
}

unsafe extern "C" fn beta_down(_class: *mut Class) {
    reducer::reduce(Action::BetaDown, STATE.as_mut().unwrap());
    update_display();
}

unsafe extern "C" fn beta_click(_class: *mut Class) {
    reducer::reduce(Action::BetaClick, STATE.as_mut().unwrap());
    update_display();
}

unsafe extern "C" fn update_display() {
    draw_display(DISPLAY.as_mut().unwrap(), &STATE.as_ref().unwrap().into()).unwrap();
    WINDOW.as_mut().unwrap().update(DISPLAY.as_mut().unwrap());
}

unsafe fn perform(
    _class: &mut Class,
    _number_of_frames: usize,
    _inlets: &[&mut [pd_sys::t_float]],
    outlets: &mut [&mut [pd_sys::t_float]],
) {
    const BUFFER_LEN: usize = 32;
    assert!(outlets[0].len() % BUFFER_LEN == 0);

    for _chunk_index in 0..outlets[0].len() / BUFFER_LEN {
        // TODO
    }
}

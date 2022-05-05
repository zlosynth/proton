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

use proton_lib::instrument::Instrument;

static mut CLASS: Option<*mut pd_sys::_class> = None;
static mut INSTRUMENT: Option<Instrument<SimulatorDisplay<BinaryColor>>> = None;
static mut WINDOW: Option<Window> = None;

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
    let mut instrument = Instrument::new();

    let display = SimulatorDisplay::new(Size::new(128, 64));
    instrument.register_display(display);

    instrument.update_display();
    window.update(instrument.mut_display());

    register_float_method(class, "control", set_control);
    register_symbol_method(class, "au", alpha_up);
    register_symbol_method(class, "ad", alpha_down);
    register_symbol_method(class, "ac", alpha_click);
    register_symbol_method(class, "ah", alpha_hold);
    register_symbol_method(class, "bu", beta_up);
    register_symbol_method(class, "bd", beta_down);
    register_symbol_method(class, "bc", beta_click);
    register_symbol_method(class, "bh", beta_hold);

    CLASS = Some(class);
    INSTRUMENT = Some(instrument);
    WINDOW = Some(window);
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

unsafe extern "C" fn set_control(_class: *mut Class, value: pd_sys::t_float) {
    INSTRUMENT
        .as_mut()
        .unwrap()
        .set_control(value.clamp(0.0, 21000.0));
}

unsafe extern "C" fn alpha_up(_class: *mut Class) {
    INSTRUMENT.as_mut().unwrap().alpha_up();
    update_display();
}

unsafe extern "C" fn alpha_down(_class: *mut Class) {
    INSTRUMENT.as_mut().unwrap().alpha_down();
    update_display();
}

unsafe extern "C" fn alpha_click(_class: *mut Class) {
    INSTRUMENT.as_mut().unwrap().alpha_click();
    update_display();
}

unsafe extern "C" fn alpha_hold(_class: *mut Class) {
    INSTRUMENT.as_mut().unwrap().alpha_hold();
    update_display();
}

unsafe extern "C" fn beta_up(_class: *mut Class) {
    INSTRUMENT.as_mut().unwrap().beta_up();
    update_display();
}

unsafe extern "C" fn beta_down(_class: *mut Class) {
    INSTRUMENT.as_mut().unwrap().beta_down();
    update_display();
}

unsafe extern "C" fn beta_click(_class: *mut Class) {
    INSTRUMENT.as_mut().unwrap().beta_click();
    update_display();
}

unsafe extern "C" fn beta_hold(_class: *mut Class) {
    INSTRUMENT.as_mut().unwrap().beta_hold();
    update_display();
}

unsafe extern "C" fn update_display() {
    INSTRUMENT.as_mut().unwrap().update_display();
    WINDOW
        .as_mut()
        .unwrap()
        .update(INSTRUMENT.as_mut().unwrap().mut_display());
}

unsafe fn perform(
    _class: &mut Class,
    _number_of_frames: usize,
    _inlets: &[&mut [pd_sys::t_float]],
    outlets: &mut [&mut [pd_sys::t_float]],
) {
    const BUFFER_LEN: usize = 32;
    assert!(outlets[0].len() % BUFFER_LEN == 0);

    for chunk_index in 0..outlets[0].len() / BUFFER_LEN {
        INSTRUMENT.as_mut().unwrap().tick();
        let buffer = INSTRUMENT.as_mut().unwrap().get_audio();
        let start = chunk_index * BUFFER_LEN;
        outlets[0][start..(BUFFER_LEN + start)].clone_from_slice(&buffer[..BUFFER_LEN]);
    }
}

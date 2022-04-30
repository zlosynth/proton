#![deny(clippy::transmute_ptr_to_ptr)]
#![allow(clippy::missing_safety_doc)]

#[macro_use]
extern crate field_offset;

#[macro_use]
mod wrapper;

mod cstr;
mod log;

use std::os::raw::{c_int, c_void};

use proton_lib::instrument::Instrument;

static mut CLASS: Option<*mut pd_sys::_class> = None;

#[repr(C)]
struct Class {
    pd_obj: pd_sys::t_object,
    instrument: Option<Instrument>,
    signal_dummy: f32,
}

#[no_mangle]
pub unsafe extern "C" fn proton_tilde_setup() {
    let class = create_class();

    CLASS = Some(class);

    register_dsp_method!(
        class,
        receiver = Class,
        dummy_offset = offset_of!(Class => signal_dummy),
        number_of_inlets = 1,
        number_of_outlets = 1,
        callback = perform
    );

    register_float_method(class, "control", set_control);
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

    let instrument = Instrument::new();
    (*class).instrument = Some(instrument);

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

unsafe extern "C" fn set_control(class: *mut Class, value: pd_sys::t_float) {
    (*class)
        .instrument
        .as_mut()
        .unwrap()
        .set_control(value.clamp(0.0, 21000.0));
}

fn perform(
    class: &mut Class,
    _number_of_frames: usize,
    _inlets: &[&mut [pd_sys::t_float]],
    outlets: &mut [&mut [pd_sys::t_float]],
) {
    const BUFFER_LEN: usize = 32;
    assert!(outlets[0].len() % BUFFER_LEN == 0);

    for chunk_index in 0..outlets[0].len() / BUFFER_LEN {
        class.instrument.as_mut().unwrap().tick();
        let buffer = class.instrument.as_mut().unwrap().get_audio();
        let start = chunk_index * BUFFER_LEN;
        outlets[0][start..(BUFFER_LEN + start)].clone_from_slice(&buffer[..BUFFER_LEN]);
    }
}

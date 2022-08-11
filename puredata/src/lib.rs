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

#[cfg(feature = "tape")]
use proton_instruments_tape::{Instrument, Rand};

#[cfg(feature = "karplus_strong")]
use proton_instruments_karplus_strong::{Instrument, Rand};

use proton_ui::action::Action;
use proton_ui::display::draw as draw_display;
use proton_ui::reaction::Reaction;
use proton_ui::reducer;
use proton_ui::state::*;

use proton_control::input_snapshot::{Cv as CvSnapshot, InputSnapshot};

pub type Display = SimulatorDisplay<BinaryColor>;

static mut CLASS: Option<*mut pd_sys::_class> = None;
static mut STATE: Option<State> = None;
static mut WINDOW: Option<Window> = None;
static mut DISPLAY: Option<Display> = None;
static mut INSTRUMENT: Option<Instrument> = None;

struct ThreadRand;

impl Rand for ThreadRand {
    fn generate(&mut self) -> u16 {
        use rand::Rng;
        rand::thread_rng().gen()
    }
}

#[repr(C)]
struct Class {
    pd_obj: pd_sys::t_object,
    right_outlet: *mut pd_sys::_outlet,
    right_inlet: *mut pd_sys::_inlet,
    signal_dummy: f32,
    input_snapshot: InputSnapshot,
}

#[no_mangle]
pub unsafe extern "C" fn proton_tilde_setup() {
    let class = create_class();

    register_dsp_method!(
        class,
        receiver = Class,
        dummy_offset = offset_of!(Class => signal_dummy),
        number_of_inlets = 2,
        number_of_outlets = 2,
        callback = perform
    );

    let mut window = Window::new("", &OutputSettingsBuilder::new().scale(2).build());
    let mut display = SimulatorDisplay::new(Size::new(128, 64));

    let sample_rate = pd_sys::sys_getsr() as u32;
    let instrument = Instrument::new(sample_rate);
    let state = instrument.state();
    #[allow(clippy::needless_borrow)] // It's not needless, it fails without it
    let view = (&state).into();
    draw_display(&mut display, &view).unwrap();
    window.update(&display);

    register_float_method(class, "control1", set_control1);
    register_float_method(class, "control2", set_control2);
    register_float_method(class, "control3", set_control3);
    register_float_method(class, "control4", set_control4);
    register_float_method(class, "control5", set_control5);
    register_symbol_method(class, "u", encoder_up);
    register_symbol_method(class, "d", encoder_down);
    register_symbol_method(class, "c", encoder_click);

    CLASS = Some(class);
    STATE = Some(state);
    WINDOW = Some(window);
    DISPLAY = Some(display);
    INSTRUMENT = Some(instrument);
}

unsafe fn create_class() -> *mut pd_sys::_class {
    log::info("[proton~] initializing");

    pd_sys::class_new(
        pd_sys::gensym(cstr::cstr("proton~").as_ptr()),
        Some(new),
        None,
        std::mem::size_of::<Class>(),
        pd_sys::CLASS_DEFAULT as i32,
        0,
    )
}

unsafe extern "C" fn new() -> *mut c_void {
    let class = pd_sys::pd_new(CLASS.unwrap()) as *mut Class;

    pd_sys::outlet_new(&mut (*class).pd_obj, &mut pd_sys::s_signal);
    (*class).right_outlet = pd_sys::outlet_new(&mut (*class).pd_obj, &mut pd_sys::s_signal);

    (*class).right_inlet = pd_sys::inlet_new(
        &mut (*class).pd_obj,
        &mut (*class).pd_obj.te_g.g_pd,
        &mut pd_sys::s_signal,
        &mut pd_sys::s_signal,
    );

    (*class).input_snapshot = InputSnapshot {
        cv: [
            CvSnapshot { value: 0.0 },
            CvSnapshot { value: 0.0 },
            CvSnapshot { value: 0.0 },
            CvSnapshot { value: 0.0 },
            CvSnapshot { value: 0.0 },
        ],
    };

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

unsafe extern "C" fn set_control1(class: *mut Class, value: pd_sys::t_float) {
    (*class).input_snapshot.cv[0].value = value;
    INSTRUMENT
        .as_mut()
        .unwrap()
        .update_control((*class).input_snapshot);
}

unsafe extern "C" fn set_control2(class: *mut Class, value: pd_sys::t_float) {
    (*class).input_snapshot.cv[1].value = value;
    INSTRUMENT
        .as_mut()
        .unwrap()
        .update_control((*class).input_snapshot);
}

unsafe extern "C" fn set_control3(class: *mut Class, value: pd_sys::t_float) {
    (*class).input_snapshot.cv[2].value = value;
    INSTRUMENT
        .as_mut()
        .unwrap()
        .update_control((*class).input_snapshot);
}

unsafe extern "C" fn set_control4(class: *mut Class, value: pd_sys::t_float) {
    (*class).input_snapshot.cv[3].value = value;
    INSTRUMENT
        .as_mut()
        .unwrap()
        .update_control((*class).input_snapshot);
}

unsafe extern "C" fn set_control5(class: *mut Class, value: pd_sys::t_float) {
    (*class).input_snapshot.cv[4].value = value;
    INSTRUMENT
        .as_mut()
        .unwrap()
        .update_control((*class).input_snapshot);
}

unsafe extern "C" fn encoder_up(_class: *mut Class) {
    let reaction = reducer::reduce(Action::EncoderUp, STATE.as_mut().unwrap());
    execute_reaction(reaction);
    update_display();
}

unsafe extern "C" fn encoder_down(_class: *mut Class) {
    let reaction = reducer::reduce(Action::EncoderDown, STATE.as_mut().unwrap());
    execute_reaction(reaction);
    update_display();
}

unsafe extern "C" fn encoder_click(_class: *mut Class) {
    let reaction = reducer::reduce(Action::EncoderClick, STATE.as_mut().unwrap());
    execute_reaction(reaction);
    update_display();
}

unsafe fn update_display() {
    draw_display(DISPLAY.as_mut().unwrap(), &STATE.as_ref().unwrap().into()).unwrap();
    WINDOW.as_mut().unwrap().update(DISPLAY.as_mut().unwrap());
}

unsafe fn execute_reaction(reaction: Option<Reaction>) {
    if let Some(reaction) = reaction {
        INSTRUMENT
            .as_mut()
            .unwrap()
            .execute(reaction.try_into().unwrap());
    }
}

unsafe fn perform(
    _class: &mut Class,
    number_of_frames: usize,
    inlets: &[&mut [pd_sys::t_float]],
    outlets: &mut [&mut [pd_sys::t_float]],
) {
    const BUFFER_LEN: usize = 32;
    assert!(number_of_frames % BUFFER_LEN == 0);
    let mut buffer = [0.0; BUFFER_LEN];

    for chunk_index in 0..number_of_frames / BUFFER_LEN {
        for (i, frame) in buffer.iter_mut().enumerate() {
            let index = chunk_index * BUFFER_LEN + i;
            *frame = inlets[0][index];
        }

        INSTRUMENT
            .as_mut()
            .unwrap()
            .process(&mut buffer, &mut ThreadRand);

        for (i, frame) in buffer.iter().enumerate() {
            let index = chunk_index * BUFFER_LEN + i;
            outlets[0][index] = *frame;
        }
    }
}

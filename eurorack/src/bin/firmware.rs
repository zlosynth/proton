#![no_main]
#![no_std]

use proton_eurorack as _; // global logger + panicking-behavior

#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use daisy::led::{Led, LedUser};

    use fugit::ExtU64;
    use proton_eurorack::system::System;
    use systick_monotonic::Systick;

    use alloc_cortex_m::CortexMHeap;

    #[global_allocator]
    static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

    use proton_eurorack::system::display::Display;
    use proton_eurorack::system::encoder::{AlphaButton, AlphaRotary, BetaButton, BetaRotary};
    use proton_lib::instrument::Instrument;
    use proton_peripherals::detent_rotary::Direction;

    type InstrumentT = Instrument<Display>;

    #[monotonic(binds = SysTick, default = true)]
    type Mono = Systick<1000>; // 1 kHz / 1 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: LedUser,
        alpha_button: AlphaButton,
        alpha_rotary: AlphaRotary,
        beta_button: BetaButton,
        beta_rotary: BetaRotary,
    }

    static mut INSTRUMENT: Option<InstrumentT> = None;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("INIT");

        init_allocator();

        let system = System::init(cx.core, cx.device);

        let display = system.display;
        let led = system.led;
        let mono = system.mono;
        let alpha_button = system.alpha_button;
        let alpha_rotary = system.alpha_rotary;
        let beta_button = system.beta_button;
        let beta_rotary = system.beta_rotary;

        let mut instrument = Instrument::new();
        instrument.register_display(display);

        unsafe { INSTRUMENT = Some(instrument) };

        foo::spawn(true).unwrap();
        control::spawn().unwrap();

        (
            Shared {},
            Local {
                led,
                alpha_button,
                alpha_rotary,
                beta_button,
                beta_rotary,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led])]
    fn foo(cx: foo::Context, on: bool) {
        if on {
            cx.local.led.on();
            foo::spawn_after(1.secs(), false).unwrap();
        } else {
            cx.local.led.off();
            foo::spawn_after(1.secs(), true).unwrap();
        }
    }

    #[task(local = [alpha_button, alpha_rotary, beta_button, beta_rotary])]
    fn control(cx: control::Context) {
        cx.local.alpha_button.sample();
        cx.local.alpha_rotary.sample().unwrap();
        cx.local.beta_button.sample();
        cx.local.beta_rotary.sample().unwrap();

        let instrument = unsafe { INSTRUMENT.as_mut().unwrap() };

        if cx.local.alpha_button.clicked() {
            defmt::info!("ALPHA ON");
            instrument.alpha_click();
        }
        match cx.local.alpha_rotary.direction() {
            Direction::Clockwise => {
                defmt::info!("ALPHA CW");
                instrument.alpha_down();
            }
            Direction::CounterClockwise => {
                defmt::info!("ALPHA CCW");
                instrument.alpha_up();
            }
            _ => (),
        }
        if cx.local.beta_button.clicked() {
            defmt::info!("BETA ON");
            instrument.beta_click();
        }
        match cx.local.beta_rotary.direction() {
            Direction::Clockwise => {
                defmt::info!("BETA CW");
                instrument.beta_down();
            }
            Direction::CounterClockwise => {
                defmt::info!("BETA CCW");
                instrument.beta_up();
            }
            _ => (),
        }

        instrument.update_display();
        instrument.mut_display().flush().unwrap();

        control::spawn_after(1.millis()).unwrap();
    }

    fn init_allocator() {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 10 * 1024;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
    }
}

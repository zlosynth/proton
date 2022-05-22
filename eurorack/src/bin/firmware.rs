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

    use proton_eurorack::system::{AlphaButton, BetaButton};
    use proton_lib::instrument::Instrument;

    #[monotonic(binds = SysTick, default = true)]
    type Mono = Systick<1000>; // 1 kHz / 1 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: LedUser,
        alpha_button: AlphaButton,
        beta_button: BetaButton,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("INIT");

        init_allocator();

        let system = System::init(cx.core, cx.device);

        let display = system.display;
        let led = system.led;
        let mono = system.mono;
        let alpha_button = system.alpha_button;
        let beta_button = system.beta_button;

        let mut instrument = Instrument::new();
        instrument.register_display(display);
        instrument.update_display();
        instrument.mut_display().flush().unwrap();

        foo::spawn(true).unwrap();
        control::spawn().unwrap();

        (
            Shared {},
            Local {
                led,
                alpha_button,
                beta_button,
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

    #[task(local = [alpha_button, beta_button])]
    fn control(cx: control::Context) {
        cx.local.alpha_button.sample();
        cx.local.beta_button.sample();

        if cx.local.alpha_button.clicked() {
            defmt::info!("ALPHA ON");
        }
        if cx.local.beta_button.clicked() {
            defmt::info!("BETA ON");
        }

        control::spawn_after(1.millis()).unwrap();
    }

    fn init_allocator() {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 10 * 1024;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE) }
    }
}

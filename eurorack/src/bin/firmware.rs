#![no_main]
#![no_std]

use proton_eurorack as _; // global logger + panicking-behavior

#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use daisy::led::{Led, LedUser};

    use fugit::ExtU64;
    use proton_eurorack::system::System;
    use systick_monotonic::Systick;

    type Instrument = proton_lib::instrument::Instrument<proton_eurorack::system::display::Display>;
    type Input = proton_ui::input::Input<
        proton_eurorack::system::encoder::AlphaRotaryPinA,
        proton_eurorack::system::encoder::AlphaRotaryPinB,
        proton_eurorack::system::encoder::AlphaButtonPin,
        proton_eurorack::system::encoder::BetaRotaryPinA,
        proton_eurorack::system::encoder::BetaRotaryPinB,
        proton_eurorack::system::encoder::BetaButtonPin,
    >;

    #[monotonic(binds = SysTick, default = true)]
    type Mono = Systick<1000>; // 1 kHz / 1 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: LedUser,
        user_input: Input,
    }

    static mut INSTRUMENT: Option<Instrument> = None;

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("INIT");

        proton_eurorack::init_allocator();

        let system = System::init(cx.core, cx.device);

        let display = system.display;
        let led = system.led;
        let mono = system.mono;
        let alpha_button = system.alpha_button;
        let alpha_rotary = system.alpha_rotary;
        let beta_button = system.beta_button;
        let beta_rotary = system.beta_rotary;

        let user_input = Input::new(alpha_button, alpha_rotary, beta_button, beta_rotary);

        let mut instrument = Instrument::new();
        instrument.register_display(display);

        unsafe { INSTRUMENT = Some(instrument) };

        foo::spawn(true).unwrap();
        control::spawn().unwrap();

        (Shared {}, Local { led, user_input }, init::Monotonics(mono))
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

    #[task(local = [user_input])]
    fn control(cx: control::Context) {
        use proton_ui::action::Action;

        let instrument = unsafe { INSTRUMENT.as_mut().unwrap() };

        for action in cx.local.user_input.process() {
            match action {
                Action::AlphaClick => {
                    defmt::info!("ALPHA ON");
                    instrument.alpha_click();
                }
                Action::AlphaUp => {
                    defmt::info!("ALPHA CCW");
                    instrument.alpha_up();
                }
                Action::AlphaDown => {
                    defmt::info!("ALPHA CW");
                    instrument.alpha_down();
                }
                Action::BetaClick => {
                    defmt::info!("BETA ON");
                    instrument.beta_click();
                }
                Action::BetaUp => {
                    defmt::info!("BETA CCW");
                    instrument.beta_up();
                }
                Action::BetaDown => {
                    defmt::info!("BETA CW");
                    instrument.beta_down();
                }
            }
        }

        instrument.update_display();
        instrument.mut_display().flush().unwrap();

        control::spawn_after(1.millis()).unwrap();
    }
}

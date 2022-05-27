#![no_main]
#![no_std]

use proton_eurorack as _; // global logger + panicking-behavior

#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [EXTI0, EXTI1])]
mod app {
    use daisy::led::{Led, LedUser};

    use fugit::ExtU64;
    use heapless::spsc::{Consumer, Producer, Queue};
    use systick_monotonic::Systick;

    use proton_eurorack::system::display::Display;
    use proton_eurorack::system::System;
    use proton_ui::action::Action as InputAction;
    use proton_ui::display::draw as draw_view_on_display;
    use proton_ui::reducer;
    use proton_ui::state::State;
    use proton_ui::view::View;

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
        display: Display,
        state: State,
        input_actions_producer: Producer<'static, InputAction, 6>,
        input_actions_consumer: Consumer<'static, InputAction, 6>,
    }

    #[init(local = [input_actions_queue: Queue<InputAction, 6> = Queue::new()])]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("INIT");

        let (input_actions_producer, input_actions_consumer) = cx.local.input_actions_queue.split();

        let system = System::init(cx.core, cx.device);

        let display = system.display;
        let led = system.led;
        let mono = system.mono;

        let user_input = Input::new(
            system.alpha_button,
            system.alpha_rotary,
            system.beta_button,
            system.beta_rotary,
        );

        let state = {
            use proton_ui::state::*;
            State::new("Proton")
                .with_attributes(&[
                    Attribute::new("scale").with_value_select(
                        ValueSelect::new(&["major", "minor", "phrygian"]).unwrap(),
                    ),
                    Attribute::new("root").with_value_select(
                        ValueSelect::new(&[
                            "c ", "c#", "d ", "d#", "e ", "f ", "f#", "g ", "g#", "a ", "a#", "b ",
                        ])
                        .unwrap(),
                    ),
                    Attribute::new("speed").with_value_f32(ValueF32::new(0.3)),
                ])
                .unwrap()
        };
        let view = (&state).into();

        update_display::spawn(view).unwrap();
        set_indicator::spawn(true).unwrap();
        read_controls::spawn().unwrap();
        update_state::spawn().unwrap();

        (
            Shared {},
            Local {
                led,
                user_input,
                display,
                state,
                input_actions_producer,
                input_actions_consumer,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [user_input, input_actions_producer], priority = 2)]
    fn read_controls(cx: read_controls::Context) {
        let input_actions_producer = cx.local.input_actions_producer;

        for action in cx.local.user_input.process() {
            input_actions_producer.enqueue(action).unwrap();
        }

        read_controls::spawn_after(1.millis()).unwrap();
    }

    #[task(local = [input_actions_consumer, state])]
    fn update_state(cx: update_state::Context) {
        let input_actions_consumer = cx.local.input_actions_consumer;

        let state = cx.local.state;

        while let Some(action) = input_actions_consumer.dequeue() {
            reducer::reduce(action, state);
        }

        let view = (&*state).into();
        update_display::spawn(view).unwrap();

        update_state::spawn_after(1.millis()).unwrap();
    }

    #[task(local = [display])]
    fn update_display(cx: update_display::Context, view: View) {
        let display = cx.local.display;
        draw_view_on_display(display, &view).unwrap();
        display.flush().unwrap();
    }

    #[task(local = [led])]
    fn set_indicator(cx: set_indicator::Context, on: bool) {
        if on {
            cx.local.led.on();
            set_indicator::spawn_after(1.secs(), false).unwrap();
        } else {
            cx.local.led.off();
            set_indicator::spawn_after(1.secs(), true).unwrap();
        }
    }
}

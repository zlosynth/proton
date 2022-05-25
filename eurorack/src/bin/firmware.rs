#![no_main]
#![no_std]

use proton_eurorack as _; // global logger + panicking-behavior

#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use daisy::led::{Led, LedUser};

    use fugit::ExtU64;
    use heapless::spsc::{Consumer, Producer, Queue};
    use systick_monotonic::Systick;

    use proton_eurorack::system::display::Display;
    use proton_eurorack::system::System;
    use proton_ui::action::Action as InputAction;
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

        let mut display = system.display;
        let led = system.led;
        let mono = system.mono;
        let alpha_button = system.alpha_button;
        let alpha_rotary = system.alpha_rotary;
        let beta_button = system.beta_button;
        let beta_rotary = system.beta_rotary;

        let user_input = Input::new(alpha_button, alpha_rotary, beta_button, beta_rotary);

        let state = {
            use heapless::Vec;
            use proton_ui::state::*;
            State {
                title: "Proton",
                attributes: Vec::from_slice(&[
                    Attribute {
                        name: "scale",
                        value: Value::Select(ValueSelect {
                            available: Vec::from_slice(&["major", "minor"]).unwrap(),
                            selected: 0,
                        }),
                    },
                    Attribute {
                        name: "root",
                        value: Value::Select(ValueSelect {
                            available: Vec::from_slice(&["c", "c#"]).unwrap(),
                            selected: 1,
                        }),
                    },
                    Attribute {
                        name: "speed",
                        value: Value::F32(1.0),
                    },
                ])
                .unwrap(),
                selected_attribute: 1,
            }
        };
        let view = (&state).into();
        proton_ui::display::draw(&mut display, &view).unwrap();
        display.flush().unwrap();

        indicator::spawn(true).unwrap();
        control::spawn().unwrap();
        state::spawn().unwrap();

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

    #[task(local = [user_input, input_actions_producer])]
    fn control(cx: control::Context) {
        let input_actions_producer = cx.local.input_actions_producer;

        for action in cx.local.user_input.process() {
            input_actions_producer.enqueue(action).unwrap();
        }

        control::spawn_after(1.millis()).unwrap();
    }

    #[task(local = [input_actions_consumer, state])]
    fn state(cx: state::Context) {
        let input_actions_consumer = cx.local.input_actions_consumer;

        let mut state = cx.local.state;

        while let Some(action) = input_actions_consumer.dequeue() {
            proton_ui::reducer::reduce(action, state);
            match action {
                InputAction::AlphaClick => {
                    defmt::info!("ALPHA ON");
                }
                InputAction::AlphaUp => {
                    defmt::info!("ALPHA CCW");
                }
                InputAction::AlphaDown => {
                    defmt::info!("ALPHA CW");
                }
                InputAction::BetaClick => {
                    defmt::info!("BETA ON");
                }
                InputAction::BetaUp => {
                    defmt::info!("BETA CCW");
                }
                InputAction::BetaDown => {
                    defmt::info!("BETA CW");
                }
            }
        }

        let view = (&*state).into();
        display::spawn(view).unwrap();

        state::spawn_after(1.millis()).unwrap();
    }

    #[task(local = [display])]
    fn display(cx: display::Context, view: View) {
        let display = cx.local.display;
        proton_ui::display::draw(display, &view).unwrap();
        display.flush().unwrap();
    }

    #[task(local = [led])]
    fn indicator(cx: indicator::Context, on: bool) {
        if on {
            cx.local.led.on();
            indicator::spawn_after(1.secs(), false).unwrap();
        } else {
            cx.local.led.off();
            indicator::spawn_after(1.secs(), true).unwrap();
        }
    }
}

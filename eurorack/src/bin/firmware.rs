#![no_main]
#![no_std]

use proton_eurorack as _; // global logger + panicking-behavior

#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [EXTI0, EXTI1, EXTI2])]
mod app {
    use daisy::led::{Led, LedUser};

    use fugit::ExtU64;
    use heapless::spsc::{Consumer, Producer, Queue};
    use systick_monotonic::Systick;

    use daisy::hal;
    use hal::adc::{Adc, Enabled};
    use hal::pac::{ADC1, ADC2};

    #[cfg(feature = "kaseta")]
    use proton_instruments_kaseta::Instrument;

    use proton_control::input_snapshot::InputSnapshot;
    use proton_eurorack::system::audio::{Audio, SAMPLE_RATE};
    use proton_eurorack::system::display::Display;
    use proton_eurorack::system::System;
    use proton_instruments_interface::Instrument as _;
    use proton_ui::action::Action as InputAction;
    use proton_ui::display::draw as draw_view_on_display;
    use proton_ui::reaction::Reaction as InputReaction;
    use proton_ui::reducer;
    use proton_ui::state::State;
    use proton_ui::view::View;

    type UserInput = proton_ui::input::Input<
        proton_eurorack::system::encoder::EncoderRotaryPinA,
        proton_eurorack::system::encoder::EncoderRotaryPinB,
        proton_eurorack::system::encoder::EncoderButtonPin,
    >;

    type ControlInput = proton_control::input_processor::InputProcessor<
        Adc<ADC1, Enabled>,
        Adc<ADC2, Enabled>,
        proton_eurorack::system::cv_input::Pot,
        proton_eurorack::system::cv_input::CvInput1,
        proton_eurorack::system::cv_input::CvInput2,
        proton_eurorack::system::cv_input::CvInput3,
        proton_eurorack::system::cv_input::CvInput4,
        proton_eurorack::system::cv_input::CvInput5,
    >;

    type ControlOutput = proton_control::output_processor::OutputProcessor<
        proton_eurorack::system::gate_output::GateOutput1,
        proton_eurorack::system::gate_output::GateOutput2,
        proton_eurorack::system::gate_output::GateOutput3,
        proton_eurorack::system::cv_output::CvOutput1,
        proton_eurorack::system::cv_output::CvOutput2,
    >;

    #[monotonic(binds = SysTick, default = true)]
    type Mono = Systick<1000>; // 1 kHz / 1 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        audio: Audio,
        instrument: Instrument,
        led: LedUser,
        user_input: UserInput,
        control_input: ControlInput,
        _control_output: ControlOutput,
        display: Display,
        state: State,
        input_actions_producer: Producer<'static, InputAction, 6>,
        input_actions_consumer: Consumer<'static, InputAction, 6>,
        input_reactions_producer: Producer<'static, InputReaction, 6>,
        input_reactions_consumer: Consumer<'static, InputReaction, 6>,
        control_input_producer: Producer<'static, InputSnapshot, 6>,
        control_input_consumer: Consumer<'static, InputSnapshot, 6>,
    }

    #[init(
        local = [
            input_actions_queue: Queue<InputAction, 6> = Queue::new(),
            input_reactions_queue: Queue<InputReaction, 6> = Queue::new(),
            control_input_queue: Queue<InputSnapshot, 6> = Queue::new(),
        ]
    )]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("INIT");

        let (input_actions_producer, input_actions_consumer) = cx.local.input_actions_queue.split();
        let (input_reactions_producer, input_reactions_consumer) =
            cx.local.input_reactions_queue.split();
        let (control_input_producer, control_input_consumer) = cx.local.control_input_queue.split();

        let system = System::init(cx.core, cx.device);

        let display = system.display;
        let led = system.led;
        let mono = system.mono;
        let mut audio = system.audio;
        audio.spawn();

        let user_input = UserInput::new(system.button, system.rotary);

        let control_input = ControlInput::new(
            system.adc_1,
            system.adc_2,
            system.pot,
            system.cv_input_1,
            system.cv_input_2,
            system.cv_input_3,
            system.cv_input_4,
            system.cv_input_5,
        );

        let control_output = ControlOutput::new(
            system.gate_1,
            system.gate_2,
            system.gate_3,
            system.cv_output_1,
            system.cv_output_2,
        );

        let instrument = Instrument::new(SAMPLE_RATE);
        let state = instrument.state();
        #[allow(clippy::needless_borrow)] // It's not needless, it fails without it
        let view: View = (&state).into();

        update_display::spawn(view).ok().unwrap();
        set_indicator::spawn(true).unwrap();
        read_user_controls::spawn().unwrap();
        read_control_input::spawn().unwrap();
        update_state::spawn().unwrap();

        (
            Shared {},
            Local {
                audio,
                instrument,
                led,
                user_input,
                control_input,
                _control_output: control_output,
                display,
                state,
                input_actions_producer,
                input_actions_consumer,
                input_reactions_producer,
                input_reactions_consumer,
                control_input_producer,
                control_input_consumer,
            },
            init::Monotonics(mono),
        )
    }

    #[task(binds = DMA1_STR1, local = [input_reactions_consumer, control_input_consumer, instrument, audio], priority = 4)]
    fn handle_dsp(cx: handle_dsp::Context) {
        use core::convert::TryInto;

        let input_reactions_consumer = cx.local.input_reactions_consumer;
        let control_input_consumer = cx.local.control_input_consumer;
        let instrument = cx.local.instrument;
        let audio = cx.local.audio;

        while let Some(control_snapshot) = control_input_consumer.dequeue() {
            instrument.update_control(control_snapshot);
        }

        while let Some(action) = input_reactions_consumer.dequeue() {
            let reaction = action.try_into();
            instrument.execute(reaction.unwrap());
        }

        audio.update_buffer(|buffer| {
            instrument.process(&mut buffer[..]);
        });
    }

    #[task(local = [user_input, input_actions_producer], priority = 3)]
    fn read_user_controls(cx: read_user_controls::Context) {
        let user_input = cx.local.user_input;
        let input_actions_producer = cx.local.input_actions_producer;

        for action in user_input.process() {
            input_actions_producer.enqueue(action).unwrap();
        }

        read_user_controls::spawn_after(1.millis()).unwrap();
    }

    #[task(local = [control_input, control_input_producer], priority = 2)]
    fn read_control_input(cx: read_control_input::Context) {
        let control_input = cx.local.control_input;
        let control_input_producer = cx.local.control_input_producer;

        control_input_producer
            .enqueue(control_input.update())
            .ok()
            .unwrap();

        read_control_input::spawn_after(1.millis()).unwrap();
    }

    #[task(local = [input_actions_consumer, input_reactions_producer, state])]
    fn update_state(cx: update_state::Context) {
        let input_actions_consumer = cx.local.input_actions_consumer;
        let input_reactions_producer = cx.local.input_reactions_producer;

        let state = cx.local.state;

        while let Some(action) = input_actions_consumer.dequeue() {
            let reaction = reducer::reduce(action, state);
            if let Some(reaction) = reaction {
                #[allow(clippy::ok_expect)]
                input_reactions_producer
                    .enqueue(reaction)
                    .ok()
                    .expect("the queue is full");
            }
        }

        #[allow(clippy::needless_borrow)] // It's not needless, it fails without it
        let view = (&*state).into();
        update_display::spawn(view).ok().unwrap();

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

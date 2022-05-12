#![no_main]
#![no_std]

use proton_eurorack as _; // global logger + panicking-behavior

#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use daisy::led::{Led, LedUser};
    use embedded_graphics::{
        pixelcolor::BinaryColor,
        prelude::*,
        primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
    };
    use fugit::ExtU64;
    use proton_eurorack::system::System;
    use systick_monotonic::Systick;

    #[monotonic(binds = SysTick, default = true)]
    type Mono = Systick<1000>; // 1 kHz / 1 ms granularity

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: LedUser,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("INIT");

        let system = System::init(cx.core, cx.device);

        let mut display = system.display;
        let led = system.led;
        let mono = system.mono;

        let yoffset = 20;

        let style = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .stroke_color(BinaryColor::On)
            .build();

        // screen outline
        // default display size is 128x64 if you don't pass a _DisplaySize_
        // enum to the _Builder_ struct
        Rectangle::new(Point::new(0, 0), Size::new(127, 63))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        // triangle
        Triangle::new(
            Point::new(16, 16 + yoffset),
            Point::new(16 + 16, 16 + yoffset),
            Point::new(16 + 8, yoffset),
        )
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

        // square
        Rectangle::new(Point::new(52, yoffset), Size::new_equal(16))
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        // circle
        Circle::new(Point::new(88, yoffset), 16)
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        foo::spawn(true).unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led])]
    fn foo(cx: foo::Context, on: bool) {
        defmt::info!("FOO: {:?}", on);

        if on {
            cx.local.led.on();
            foo::spawn_after(1.secs(), false).unwrap();
        } else {
            cx.local.led.off();
            foo::spawn_after(1.secs(), true).unwrap();
        }
    }
}

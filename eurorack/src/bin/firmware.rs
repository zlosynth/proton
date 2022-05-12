#![no_main]
#![no_std]

use proton_eurorack as _; // global logger + panicking-behavior

#[rtic::app(device = stm32h7xx_hal::pac, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use daisy::hal::prelude::_stm32h7xx_hal_spi_SpiExt;
    use daisy::hal::prelude::_stm32h7xx_hal_timer_TimerExt;
    use daisy::hal::{delay::DelayFromCountDownTimer, spi};
    use daisy::led::{Led, LedUser};
    use embedded_graphics::{
        pixelcolor::BinaryColor,
        prelude::*,
        primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
    };
    use fugit::ExtU64;
    use fugit::RateExtU32;
    use ssd1306::{prelude::*, Ssd1306};
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

        let mono = Systick::new(cx.core.SYST, 480_000_000);

        let dp = cx.device;
        let board = daisy::Board::take().unwrap();
        let ccdr = daisy::board_freeze_clocks!(board, dp);
        let pins = daisy::board_split_gpios!(board, ccdr, dp);
        let led = daisy::board_split_leds!(pins).USER;

        let mut delay = DelayFromCountDownTimer::new(dp.TIM2.timer(
            100.Hz(),
            ccdr.peripheral.TIM2,
            &ccdr.clocks,
        ));

        let sck = pins.GPIO.PIN_8.into_alternate();
        let cs = pins.GPIO.PIN_7.into_push_pull_output();
        let miso = spi::NoMiso;
        let mosi = pins.GPIO.PIN_10.into_alternate();
        let mut rst = pins.GPIO.PIN_30.into_push_pull_output();
        let dc = pins.GPIO.PIN_9.into_push_pull_output();

        let spi = dp.SPI1.spi(
            (sck, miso, mosi),
            spi::MODE_0,
            3.MHz(),
            ccdr.peripheral.SPI1,
            &ccdr.clocks,
        );

        let interface = display_interface_spi::SPIInterface::new(spi, dc, cs);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        display.reset(&mut rst, &mut delay).unwrap();
        display.init().unwrap();

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

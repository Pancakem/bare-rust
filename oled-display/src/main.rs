#![no_main]
#![no_std]

// set the panic handler
extern crate panic_semihosting;

use cortex_m;
use cortex_m_rt::{entry, exception};
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyle,
};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
};

use ssd1306::{mode::*, prelude::DisplaySize, Builder};

#[entry]
fn main() -> ! {
    let _core = cortex_m::Peripherals::take().unwrap();
    let device = stm32f1xx_hal::stm32::Peripherals::take().unwrap();
    let mut rcc = device.RCC.constrain();
    let mut flash = device.FLASH.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(16.mhz())
        .freeze(&mut flash.acr);

    // Set up I2C
    let mut afio = device.AFIO.constrain(&mut rcc.apb2);
    // SCL - PB8
    // SDA - PB9
    let mut gpiob = device.GPIOB.split(&mut rcc.apb2);
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        device.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let mut disp: GraphicsMode<_> = Builder::new()
        .size(DisplaySize::Display128x64)
        .connect_i2c(i2c)
        .into();

    disp.init().unwrap();
    disp.flush().unwrap();

    let _ = disp.clear();

    let text_style = TextStyle::new(Font6x8, BinaryColor::On);
    let message = "Hello from SMT32 blue-pill";
    let width = message.len() as i32 * 6;
    Text::new(message, Point::new(64 - width / 2, 43))
        .into_styled(text_style)
        .draw(&mut disp)
        .unwrap();

    loop {}
}

#[exception]
unsafe fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

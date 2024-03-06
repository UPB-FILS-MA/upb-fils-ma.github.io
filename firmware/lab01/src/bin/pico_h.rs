//! Use this binary for the Raspberry Pi Pico H
//! 
//! The on-board LED is connected to `PIN_25`

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::cell::RefCell;

use core::fmt::Write;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_rp::adc;
use embassy_rp::gpio::{self, Input, Level, Output, Pull};
use embassy_rp::peripherals::PIN_2;
use embassy_rp::spi::{Blocking, Spi};
use embassy_rp::{bind_interrupts, spi};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_time::{Delay, Timer};
use embedded_graphics::mono_font::iso_8859_16::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::text::renderer::CharacterStyle;
use embedded_graphics::text::Text;
use heapless::String;
use panic_probe as _;
use lab01::SPIDeviceInterface;
use st7789::{Orientation, ST7789};

const DISPLAY_FREQ: u32 = 64_000_000;

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => adc::InterruptHandler;
});

#[embassy_executor::task]
async fn btn_listener(mut btn: Input<'static, PIN_2>) {
    loop {
        btn.wait_for_falling_edge().await;
        unsafe { STATE = !STATE }
        btn.wait_for_rising_edge().await;
    }
}

static mut STATE: bool = false;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // FONT STYLE
    let mut style = MonoTextStyle::new(&FONT_10X20, Rgb565::GREEN);
    style.set_background_color(Some(Rgb565::BLACK));

    // ************** Display initialization - DO NOT MODIFY! *****************
    let miso = p.PIN_4;
    let display_cs = p.PIN_17;
    let mosi = p.PIN_19;
    let clk = p.PIN_18;
    let rst = p.PIN_0;
    let dc = p.PIN_16;
    let mut display_config = spi::Config::default();
    display_config.frequency = DISPLAY_FREQ;
    display_config.phase = spi::Phase::CaptureOnSecondTransition;
    display_config.polarity = spi::Polarity::IdleHigh;

    // Init SPI
    let spi: Spi<'_, _, Blocking> =
        Spi::new_blocking(p.SPI0, clk, mosi, miso, display_config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

    let display_spi = SpiDeviceWithConfig::new(
        &spi_bus,
        Output::new(display_cs, Level::High),
        display_config,
    );

    let dc = Output::new(dc, Level::Low);
    let rst = Output::new(rst, Level::Low);
    let di = SPIDeviceInterface::new(display_spi, dc);

    // Init ST7789 LCD
    let mut display = ST7789::new(di, rst, 240, 240);
    display.init(&mut Delay).unwrap();
    display.set_orientation(Orientation::Portrait).unwrap();
    display.clear(Rgb565::BLACK).unwrap();
    // ************************************************************************

    // Clear display
    display.clear(Rgb565::BLACK).unwrap();

    // ADC channel setup
    let mut adc = adc::Adc::new(p.ADC, Irqs, adc::Config::default());
    let mut p26 = adc::Channel::new_pin(p.PIN_26, Pull::Down);

    // Always-high pin setup
    let _pin_high = gpio::Output::new(p.PIN_1, Level::High);

    // Button pin setup
    let btn = gpio::Input::new(p.PIN_2, Pull::None);
    spawner.spawn(btn_listener(btn)).unwrap();

    // Board LED setup
    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        let level = adc.read(&mut p26).await.unwrap() as f64 * 0.825 / 1000.0;

        if unsafe { !STATE } {
            led.set_high();
        } else {
            led.set_low();
        }

        let mut text = String::<64>::new();
        write!(text, "Voltage: {:.4} V", level).unwrap();

        Text::new(&text, Point::new(40, 110), style)
            .draw(&mut display)
            .unwrap();

        // Small delay for yielding
        Timer::after_millis(1).await;
    }
}

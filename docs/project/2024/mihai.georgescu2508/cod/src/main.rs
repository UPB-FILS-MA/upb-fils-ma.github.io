#![no_std]
#![no_main]
#![allow(unused_imports, dead_code)]

use core::panic::PanicInfo;

use cyw43_pio::PioSpi;
use embassy_executor::Spawner;

use embassy_net::Stack;
use embassy_rp::bind_interrupts;
use embassy_rp::pwm::Config as ConfigPwm; // PWM config
use embassy_rp::config::Config;
// GPIO
use embassy_rp::gpio::{Input, Level, Output, Pull};

use embassy_rp::pio::{InterruptHandler, Pio};
// PWM
use embassy_rp::pwm::{Config as PwmConfig, Pwm};

// USB driver
use embassy_rp::peripherals::{DMA_CH0, PIO0, USB};
use embassy_rp::usb::{Driver, Endpoint, InterruptHandler as USBInterruptHandler};
use embassy_time::Timer;
use static_cell::StaticCell;

use log::info;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => USBInterruptHandler<USB>;
    // PIO interrupt for CYW SPI communication
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const WIFI_NETWORK: &str = "iPhone - Antonio";
const WIFI_PASSWORD: &str = "roe5710398g24";

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let peripherals = embassy_rp::init(Config::default());

    // USB logger driver
    let driver = Driver::new(peripherals.USB, Irqs);
    spawner.spawn(logger_task(driver)).unwrap();

    // Link CYW43 firmware
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

    // Init SPI for communication with CYW43
    let pwr = Output::new(peripherals.PIN_23, Level::Low);
    let cs = Output::new(peripherals.PIN_25, Level::High);
    let mut pio = Pio::new(peripherals.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        pio.irq0,
        cs,
        peripherals.PIN_24,
        peripherals.PIN_29,
        peripherals.DMA_CH0,
    );

    // Create config for PWM slice
    let mut config: ConfigPwm = Default::default();
    // Set top value (value at which PWM counter will reset)
    config.top = 0x8000; // in HEX, equals 32768 in decimal
    // Set compare value (counter value at which the PWM signal will change from 1 to 0)
    config.compare_a = config.top / 2;
    config.compare_b = config.top / 2;

    let mut pwm1 = Pwm::new_output_ab( // output AB
        peripherals.PWM_SLICE2, // channel 0
        peripherals.PIN_4, // pin 0
        peripherals.PIN_5, // pin 1
        config.clone()
    );

    let mut pwm2 = Pwm::new_output_ab( // output AB
        peripherals.PWM_SLICE1, // channel 0
        peripherals.PIN_2, // pin 0
        peripherals.PIN_3, // pin 1
        config.clone()
    );

    // Start Wi-Fi task
    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(wifi_task(runner)).unwrap();

    // WIFI INIT END

    // Init the device
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    loop {
        Timer::after_millis(1000).await;
        info!("running");
    }
}

    
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
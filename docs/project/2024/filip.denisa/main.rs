use embassy::executor::Executor;
use embassy::time::{Duration, Timer};
use embassy_rp::gpio::{Output, Level, Input};
use embassy_rp::i2c::{self, Config, I2c};
use embassy_rp::init;
use embassy_nrf::init as init_nrf;
use hd44780_driver::HD44780;
use heapless::String;

const LED1_R_PIN: u8 = 2;
const LED1_G_PIN: u8 = 3;
const LED1_B_PIN: u8 = 4;
const LED2_R_PIN: u8 = 5;
const LED2_G_PIN: u8 = 6;
const LED2_B_PIN: u8 = 7;

const SENSOR1_PIN: u8 = 8;
const SENSOR2_PIN: u8 = 9;

const FAR_DISTANCE: u8 = 100;
const MID_DISTANCE: u8 = 50;
const CLOSE_DISTANCE: u8 = 3;

#[embassy::main]
async fn main(spawner: embassy::executor::Spawner) {
    let p = init(Default::default());

    let mut led1_r = Output::new(p.PIN_2, Level::High);
    let mut led1_g = Output::new(p.PIN_3, Level::High);
    let mut led1_b = Output::new(p.PIN_4, Level::High);
    let mut led2_r = Output::new(p.PIN_5, Level::High);
    let mut led2_g = Output::new(p.PIN_6, Level::High);
    let mut led2_b = Output::new(p.PIN_7, Level::High);

    let sensor1 = Input::new(p.PIN_8, embassy_rp::gpio::Pull::None);
    let sensor2 = Input::new(p.PIN_9, embassy_rp::gpio::Pull::None);

    let i2c = I2c::new(p.I2C, p.PIN_0, p.PIN_1, Config::default());
    let mut lcd = HD44780::new_i2c(i2c, 0x27, 16, 2).unwrap();

    loop {
        let distance1 = read_distance(&sensor1).await;
        let distance2 = read_distance(&sensor2).await;

        let distance = (distance1 + distance2) / 2;

        if distance > FAR_DISTANCE {
            set_led_color(&mut led1_r, &mut led1_g, &mut led1_b, 0, 255, 0);
            set_led_color(&mut led2_r, &mut led2_g, &mut led2_b, 0, 255, 0);
        } else if distance > MID_DISTANCE {
            set_led_color(&mut led1_r, &mut led1_g, &mut led1_b, 0, 0, 255);
            set_led_color(&mut led2_r, &mut led2_g, &mut led2_b, 0, 0, 255);
        } else if distance > CLOSE_DISTANCE {
            set_led_color(&mut led1_r, &mut led1_g, &mut led1_b, 255, 0, 0);
            set_led_color(&mut led2_r, &mut led2_g, &mut led2_b, 255, 0, 0);
        } else {
            set_led_color(&mut led1_r, &mut led1_g, &mut led1_b, 255, 0, 0);
            set_led_color(&mut led2_r, &mut led2_g, &mut led2_b, 255, 0, 0);
            lcd.clear().unwrap();
            let message: String<32> = format!("Distance {}cm, CANNOT PARK", distance).into();
            lcd.write_str(&message).unwrap();
        }

        lcd.clear().unwrap();
        let message: String<32> = format!("Distance: {}cm", distance).into();
        lcd.write_str(&message).unwrap();

        Timer::after(Duration::from_millis(500)).await;
    }
}

async fn read_distance(sensor: &Input) -> u8 {
   
    10
}

fn set_led_color(r: &mut Output, g: &mut Output, b: &mut Output, red: u8, green: u8, blue: u8) {
    r.set_level(if red > 0 { Level::Low } else { Level::High });
    g.set_level(if green > 0 { Level::Low } else { Level::High });
    b.set_level(if blue > 0 { Level::Low } else { Level::High });
}

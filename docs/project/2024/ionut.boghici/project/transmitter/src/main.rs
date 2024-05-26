#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_rp::init;
use embassy_time::{Duration, Timer};
use panic_probe as _; 

// Morse code dictionary
const MORSE_CODE: [(&str, &str); 36] = [
  ("A", ".-"), ("B", "-..."), ("C", "-.-."), ("D", "-.."), ("E", "."),
  ("F", "..-."), ("G", "--."), ("H", "...."), ("I", ".."), ("J", ".---"),
  ("K", "-.-"), ("L", ".-.."), ("M", "--"), ("N", "-."), ("O", "---"),
  ("P", ".--."), ("Q", "--.-"), ("R", ".-."), ("S", "..."), ("T", "-"),
  ("U", "..-"), ("V", "...-"), ("W", ".--"), ("X", "-..-"), ("Y", "-.--"),
  ("Z", "--.."), ("0", "-----"), ("1", ".----"), ("2", "..---"), ("3", "...--"),
  ("4", "....-"), ("5", "....."), ("6", "-...."), ("7", "--..."), ("8", "---.."),
  ("9", "----.")
];

// Function to convert text to Morse code
fn text_to_morse(text: &str) -> &str {
  for &(c, code) in MORSE_CODE.iter() {
    if text.eq_ignore_ascii_case(c) {
      return code;
    }
  }
  "" // Return empty string if character not found
}

// Function to play Morse code
async fn play_morse_code<T: Pin, U: Pin>(buzzer: &mut Output<'_, T>, led: &mut Output<'_, U>, text: &str) {
  for c in text.chars() {
    match c {
      '.' => {
        buzzer.set_high();
        led.set_high();
        Timer::after(Duration::from_millis(300)).await; // Delay for 300 ms
        buzzer.set_low();
        led.set_low();
        Timer::after(Duration::from_millis(200)).await; // Delay for 200 ms
      }
      '-' => {
        buzzer.set_high();
        led.set_high();
        Timer::after(Duration::from_millis(600)).await; // Delay for 600 ms
        buzzer.set_low();
        led.set_low();
        Timer::after(Duration::from_millis(200)).await; // Delay for 200 ms
      }
      ' ' => Timer::after(Duration::from_millis(800)).await, // Pause between letters
      _ => {} 
    }
  }
}


#[entry]
fn main() -> ! {
  embedded_main();
  loop {}
}


async fn embedded_main() -> ! {
  let p = init(Default::default());

  let mut buzzer = Output::new(p.PIN_15, Level::Low);
  let mut led = Output::new(p.PIN_16, Level::Low);

  let text = "PROJECT";

  // Convert text to Morse code and play
  let morse_text = text_to_morse(text);
  play_morse_code(&mut buzzer, &mut led, morse_text).await;

  ! 
}
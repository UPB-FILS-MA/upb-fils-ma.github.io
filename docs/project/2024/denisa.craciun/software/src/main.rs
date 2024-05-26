extern crate embedded_hal;
extern crate rppal;
extern crate st7735_lcd;

use std::thread::sleep;
use std::time::Duration;
use embedded_hal::blocking::spi::Write;
use rppal::gpio::Gpio;
use rppal::spi::{Spi, Bus, Mode, SlaveSelect};
use st7735_lcd::{Orientation, ST7735};

// Import custom modules and use statements for icon management
mod icon;
use icon::{Animate, Icon, Toolbar, Button, Event};

/// Clears the display by filling it with black.
fn clear(display: &mut ST7735<Spi>) {
    display.clear(Rgb565::BLACK).unwrap();
}

/// Builds the toolbar.
fn build_toolbar() -> Toolbar {
    let mut toolbar = Toolbar::new();
    toolbar.spacer = 2;
    toolbar.add_item(Icon::new("assets/food.pbm", 8, 8, "food"));
    toolbar.add_item(Icon::new("assets/lightbulb.pbm", 8, 8, "lightbulb"));
    toolbar.add_item(Icon::new("assets/game.pbm", 8, 8, "game"));
    toolbar.add_item(Icon::new("assets/firstaid.pbm", 8, 8, "firstaid"));
    toolbar.add_item(Icon::new("assets/toilet.pbm", 8, 8, "toilet"));
    toolbar.add_item(Icon::new("assets/heart.pbm", 8, 8, "heart"));
    toolbar.add_item(Icon::new("assets/call.pbm", 8, 8, "call"));
}

fn main() {
    fn main() {
    // Initialize I2C interface for the ST7735 display
    let i2c = I2c::new().unwrap();
    let mut display = ST7735::new(i2c, 0x3C, 128, 128);
    
    // Set up GPIO pin for control
    let rst = Gpio::new().unwrap().get(14).unwrap().into_output();
    
    // Initialize the display
    display.init().unwrap();
    display.set_orientation(&Orientation::Landscape).unwrap();

    // Initialize game variables
    let mut health = 1;
    let mut happiness = 1;
    let mut energy = 1;

    // Build the toolbar with icons
    let mut tb = build_toolbar();
    tb.set_position(0, 120); // Align toolbar to the bottom of the screen

    // Initialize animations
    let poopy = Animate::new(80, 80, 16, 16, "assets/poop.pbm");
    let baby = Animate::new(40, 40, 48, 48, "assets/baby_bounce.pbm", "bounce");
    let eat = Animate::new(40, 40, 48, 48, "assets/eat.pbm");
    let babyzzz = Animate::new_with_type(40, 40, 48, 48, "assets/baby_zzz.pbm", "loop");
    let death = Animate::new_with_type(24, 40, 16, 16, "assets/skull.pbm", "bounce");
    let go_potty = Animate::new_with_type(48, 40, 48, 48, "assets/potty.pbm", "bounce");
    let call_animate = Animate::new(84, 0, 16, 16, "assets/call_animate.pbm");

    // Initialize buttons with GPIO pin numbers
    let button_a = Button::new(4);
    let button_b = Button::new(3);
    let button_x = Button::new(2);

    // Set initial selected toolbar index
    let mut index = 0;
    tb.select(index, &mut display);
    let mut cancel = false;
    let mut feeding_time = false;
    let mut sleeping = false;
    death.set = true;

    // Set up game events
    let energy_increase = Event::new("Increase Energy", Icon::new(Path::new("assets/heart.pbm"), 16, 16, "heart"), 1);
    let firstaid_event = Event::new("First Aid", Icon::new(Path::new("assets/firstaid.pbm"), 16, 16, "firstaid"), 0);
    let toilet_event = Event::new("Toilet", Icon::new(Path::new("assets/toilet.pbm"), 16, 16, "toilet"), 0);
    let sleep_time = Event::new("sleep time", Icon::new(Path::new("assets/lightbulb.pbm"), 16, 16, "lightbulb"), 1);
    let heart_status = Event::new("Status", Icon::new(Path::new("assets/heart.pbm"), 16, 16, "heart"));

    // Configure animation settings
    baby.bounce();
    poopy.bounce();
    death.loop_anim(-1);
    death.set_speed("slow");
    babyzzz.set_speed("very slow");
    go_potty.loop_anim(1);
    go_potty.set = true;
    poopy.set = false;
    go_potty.load();
    }

    // Main game loop
    loop {
        if !cancel {
            tb.unselect(index, &mut display);
        }

        // Button A cycles through toolbar items
        if button_a.is_pressed() {
            index += 1;
            if index == 7 {
                index = 0;
            }
            cancel = false;
        }

        // Button X cancels current selection
        if button_x.is_pressed() {
            cancel = true;
            index = -1;
        }

        if !cancel {
            tb.select(index, &mut display);
        }

        // Button B handles toolbar item selection
        if button_b.is_pressed() {
            match tb.selected_item.as_str() {
                "food" => {
                    feeding_time = true;
                    sleeping = false;
                    baby.unload();
                }
                "game" => {
                    println!("game");
                }
                "toilet" => {
                    toilet_event.message = String::from("Cleaning...");
                    toilet_event.popup(&mut display);
                    poopy.set = false;
                    baby.set = true;
                    happiness += 1;
                    clear(&mut display);
                    poopy.unload();
                }
                "lightbulb" => {
                    if !sleeping {
                        sleeping = true;
                        babyzzz.load();
                        sleep_time.message = String::from("Night Night");
                        sleep_time.popup(&mut display);
                        clear(&mut display);
                    } else {
                        sleeping = false;
                        babyzzz.unload();
                    }
                    println!("lightbulb");
                }
                "firstaid" => {
                    firstaid_event.message = String::from("Vitamins");
                    firstaid_event.popup(&mut display);
                    health += 1;
                    clear(&mut display);
                }
                "heart" => {
                    heart_status.message = format!("health = {}", health);
                    heart_status.popup(&mut display);
                    heart_status.message = format!("happy = {}", happiness);
                    heart_status.popup(&mut display);
                    heart_status.message = format!("energy = {}", energy);
                    heart_status.popup(&mut display);
                    clear(&mut display);
                }
                "call" => {
                    call_animate.set = false;
                    println!("call");
                }
                _ => {}
            }
        }

        // Handle feeding time animation and logic
        if feeding_time {
            eat.load();
            if !eat.is_done() {
                eat.animate(&mut display);
            }
            if feeding_time && eat.is_done() {
                feeding_time = false;
                energy_increase.message = String::from("ENERGY + 1");
                energy_increase.popup(&mut display);
                energy += 1;
                clear(&mut display);
                eat.unload();
                baby.load();
            }
        } else {
            if sleeping {
                babyzzz.animate(&mut display);
            } else {
                if baby.set {
                    baby.load();
                    baby.animate(&mut display);
                }
                if go_potty.set {
                    go_potty.animate(&mut display);
                }
                if go_potty.is_done() {
                    println!("potty done");
                    go_potty.set = false;
                    poopy.set = true;
                    baby.load();
                    baby.bounce(-1);
                    baby.set = true;
                }
            }
        }

        // Check for game over condition
        if energy <= 1 && happiness <= 1 && health <= 1 {
            death.set = true;
        } else {
            death.set = false;
        }

        // Update call animation based on status
        if energy <= 1 || happiness <= 1 || health <= 1 {
            call_animate.set = true;
        } else {
            call_animate.set = false;
        }

        // Run animations based on conditions
        if poopy.set {
            poopy.load();
            poopy.animate(&mut display);
        }
        if death.set {
            death.animate(&mut display);
        }
        tb.show(&mut display);

        if index == 6 {
            tb.select(index, &mut display);
        } else {
            if call_animate.set {
                call_animate.animate(&mut display);
            }
        }

        // Flush display updates
        display.flush().unwrap();
        sleep(Duration::from_millis(50));
    }
}

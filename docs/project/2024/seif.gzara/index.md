# Weather App For the future.

:::info 

**Author**: GZARA Seif \
**GitHub Project Link**: (https://github.com/UPB-FILS-MA/project-seifgz)

:::

## Description

A robot that displays the weather of the day and makes it readable on an lcd screen and upgradable to be hearable on a small speaker.

## Motivation

I remember there was a rainy day where i had to do some groceries shopping and found this old blind lady that was drenched from the rain that turned out to be my neighbor, i helped her get home and she told me her life story and that it's very difficult to do very simple things such as taking an umberalla on a rainy day, that's not when i decided to become a programmer but since i'm becoming one now i remembered this instance and i wanna do something about it so i'm gonna create this small useful robot.

## Architecture 

Raspberry Pi Pico W: The Raspberry Pi Pico W serves as the central microcontroller unit. It can be programmed to control various functions and interfaces with other components.
16x2 I2C LCD Display: The LCD display can be connected to the Raspberry Pi Pico W via the GPIO pins. Since it uses the I2C communication protocol, we need to connect its SDA (data) and SCL (clock) pins to the corresponding GPIO pins on the Pico W. Additionally, power (VCC) and ground (GND) connections are necessary.
Jumper Wires: These are used to make the connections between the Raspberry Pi Pico W, the LCD display, and any other components on the breadboard. They're essential for establishing temporary electrical connections.
Breadboard: The breadboard provides a platform for mounting and connecting the various components. The Raspberry Pi Pico W can be placed on the breadboard, and jumper wires can be used to connect its GPIO pins to the appropriate rows and columns on the breadboard. Other components can also be connected to the breadboard using jumper wires as needed.
Micro-USB Cable: This is used to power the Raspberry Pi Pico W and establish a connection with a computer or power source for programming and data transfer. The cable connects to the micro-USB port on the Pico W and the USB port on the computer or power adapter.

## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

Raspberry Pi Pico W: A wireless-enabled microcontroller board for embedded projects.

16x2 I2C LCD Display: A compact text display module for showing information in DIY electronics projects.

Jumper Wires: Essential for making temporary connections between components on a breadboard.

Breadboard: A solderless prototyping board for building and testing electronic circuits.

Micro-USB Cable: Used for powering and connecting devices like the Raspberry Pi Pico W to a computer or power source.

### Schematics

Place your KiCAD schematics here.

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |

| [LCD SCREEN](https://cleste.ro/ecran-lcd-1602-iic-i2c.html?utm_medium=GoogleAds&utm_campaign=&utm_source=&gad_source=1&gclid=Cj0KCQjwir2xBhC_ARIsAMTXk84gnt7kedeHZoojkhaEFgfC5FckUBmY7FJZONkuk1zFoACiWBMxrTEaAkoAEALw_wcB) | The Screen | [28 RON](https://cleste.ro/ecran-lcd-1602-iic-i2c.html?utm_medium=GoogleAds&utm_campaign=&utm_source=&gad_source=1&gclid=Cj0KCQjwir2xBhC_ARIsAMTXk84gnt7kedeHZoojkhaEFgfC5FckUBmY7FJZONkuk1zFoACiWBMxrTEaAkoAEALw_wcB) |

| [Breadboard](https://cleste.ro/breadboard-400-puncte.html?gad_source=1&gclid=Cj0KCQjwir2xBhC_ARIsAMTXk85FFCbq9EuD_029Or_Gg659DUnbj0Owr4K_b9maB8LJiks2TIHooaUaAkohEALw_wcB) | The microcontroller | [3 RON](https://cleste.ro/breadboard-400-puncte.html?gad_source=1&gclid=Cj0KCQjwir2xBhC_ARIsAMTXk85FFCbq9EuD_029Or_Gg659DUnbj0Owr4K_b9maB8LJiks2TIHooaUaAkohEALw_wcB) |

| [Micro-USB Cable ](https://www.f64.ro/cablu-baseus-cafule-usb-micro-usb-2-4a-0-5m-gri-negru/p?gad_source=1&gclid=Cj0KCQjwir2xBhC_ARIsAMTXk86fA0iMZKq-Vf69DcwjFG-B384hStRKYSYWuRUMVEpTIEH5wtDNm7gaAudPEALw_wcB) | The USB Cable | [18 RON](https://www.f64.ro/cablu-baseus-cafule-usb-micro-usb-2-4a-0-5m-gri-negru/p?gad_source=1&gclid=Cj0KCQjwir2xBhC_ARIsAMTXk86fA0iMZKq-Vf69DcwjFG-B384hStRKYSYWuRUMVEpTIEH5wtDNm7gaAudPEALw_wcB) |

| [Jumper Wires ](https://www.distrelec.ro/ro/jumper-wire-male-to-female-10-st-150-mm-multicoloured-rnd-rnd-255-00013/p/30115111?cq_src=google_ads&cq_cmp=18923290032&cq_con=&cq_term=&cq_med=pla&cq_plac=&cq_net=x&cq_pos=&cq_plt=gp&gad_source=1&gclid=Cj0KCQjwltKxBhDMARIsAG8KnqVSr-nvPIsdO7amnvvvBrSmOIXRjgAia9qHVP2dNyW8QDwB6g-IvwYaAt5EEALw_wcB&gclsrc=aw.ds) | The wires | [5 RON](https://www.distrelec.ro/ro/jumper-wire-male-to-female-10-st-150-mm-multicoloured-rnd-rnd-255-00013/p/30115111?cq_src=google_ads&cq_cmp=18923290032&cq_con=&cq_term=&cq_med=pla&cq_plac=&cq_net=x&cq_pos=&cq_plt=gp&gad_source=1&gclid=Cj0KCQjwltKxBhDMARIsAG8KnqVSr-nvPIsdO7amnvvvBrSmOIXRjgAia9qHVP2dNyW8QDwB6g-IvwYaAt5EEALw_wcB&gclsrc=aw.ds) |

## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded_hal](https://github.com/embassy-rs/embassy) | Hardware abstraction layer for embedded systems | Used for abstracting hardware interactions |
| [embassy_rp](https://github.com/embassy-rs/embassy) | Provides abstractions and APIs|  Communicate via UART, SPI, and I2C using their respective modules |
| [embassy_time](https://github.com/embassy-rs/embassy) | Provides time-related functionality | Schedule tasks to run at specific times |
| [embassy_executor](https://github.com/embassy-rs/embassy) | Provides an executor for running asynchronous tasks concurrently | Execute asynchronous tasks concurrently  |
| [embassy_sync](https://github.com/embassy-rs/embassy) |Provides synchronization primitives | Create mutexes, semaphores, and barriers for coordinating access to shared resources |
| [Python](https://github.com/python) |Python serves as the programming language to interact with the hardware components, allowing you to create a wide range of embedded projects|

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [How2electronics](https://how2electronics.com/raspberry-pi-pico-w-iot-weather-station-openweathermap/)
2. [Youtube](https://youtu.be/3q807OdvtH0?si=4Sr_54eqjX7ew_TJ)
...

# Project Name


**Author**: Oblu Alexandra \
**GitHub Project Link**: link_to_github


## Description

This project aims to create a clock-in device using a Raspberry Pi Pico W microcontroller and a display module.

## Motivation

Why did you choose this project?

## Architecture 

The main components of the project include:

    Raspberry Pi Pico W microcontroller
    LCD display module

The microcontroller controls the display module to show the user interface for clocking in.
## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

Detail in a few words the hardware used.

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
| [Push buttons](https://components101.com/switches/push-button) | to clock-in with them | [1 RON](https://ardushop.ro/ro/home/97-buton-mic-push-button-trough-hole.html) |
| [Real-Time Clock (RTC) Module](https://components101.com/modules/ds3231-rtc-module-pinout-circuit-datasheet) | to remember the time even if the pico is off | [24 RON](https://ardushop.ro/en/electronics/231-precision-rtc-module-ds3231.html?search_query=rtc&results=12) |
| [LCD Display](https://www.farnell.com/datasheets/58820.pdf) | to show who clocked-in | [10 RON](https://ardushop.ro/ro/electronica/36-lcd-1602.html) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | Hardware abstraction layer | Used for interfacing with hardware peripherals such as GPIO pins and SPI for the LCD display |
| [rp-hal](https://github.com/rp-rs/rp-hal) | HAL for Raspberry Pi Pico | Provides higher-level abstractions for peripherals on the Raspberry Pi Pico |

## Links


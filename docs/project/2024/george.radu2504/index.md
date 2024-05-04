
# Smart lamp
The project involves creating a smart lamp that responds to motion and ambient light levels. 

:::info 

**Author**: Radu George-Alexandru
**GitHub Project Link**: link_to_github

:::

## Description

This project revolves around creating a smart lamp system that enhances both convenience and energy efficiency. Utilizing the Raspberry Pi Pico W as its core, along with a motion sensor and a photoresistor, the lamp dynamically responds to its environment. By detecting motion, it ensures that illumination is provided when needed. Additionally, the led turns on/off based on ambient light levels, conserving energy when sufficient natural light is available and optimizing comfort when there is not enough light. This integration of motion and light sensing technology transforms the lamp into a versatile and adaptive lighting solution for any space.


## Motivation

I choose to persue this smart lamp project because I was thinking that one of the most important facilities of a smart house are the lights. This project tackles energy efficiency by innovating traditional lighting with a Raspberry Pi Pico W and motion/light sensors. It's a technical challenge that excites me, merging hardware and software seamlessly, so that I can craft a smart home solution that contributes to a mindful energy consumption.

## Architecture 

![Rasberry Pi Pico W (3)](https://github.com/George2543/upb-fils-ma.github.io/assets/154756379/c2007868-f619-4784-a06f-4510111d4b02)

Main components:
- Raspberry Pi Pico W: Is the control unit, it manages the motion sensor, photoresistor and LED
- Magnetic sensor Hall: Detects motion within its range
- Photoresistor: Measures ambient light levels
- LED

 Connection between components:
 - Motion sensor to microcontroller: The motion sensor is connected to the Raspberry Pi Pico W through GPIO pins.
 - Photoresistor to microcontroller: The photoresistor is directly connected to the Raspberry Pi Pico W.
 - LED to microcontroller: The LED is connected to the Raspberry Pi Pico W, serving as the output. Through the microcontroller, the LED is controlled based on the inputs received from the magnetic motion sensor and photoresistor.
 

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


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...

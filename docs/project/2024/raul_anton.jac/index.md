# MeloLED
MP3 Player with LED strip using Pico RP2040

:::info 

**Author**: Raul-Anton Jac \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-jacraul

:::

## Description

The project aims to create a multifunctional MP3 player using the Raspberry Pi Pico microcontroller, enhanced with vibrant LED lighting effects for added ambiance. The MP3 player will allow users to enjoy their favorite music tracks with ease, featuring controls for play, pause, previous song, next song, and volume adjustment. Additionally, the project will incorporate an infrared (IR) remote control for convenient playback management and to control the LED Bar. The LED lighting effects will be synchronized with the music, providing an immersive audiovisual experience tailored to the user's preferences.Depending on time, I hope I will add an application for controlling it at distance.

## Motivation

I chose this project because I am a fan and I have a blog about Eurovision Song Contest. I want to create something which is useful for me and in the same time something very nice and colorful.

## Architecture 

![Architecture photo](./schematicsforcomponents.png)


 **Raspberry Pico RP2040**
    - "the brain", controlls everything
    - it is connected with every single module, led strip

 **1602 LCD**
    - interface: I2C
    - connections: GP0 (SDA), GP1 (SCL)
    - it will display the artist and the song playing

 **IR Receiver**
    - connection: GP15
    - it will transmit to pico the request from remote control

 **Micro SD Module**
    - interface: SPI
    - connections: GP13 (CS), GP12 (MISO), GP11 (MOSI), GP10 (SCK)
    - it will read and write on Micro SD Card

 **Potentiometer**
    - connection: GP26 (Analog Out)
    - it will control the volume of the player

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


| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [1602 LCD with I2C Interface and Yellow-Green Backlight](https://www.optimusdigital.ro/en/lcds/62-1602-lcd-with-i2c-interface-and-yellow-green-backlight.html) | LCD Screen for artist & song| [15 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [WS2812 RGB LED Bar (8 LEDs)](https://www.optimusdigital.ro/en/led-bars/753-bara-de-led-uri-rgb-ws2812-cu-8-led-uri.html) | LED bar for ambient | [6 RON](https://www.optimusdigital.ro/en/led-bars/753-bara-de-led-uri-rgb-ws2812-cu-8-led-uri.html) |
| [2 x 20p 2.54 mm Male Pin Header](https://www.optimusdigital.ro/en/pin-headers/8445-20-x-2p-254-mm-male-pin-header.html) | Pins for PICO | [1 RON](https://www.optimusdigital.ro/en/pin-headers/8445-20-x-2p-254-mm-male-pin-header.html) |
| [MicroSD Card Slot Module](https://www.optimusdigital.ro/en/memories/1516-microsd-card-slot-module.html) | MicroSD reader for songs | [5 RON](https://www.optimusdigital.ro/en/memories/1516-microsd-card-slot-module.html)|
| [100k Mono Potentiometer](https://www.optimusdigital.ro/en/potentiometers/1887-100k-mono-potentiometer.html?search_query=potentio&results=223) | Controlling the volume of player | [2 RON](https://www.optimusdigital.ro/en/potentiometers/1887-100k-mono-potentiometer.html?search_query=potentio&results=223)|
| [Infrared Remote Receiver Module](https://www.optimusdigital.ro/en/others/755-modul-receptor-telecomanda-infrarou.html) | Infrared sensor for remote control | [9 RON](https://www.optimusdigital.ro/en/others/755-modul-receptor-telecomanda-infrarou.html)|
| [Mini Infrared Remote](https://www.optimusdigital.ro/en/others/11-mini-infrared-remote.html) | Remote Control for player and led bar color and modes| [4 RON](https://www.optimusdigital.ro/en/others/11-mini-infrared-remote.html)|
| [Plusivo Resistor Kit 250 pcs](https://www.optimusdigital.ro/en/resistors/10928-250-pcs-plusivo-resistor-kit.html?search_query=resistors&results=184) | Used for Voltage Divider | [15 RON](https://www.optimusdigital.ro/en/resistors/10928-250-pcs-plusivo-resistor-kit.html?search_query=resistors&results=184)|
| [Breadboard Jumper Wires Set](https://www.optimusdigital.ro/en/wires-with-connectors/12-breadboard-jumper-wire-set.html?search_query=wires&results=561) | Connecting components | [8 RON](https://www.optimusdigital.ro/en/wires-with-connectors/12-breadboard-jumper-wire-set.html?search_query=wires&results=561)|
| [2 x Breadboard HQ (830 points)](https://www.optimusdigital.ro/en/breadboards/8-breadboard-hq-830-points.html) | Base for the project | [20 RON](https://www.optimusdigital.ro/en/breadboards/8-breadboard-hq-830-points.html)|
| [Speaker](#) | Song output | [TBA](#)|


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [FatFS](https://github.com/rafalh/rust-fatfs) | Foundation for drivers | Used for components to communicate with software |
| [embedded-hal](https://github.com/rust-embedded/embedded-hal) | A lightweight file system library for embedded systems, compatible with FAT32, FAT16, and FAT12 | Handles reading and writing audio files to and from the microSD card efficiently |


## Links

1. [Arduino iPod](https://ocw.cs.pub.ro/courses/pm/prj2023/avaduva/arduino-ipod)
...

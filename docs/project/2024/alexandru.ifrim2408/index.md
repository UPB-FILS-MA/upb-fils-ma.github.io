# LED MIDI Controller with FL Studio
A LED MIDI Controller with FL Studio using Raspberry Pico W, that synchronizes music and lights.

:::info 

**Author**: Ifrim Alexandru-Constantin \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-pitisef

:::

## Description

Here's a simple project that adds fun to music-making in FL Studio. We'll sync music and lights, just like syncing your ears and eyes. We'll use FL Studio, download some MIDI libraries, and program in Rust. These MIDI Controllers are often used in the industry but with more add-ons and features, but for now we will create a simple one using 10 Resistors and LEDS, a Raspberry Pico W and of course the basics. (wires, breadboard and so on)

## Motivation

I chose this project due to my past experience in FL Studio and my desire to work in the music production industry. Moreover, I find this LED MIDI Controller useful to me, because I really wanted to buy one quite expensive, but the pleasure to make one, even though it will not reach the same standards, would make me even proud of myself. Music is my passion, might i say one of my hobbies and every interaction with it, even if I'm clueless about it makes me interested.

## Architecture 
[LED MIDI Controller](./architecture.jpeg)
## Log


### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

    

### Schematics

Place your KiCAD schematics here.

### Bill of Materials


| Device | Usage | Price |
|--------|--------|-------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Headers 2.54](https://en.wikipedia.org/wiki/Pin_header) | The microcontroller | [1 RON](https://www.optimusdigital.ro/ro/componente-electronice-headere-de-pini/463-header-de-pini-alb-254-mm-40p.html) |
| [Starter Kit also having an Arduino](https://en.wikipedia.org/wiki/Arduino) | Wires, Breadboard, Resistors, LEDS | [80 RON](https://www.optimusdigital.ro/en/optimus-digital-kits/7356-kit-wireless-super-starter-cu-esp8266.html?search_query=Wireless+Super+Starter+Kit+with+ESP8266+%28Programmable+with+Arduino+IDE%29%09&results=1) |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |
 [hairless-midi](https://projectgus.github.io/hairless-midiserial/) | hairless midi Sodtware | Used for connecting the virtual midi to the serial of the Raspberry PI Pico W and debug the messages too. |
  [loop-midi](https://www.tobias-erichsen.de/software/loopmidi.html) | loop midi Software | Used for creating a virtual midi devise to the system in order to connect it to fl-studio |
  [FL-Studio](https://www.image-line.com/fl-studio-download/) | Music Production Software | Used for configuring MIDI |
## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [MIDI Controller LED STRIP (my first idea of the project)](https://www.youtube.com/watch?v=iUlPr0yWuOM)
2. [MIDI Controller using Arduino and FL Studio](https://www.youtube.com/watch?v=yCTj0BCWrsk)
...
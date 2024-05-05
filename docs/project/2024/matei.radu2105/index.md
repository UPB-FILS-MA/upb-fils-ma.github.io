# Snakes and Apples
My project has the goal of using a Raspberry Pi to simulate the old video game Snake.

:::info 

**Author**: Radu Matei \
**GitHub Project Link**: [This is my project.](idk how to find the link or how to use github properly)

:::

## Description

In my project I want to recreate the original game simply known as "Snake" on a Raspberry Pi,
using an LCD display as a screen and four buttons as the movements of the player-controlled snake. 
The fifth and final button will only be for starting the game or for stopping it. The game consists
of a snake that can move up, left, down or right and of randomly generating apples that the snake 
has to eat in order to get bigger and, therefore, gain a higher score. The display will work using
Embedded-graphics, displaying for the whole duration of the game the game area, where apples can spawn
and the player can move to get them, and at the top, separated from the rest of the game, there will
be a score counter and a current high score. The game stops when the snake collides with its own head
or one of the borders. During the game, buzzers will be used to create sound effects when gaining score
or dying, and for the background music.

## Motivation

Ever since I was a child, I have been playing video games - from the first ever look over my dad's shoulder to see
what he was doing on the computer I was hooked. I've been playing them my whole life ever since, and since 
high school, where I discovered coding and that I was very good at it, my dream became very clear to me:
creating video games. I have done my fair share of coding problems and even games in Unity, and the
software part is what I really love. And now, I have the opportunity to make a game, code it, and
then make a physical machine that will run my game. Physically. That sounds amazing and, even if
hardware is not really my specialty, I will do my best to make the best game I can.

## Architecture 

![Project_architecture](https://github.com/matei7-7/upb-fils-ma.github.io/assets/163386907/7c0f97c5-45d2-4cf8-942e-5c31e0257cbe)

Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other

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
|test|test again|1 ron|

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

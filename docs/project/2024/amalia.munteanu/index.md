# The Hangmaster
A hangman game programmed on a Raspberry Pi Pico. 

:::info 

**Author**: Munteanu Amalia-Nicole \
**GitHub Project Link**: link_to_github

:::

## Description

My project is based on an old timey game that we all used to play during school hours and even in our free time with our friends, the hangman. The game's purpose is to crrate a fun enviroment in which people can relax and rewind old memories from their childhood. The player is given a word and must guess the letters it consists of. For example, if a player choose a letter which is part of the chosen word, the positions of the letter will be revealed to the player. However, if the letter is not part of the word, a red LED will light up. When all 3 LEDs on the breadboard are lit up, the player has lost, and the game is over and a buzzer will go off. The game will be able to function on a Raspberry Pi, using a LCD display which uses Embeded-graphics, a buzzer, a keypad, a potentiometer, 3 LEDs and a button for starting the game and stopping it. 

## Motivation

I've always taken a big interest in the game development area, and thought it would be fun to try challenging myself by coding a game designed to work on a microprocessor in a newly encountered programming language, rust. The question was which game. Well, it all came around when one day, during free hours, my friends and I decided to play Hangman and thought it would be a great idea to revolve my project on this timeless game.

## Architecture 
<img width="443" alt="Architecture1" src="https://github.com/nikkoxp/upb-fils-ma.github.io/assets/163386570/967ee4db-efe6-4d7e-8d41-958a847f65ab">


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

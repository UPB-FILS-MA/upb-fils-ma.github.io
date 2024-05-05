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
| [LCD 1602 cu Interfata I2C si Backlight Albastru] | The Display | [16,34 RON](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/2894-lcd-cu-interfata-i2c-si-backlight-albastru.html) |
| [Wires father-father] | The Wires Father-Father | [6,99 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/884-set-fire-tata-tata-40p-10-cm.html?search_query=fire&results=437) |
| [Wires mother-mother] | The Wires Mother-Mother | [7,73 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/652-fire-colorate-mama-mama-40p-10-cm.html?search_query=fire&results=437) |
| [Buzzer] | The Buzzer | [1,95 lei RON](https://www.optimusdigital.ro/ro/audio-buzzere/12247-buzzer-pasiv-de-33v-sau-3v.html?search_query=buzzer&results=62) |
| [Tastatură Matriceală 4x4 cu Butoane] | The Keypad | [3,99 RON](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/2894-lcd-cu-interfata-i2c-si-backlight-albastru.html) |
| [Potentiometru Stereo 10k] | The Potentiometer | [1,99 RON](https://www.optimusdigital.ro/ro/componente-electronice-potentiometre/1886-potentiometru-stereo-10k.html?search_query=Potentiometru+stereo+10k&results=2) |
| [Breadboard HQ (830 Puncte)] | The Breadboard | [9,98 RON](https://www.optimusdigital.ro/ro/prototipare-breadboard-uri/8-breadboard-830-points.html?search_query=Breadboard&results=145) |
| [LED Roșu de 3 mm cu Lentile Difuze] | The LEDs | [0,39 RON](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/696-led-rou-de-3-mm-cu-lentile-difuze.html?search_query=LED&results=818) |


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

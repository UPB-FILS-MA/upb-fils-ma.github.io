# Morse Code Translator
    
A two Raspberry Pi Morse Code translator system: one converts English to Morse, blinking LED and buzzer; the other translates Morse back to English displayed on an LCD via microphone detection.

:::info 

**Author**: Boghici Ionut-Daniel \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-illusion2626

:::

## Description

A Morse Code translator using an LED, a buzzer and a microphone;

It uses two Raspberry Pico Pi's. One receives input from a form on a browser, in english latin, translates it into morse then displays it via a blinking LED and a buzzer that makes a sound in unison with the LED's blinks. The other uses a microphone that listens for the buzzes, it "captures" them and then translates those back into english latin. Finally, the translated buzzes are shown on a LCD Display.

## Motivation

I decided to create a Morse code translator because I've always been fascinated by its simplicity yet effectiveness as a means of communication. Plus, it's a great way to blend my interest in coding with a practical application that could potentially be useful in various scenarios.

## Architecture 

Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other

## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

An LED that blinks corresponding to the translated letter
A Buzzer that, like the previous, makes sounds corresponding to the translated letter
A Microphone that listens for sounds, which get translated back into english
An LCD display that shows the translated text on it  

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
| 2* [Rapspberry Pi Pico WH](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontrollers | [39 * 2 = 78 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/12395-raspberry-pi-pico-wh.html?search_query=raspberry+pi+pico+wh&results=24) |
| 10* LED | (https://www.optimusdigital.ro/ro/optoelectronice-led-uri/696-led-rou-de-3-mm-cu-lentile-difuze.html?search_query=led&results=818) | Used to display the Morse-translated words | [0.39 RON]
| 1* Buzzer | (https://www.optimusdigital.ro/ro/audio-buzzere/10-modul-cu-buzzer-activ.html?gad_source=1&gclid=Cj0KCQjw_qexBhCoARIsAFgBletwqrU5imri4tTqjWDFAvdBdadg_4NQYl-zPxhmkUOvEQEgqrzdgXoaAjJsEALw_wcB) | Used to display the Morse-translated words | 
| 1* LCD Module | (https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/62-lcd-1602-cu-interfata-i2c-si-backlight-galben-verde.html?search_query=lcd+i2c&results=18) | Used to display the words that were translated from Morse back to english | [15 RON]
 


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Simple translator](http://warmcat.uk/?p=400)


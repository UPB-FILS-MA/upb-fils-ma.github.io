# Pulse Oximeter Health Monitoring

:::info

**Author**: Buga Mihai
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-MishulicaZeu

:::

## Description

I want to make a pulse oximeter with a Oled display. The purpose of the project is to monitor a person's pulse and oxygenation.


## Motivation
I choose this project because I wanted to make something what it could be usefull also later in life outside getting my grade.

## Architecture 
![image](architecture.png)



## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

The hardware part consisted of the following steps:

1. Powered the Raspberry Pi Pico through the USB port of the laptop.
2. Connected the LCD and verified its functionality.
3. Connected the pulse sensor.
4. Displayed the data received from the MAX30100 sensor on the LCD screen.
5. Added a buzzer that emits a sound each time the pulse sensor detects a heartbeat.

### Schematics

Place your KiCAD schematics here.( at this moment I have a problem with kicad I will update it later)

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
 [buzzer](https://www.bitmi.ro/module-electronice/modul-buzzer-activ-compatibil-arduino-10397.html) |used for creating sound| |[5 lei]|
 [Breadboard 830 puncte MB-102] |(https://www.bitmi.ro/componente-electronice/breadboard-830-puncte-mb-102-10500.html) |to assamble the Project| |[9 lei]|
 [Modul senzor pulsoximetru MAX30102] |(https://www.bitmi.ro/senzori-electronici/modul-senzor-pulsoximetru-max30102-10803.html) | to measure-heart-rate-and-spo2-with-max30102-eb4f74| |[16 lei]|
## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [inspired](https://all3dp.com/2/easy-simple-arduino-projects/)
2. [project ideea](https://projecthub.arduino.cc/SurtrTech/measure-heart-rate-and-spo2-with-max30102-eb4f74)
...
# Project Name

:::info 

**Author**: El-Ghoul Layla \
**GitHub Project Link**: link_to_github

:::

## Description

The project revolves around a multi-storey parking lot. At the entrance, an infrared sensor will detect the presence of vehicles. After the detection of a vehicle a servomotor barrier will allow its entry. Infrared sensors will be implanted at each parking lot to monitor occupation.There will be green leds indicating available parking spaces, and red ones indicating occupancy.The second floor will be designated for authorized vehicles only.To make this happen, an RFID system will be utilized. In case an unauthorized vehicle tries to access, a buzzer will be activated alerting security personnel. When an authorized vehicle is detected, an LED light will turn on. The exit will also implement an infrared sensor and a servomotor barrier. A flame sensor will indicate the presence of fire, in case of fire a buzzer will be activated and the servomotors will automatically open to enable evacuation procedures.


## Motivation

These days, cars have become a necessity rather than a luxury. People rely on them for their everyday activities, whether it's going to work, running errands, or simply enjoying some leisure time. One major concern for them is finding a place to park wherever they go. Whether it's for work, fun, or shopping, having a convenient and safe parking space is crucial for their comfort and peace of mind.
I picked this project because it addresses this concern. My project focuses on making parking lots safer, ensuring a better parking experience for everyone.

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
|[Led 5mm](https://www.farnell.com/datasheets/1498852.pdf)| Red Led light| [0.26 RON](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/700-led-rou-de-3-mm-cu-lentile-transparente.html)|
| [Led 5mm](https://www.farnell.com/datasheets/2861534.pdf) | Green Led light | [0.26 RON](https://www.optimusdigital.ro/ro/optoelectronice-led-uri/931-led-verde-de-3-mm-cu-lentile-transparente.html?search_query=led+verde&results=93) |
|[Micro Servomotor SG90 90°](http://www.ee.ic.ac.uk/pcheung/teaching/DE1_EE/stores/sg90_datasheet.pdf)| The micro servomotor | [14 RON](https://www.optimusdigital.ro/ro/motoare-servomotoare/26-micro-servomotor-sg90.html)|
| [Modul cu Buzzer activ](link://to/device) | Active buzzer | [2,49 RON](https://www.optimusdigital.ro/ro/audio-buzzere/10-modul-cu-buzzer-activ.html) |
| [Male to male jumper wires](link://to/device) | Wires | [6.98 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/888-set-fire-tata-tata-40p-20-cm.html?search_query=fire+tata+tata&results=80) |
| [Male to male jumper wires](link://to/device) | Wires | [6.98 RON](https://www.optimusdigital.ro/ro/fire-fire-mufate/880-fire-colorate-mama-mama-10p-10-cm.html?search_query=fire+mama+mama&results=63) |
| [Infrared Sensor ](link://to/device) | infrared sensor | [8,78 RON](https://ardushop.ro/ro/electronica/41-modul-senzor-ir-infrarosu-evita-obstacole.html) |
| [ServoMotor](link://to/device) | Micro Servomotor SG90 90° | [13,99 RON](https://www.optimusdigital.ro/ro/motoare-servomotoare/26-micro-servomotor-sg90.html?search_query=servo&results=194) |
| [Infrared Flame Sensor ](link://to/device) | flame sensor | [2,49 RON](https://www.optimusdigital.ro/en/optical-sensors/110-ir-flame-sensor.html) |
| [Modul RFID RC522 ](link://to/device) | Rfid RC522 | [2,49 RON](https://www.optimusdigital.ro/ro/wireless-rfid/67-modul-cititor-rfid-mfrc522.html?search_query=rfid&results=44) |



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [PM projects 2023](https://ocw.cs.pub.ro/courses/pm/prj2023)
2. [link](https://example3.com)
...

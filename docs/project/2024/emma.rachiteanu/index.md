# Robotic Tic Tac Toe
An interactive game of tic tac toe against a robotic arm.

:::info 

**Author**: Emma Răchițeanu \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-emma-rachiteanu

:::

## Description

Robo Tic Tac Toe offers an engaging gameplay where a human player competes against a pre-programmed robotic arm. For simplicity reasons, the game is designed to have a display of the game to which the robotic arm points its move. The player uses a control panel on a mobile website, each corresponding to a different space on the game board where the move will be executed. This information is then transmitted to the computer for it to make its move. The robotic arm operates using servo motors to maneuver 3D-printed components that form the arm's structure.

## Motivation

As a game enthusiast and tech geek, I wanted something to combine both of them. As only a game would be too little for a project, I decided to make a robotic arm to play with. As tic tac toe is a fairly easy game to program, as there are many examples online for it, 

## Architecture 

![Block Diagram](img/front%20view%20robotic%20arm.jpg)
![Block Diagram](img/side%20view%20robotci%20arm.jpg)
![Block Diagram](img/schematic.jpg)
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other

## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

The robotic arm will use for the joints mainly servo-motors, which can lift up to 750g at 5V. As such, the structures will be made up of 3D printed components with as little infill as possible.

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
| [Motor Servo (probably) SG90 9g](https://datasheetspdf.com/datasheet-pdf/791970/SG90.html) | The servomotors | [15 RON](https://www.emag.ro/servomotor-sg90-180-de-grade-ai0156-s297/pd/D33V1GMBM/) |



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://docs.sunfounder.com/projects/ultimate-sensor-kit/en/latest/components_basic/27-component_servo.html)
2. [link](https://github.com/Makerfabs/PICO_Merchanical_Hand_Driver)
...

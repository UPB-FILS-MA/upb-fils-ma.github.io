# Robo Tic Tac Toe
An interactive game of tic tac toe against a robotic arm.

:::info 

**Author**: Emma Răchițeanu \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-emma-rachiteanu

:::

## Description

Robo Tic Tac Toe provides engaging gameplay in which a human player competes against a pre-programmed robotic arm. For simplicity, the game features a display board to which the robotic arm points to indicate its move. Players use a control panel on a website, with each control corresponding to a different space on the game board where the move will be executed. This information is transmitted via Wi-Fi to the micro-controller, enabling it to make its move.

## Motivation

For this project, I aimed to create something both fun and intricate. Merely creating a game felt insufficient, so I decided to build a robotic arm to enhance the interaction. I chose tic tac toe because it is relatively straightforward to program, making it the ideal game for this task. I found this approach to be both innovative and enjoyable.

## Architecture 

- _Microcontroller_: The Raspberry Pi Pico microcontroller handles input from the mobile website and processes it to determine subsequent moves. The programming aims to maximize the robotic arm's chances of winning.
- _Display_: This component displays game data on a 9x9 grid filled with X's and O's. The display is sized to allow for pointer inaccuracies.
- _Mobile website_: This site features a simple 9x9 grid design, acting as the control interface for the player. Inputs are transmitted to the microcontroller via Wi-Fi.
- _Robotic arm_: After the microcontroller decides on a move, the robotic arm points to one of the nine grid positions. Both servos adjust continuously to ensure accuracy. The lower servo is equipped with an L-shaped component to enable slight rotation, while the upper servo has a longer attachment to ensure it can reach all positions.
![Block Diagram](img/front%20view%20robotic%20arm.jpg)
![Block Diagram](img/side%20view%20robotci%20arm.jpg)
![Block Diagram](img/schematic.jpg)

## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

1. *]The Arm
The robotic arm functions using servo motors capable of lifting up to 750g at 5V, which maneuver 3D-printed components forming the structure of the arm. These components are designed with minimal infill to ensure they are lightweight enough for smooth movement. The pointer is mounted loosely on an axis, allowing it to maintain vertical orientation consistently. Each component attached to a servo is elongated, with the excess length acting as a counterweight.

2. Display 
The display is directly mounted on the board.

### Schematics

Place your KiCAD schematics here.

### Bill of Materials

<!-- 
| [Device](link://to/device) | This is used ... | [price](link://to/store) |
-->

| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Motor Servo (probably) SG90 9g](https://datasheetspdf.com/datasheet-pdf/791970/SG90.html) | The servomotors | [15 RON](https://www.emag.ro/servomotor-sg90-180-de-grade-ai0156-s297/pd/D33V1GMBM/) |
| [ Display LCD TFT ](link://to/device) | Display of the game | [64 RON](hhttps://ardushop.ro/en/home/1029-display-lcd-tft-32-320x240-with-touch-for-stm32.html?search_query=display+3%22&results=87) |




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

# Wifi Robot Car

:::info 

**Author**: Dumitru Teodora-Iulia \
**GitHub Project Link**: [link_to_github](https://github.com/UPB-FILS-MA/project-Teodora1807.git)

:::

## Description

I intend to do a robot car that can be controlled through a website accessed from the mobile phone or a computer . The website will have on the home page a controller with all 4 directions of movement.

Welcome to the documentation for a Wi-Fi robot car, powered by the Pico W microcontroller and programmed using the Rust programming language. This comprehensive guide will present, at the final deadline, the assembly, setup, and programming of a Wi-Fi-enabled robot car. In this project, I aim to build a Wi-Fi robot car. The robot car will be capable of wireless communication over a Wi-Fi network, enabling remote control and autonomous operation. This project, if everything works at the end, can be stilized with specific components(like a proper car kit) for a more stylished look.

## Motivation

When I was little, besides dolls and legos , i liked remote controlled cars or even helicopters so this is more like an ambition for me.

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

The robot will have a microcontroller board like Pico w which support Wi-Fi and is connected to a Wi-Fi module or module with the Wi-Fi function integrated, that allows the car to connect to a Wi-Fi network and receive data from a smartphone. I am going to use the pico w microcontroller, a motor driver module, 2 motors, 4 rechargeable batteries but only 2 used simutaneous,a battery charger and a battery holder, the skelet of the car(2 big wheels + a small one for the front,the platform) , jumper wires and also a switch so that the battery can last longer. I will also design a web interface to make possible the automatic movement of the car. For the assembly part i will use screws for the skeleton of the care and fludor for the components that need to be glued.
More details for each component and the ispiration behind them can be seen on the Readme file on github.

### Schematics

Place your KiCAD schematics here.

### Block diagram

[block diagram](image.png)

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
| [2 wheels,1 small wheel, 2 motors, platform ] | The kit | [15 RON](second-hand) |
| [Battery Charger for 2] | To recharge the used batteries | [24,99 RON](https://www.optimusdigital.ro/ro/incarcatoare-de-baterii/11021-incarcator-1865026650-dublu-cu-cablu-usb-pentru-baterii-cu-litiu-ion.html) |
| [4 x Batteries] | The power source of the whole project | [68 RON](https://www.dedeman.ro/ro/acumulator-li-ion-well-18650-3-7v-2200-mah/p/1050265) |
| [Battery holder for 2] | ensure reliable power supply | [4,99 RON](https://www.optimusdigital.ro/ro/suporturi-de-baterii/941-suport-de-baterii-2-x-18650.html) 
| [L298N Motor Driver Module] |  directions controlling  | [20 RON](https://www.optimusdigital.ro/ro/drivere-de-motoare-cu-perii/145-driver-de-motoare-dual-l298n.html)|
| [Jumper wires]| To ensure the connections between components | [already have]| 



## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [L298N](https://docs.rs/l298n/latest/l298n/struct.L298N.html) | Motor driver implementation | Used for speed and direction control |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Robot car1](https://www.instructables.com/Wifi-Controlled-Robot-Using-Raspberry-Pi/)
2. [Robot car2](https://diyprojectslab.com/raspberry-pi-pico-w-remote-controlled-car/)

# Automated Window Blinds

Fully automated blinds that roll up and down at a set time.

:::info 

**Author**: Naomi Lita in collaboration with Rebeca Chiorean \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-nimintz

:::

## Description

My project aims to automate a window blinds roller. The roller will be controlled through a web page on which I can set a time (like an alarm) to roll up/down the blinds or with a button to roll up/down directly. Also, my project will be connected to Rebeca's project. Her pico wand will be able to control the up/down movement of my window blinds with a hand gesture. 

## Motivation

I chose this project because it was a dream of mine to wake up with natural sunlight at sunrise, but also to sleep well in a dark room with the rollers closed.

## Architecture 

![architecture](schematic_project.png)


## Log

<!-- write every week your progress here -->

### Week 6 - 12 May
I purchased the components for the project. I tried to see if the motor would rotate using thony and micropython, and happily, it worked.

### Week 7 - 19 May
I set up the project in rust. I'm still fixing errors.
On the other hand, I started designing the toothed disks for the 3D printer.

### Week 20 - 26 May

### Hardware

The pico gets input either from a web app or another pico connected through wifi(Rebeca's project: pico wand) and sends a signal to the motor driver (ULN2003) which in turn starts the stepper motor (28BYJ-48) that rolls the blinds up or down.

## Components
 - **28BYJ-48 stepper motor**: this motor is a unipolar stepper motor with 4 phases. It has the torque of about 0.34 kg per cm on its axis. This is enough to raise a lightweight model blind. 

I will use this table for the clockwise direction to create a matrix where high is 1 and low is 0: 
![motor_rotation](motor_rotation.jpg)

For counterclockwise rotation, it is the same matrix, only reversed.

 - **ULN2003 motor driver**: this module connects the motor to the pico. This allows the 4 phases of the motor to be controlled through GP02, GP03, GP04, GP05 on the pico which are set to high or low.

### Schematics

![kicad_schematic](kicad_schematic.jpg)

## Photos
![picture01](project_image01.jpg)
![picture02](project_image02.jpg)

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico WH](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | the microcontroller | [56,23 RON](https://ardushop.ro/ro/home/2819-raspberry-pi-pico-wh.html?search_query=Raspberry+Pi+Pico+WH%2C+Wireless+Headers&results=1031) |
| Kit Breadboard | to place the components | [24,61 RON](https://ardushop.ro/ro/electronica/163-kit-breadboard830-65xfire-jumper-sursa-alimentare-335v.html?search_query=KIT+Breadboard830+++65xfire+jumper+++sursa+alimentare+3%2C3%2F5V&results=694) |
| Resistors | regulate the power supply | [12,29 RON](https://ardushop.ro/ro/electronica/212-set-rezistente-14w-600buc30-valori-10r-1m.html?search_query=SET+rezistori+&results=429) |
| [Stepper Motor 28BYJ-48 5V](http://descargas.cetronic.es/28BYJ-48.pdf) and [Driver ULN2003](https://www.mouser.com/datasheet/2/115/ULN200xA-3216185.pdf)| the motor to rotate the blinds and the driver to connect it to the microcontroller | [16,97 RON](https://www.optimusdigital.ro/ro/motoare-motoare-pas-cu-pas/101-driver-uln2003-motor-pas-cu-pas-de-5-v-.html) |
| 3D printed toothed disks | connect the motor to the blinds |
| 3D printed case for device | hold all components together |


## Software

I only included those specific for this project.

| Library | Description | Usage |
|---------|-------------|-------|
| [uln2003](https://github.com/MnlPhlp/uln2003) | 28BYJ-48 stepper motor with ULN2003 driver | Used for controlling the motor with rust |
| [neon](https://github.com/neon-bindings/neon) | Rust binding for Node.js | Used to connect the Rust code with the Node.js for the webpage |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [3D printed and fully automated Roller Blind Motor](https://imgur.com/a/xuQjH3z)
2. [Pico Project: Automated window blinds that open at sunrise and close at sunset](https://www.reddit.com/r/raspberrypipico/comments/wbdsz1/pico_project_automated_window_blinds_that_open_at/)
3. [28BYJ-48 Stepper Motor with Raspberry PI Pico](https://www.youtube.com/watch?v=VM3S9CiyPzY&t=2s)
4. [28BYJ-48 Stepper Motor Description](https://www.hwlibre.com/en/28bj-48/)
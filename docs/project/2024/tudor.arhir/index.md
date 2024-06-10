# Breathalyzer
A breathalyzer which detects the alcohol in the blown air.

:::info 

**Author**: Tudor-Cristian Arhir \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-TudorArhir

:::

## Description

 My project is a breathalyzer with a LCD (which shows the precentage of alchohol in the air(and some other stuff)), a buzzer, an alcohol sensor, 2 LED's  and a button which resets the number so we can have another reading.( the "no idea" price tag on the components is there because they come from an arduino kit which i bought 1 year ago and i cannot remember the price). It has a 3D-printed case which makes it look more practical than just wires and components on a breadboard . I want this project to be something useful and at the same time something fun which anybody can use.


## Motivation

I chose this project based on two important factors:
- on my 20th birthday my friends got me a real breathalyzer which is very usefull for me because i love to drive
- I wanted to make it more practical and easy to use for anybody at any time(we use it at gatherings and parties)

I want to put in use everything that i learned until now and to learn new things that will make this passion of mine to grown on a larger scale.

## Architecture 

Add here the schematics with the architecture of your project. Make sure to include:
 - what are the main components (architecture components, not hardware components)
 - how they connect with each other
![Block diagram](image-1.png)
## Log

<!-- write every week your progress here -->

### Week 6 - 12 May
In this week i've constructed the project itself on the breadboard. At first , it was kinda difficult to figure out how all those componets should be put on the breadboard but eventually with some research i found out how to do it. I also had to buy another LCD because mine did not have the I2C interface and i couldn't use it without that interface.
### Week 7 - 19 May
In this week I started to work on the code. I had some misunderstandings on how to do this part, but fortunately I managed to make it happen. The only problem was that the project did not work I didn't know how to fix it.
### Week 20 - 26 May
In the last week, I found the problem to my project. The MQ-3 sensor had to be connected to 5V and I found out about this when searching for a formula on how to calculate the alcohol concentration(I still didn't manage to find any mathematical formula for this so I remained with the raw values that the senzor reads.). I also had a small problem with my buzzer, but I quickly managed to fix it.Regarding the 3D printed case, I didn't manage to do it because the friend of mine that had the 3D printer told me that too much material would have been wasted. In the end, I made a case from cardboard which I think it's a pretty good alternative to what I had in mind.
The project is finished and ready to be presented.
I had fun building this project. It made me get out of my comfort zone and to finally see how hard it really is to make something like this. In the future, with the knowledge I've aquired throught this semester, I will surely build something even bigger than this.

## Hardware

The idea is simple : someone who drinks alcohol(a beer for example) blows into the MQ-3 sensor and then 3 thing will happen :
 1. The LCD will show some data which shows the precentage of alcohol in the air that was blown into the sensor
 2. The buzzer will make a sound to signal that the upper limit of detected alcohol in the air has been passed
 3. The LED's will be one red and one blue, which resembles the lights on a police car ( this is just for looks )
### Schematics

Place your KiCAD schematics here.


![Kicad Schematic](schematic.png)

![Photo of the project](case.jpeg)

![Led's](inside.jpeg)

![MQ-3 sensor](sensor.jpeg)

![alcohol detected](alc_detected.jpeg)


### Bill of Materials

| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| LCD1602 Module | The display | from an arduino kit |
| MQ-3 | The alcohol sensor| [15 lei](https://www.optimusdigital.ro/en/gas-sensors/1125-modul-senzor-de-gaz-mq-3.html) |
| Buzzer | Emits sound if you pass the alcohol limit | from an arduino kid |
| Jumper cables | To cable things | from an arduino kit |
| 2 LED's | Used for signaling when the limit was passed | from an arduino kit |
| Cardboard case | To make it more practical | found it in my house |


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [embassy-rp](https://github.com/embassy-rs/embassy/tree/main/embassy-rp) | RP2040 peripherals | Used for accessing the peripherals of the Pico W |
| [LCD-1602_driver](https://crates.io/crates/lcd1602-driver) | A LCD1602 driver | To use the display |
| [embassy-executor](https://crates.io/crates/embassy-executor) | async/await executor designed for embedded usage | Used for spawning asynschronous functions like `main` and `read_adc_value` |
| [embassy-time](https://crates.io/crates/embassy-time) | Instant and Duration for embedded no-std systems, with async timer support | Stops code execution for a predefined time period |
| [rp2040-hal](https://crates.io/crates/rp2040-hal) | A Rust Embedded-HAL impl for the rp2040 microcontroller| Used as a dependencie for the LCD|
| [heapless](https://crates.io/crates/heapless) | `static` friendly data structures that don't require dynamic memory allocation| Used to convert values to `strings` |


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [How does a breathalyzer work](https://www.medicalnewstoday.com/articles/breathalyzer-test)
2. [Breathalyzer with pico w](https://forums.raspberrypi.com/viewtopic.php?t=226459)
3. [How does an MQ-3 work](https://help.sinric.pro/pages/tutorials/custom-device-types/alcohol-sensor/MQ-3)

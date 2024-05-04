# Fire alarm system

:::info

*Author*: Buta Iulia-Mirela
*GitHub Project Link*: https://github.com/UPB-FILS-MA/project-ButaIulia

:::

## Description:

A fire alarm system trigerred by a fire source or other sort of light source in the proximity. It includes the following features:
- Detection of the fire using a Flame Sensor.
- Activating a buzzer alarm when fire is detected.
- Displaying a warning message on a LCD display.

## Motivation
For my first embedded project I wanted to do something usefull for the everyday life and also to practice all the skills that I gained at the laboratory and at the course, and so I choose to  develop a reliable and cost-effective fire alarm system using Raspberry Pi Pico that can detect and alert users in case of fire emergencies.

## Arhitecture
The architecture of the project is the following:
1	Raspberry Pi Pico W: Microcontroller unit responsible for processing and controlling the system.
2. ST7789 Display: Display module for visual notifications and system status.
3. Fire Sensor: Detects the presence of fire or smoke.
4. Buzzer: Audible alarm for alerting users.
5. Power Supply: Provides power to the Raspberry Pi Pico and peripherals.

## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware:

1. Raspberry Pi Pico Board – 1
2. 16×2 I2C LCD Display – 1
3. Flame Sensor – 1
4. Breadboard – 1
5. RGB LED Module – 1
6. Jumper Wires – 10
7. Micro-USB Cable – 1
9. Buzzer - 1

## Connection Overview:
* The fire sensor is connected to one of the GPIO pins of the Raspberry Pi Pico for detecting fire or smoke.
* The buzzer is connected to another GPIO pin to sound the alarm when triggered.
* The ST7789 display is connected to the Raspberry Pi Pico via SPI for displaying system status and alerts.
* Power supply is connected to Raspberry Pi Pico for powering the entire system.

### Schematics

+-------------------------+
 |      Fire Sensor        |
 +-------------------------+

                 |
                 | (Signal)
                 V
         +-----------------+
         | Raspberry Pi Pico|
         +-----------------+
                 |
      +----------+----------+
      |                     |
      V                     V
 +---------+           +-----------+
 |  Buzzer |           |  ST7789   |
 +---------+           +-----------+

### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is

| [Device](link://to/device) | This is used ... | [price](link://to/store) |



-->

| Device                                                                                                  | Usage               | Price                                                                                                                                                                                                                                                                                |
| ------------------------------------------------------------------------------------------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Rapspberry Pi Pico](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [39 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html)                                                                                                                                                                                        |
| [Breadboard]()                                                                                          | Main board          | [0 RON(Already had this)]()                                                                                                                                                                                                                                                          |
| [ 16×2 I2C LCD Display ]()                                                                         | Display             | [14,99 RON](https://www.optimusdigital.ro/ro/optoelectronice-lcd-uri/62-lcd-1602-cu-interfata-i2c-si-backlight-galben-verde.html)                                                                                                                                                           |
| [IR Flame Sensor]()                                                                | Flame Sensor     | [2,49 RON](https://www.optimusdigital.ro/en/optical-sensors/110-ir-flame-sensor.html)                                                                                                                         |
| [Buzzer]()                                                                                              | Buzzer              | [0 RON (Already had this)]()                                                                                                                                                                                                                                                         |
| [4 Mother-Father wires 30 cm]()                                                                         | Connectors          | [0 RON(Already had this)]()                                                                                                                                                                                                                                                          |
| [6 Father-Father wires]()                                                                               | Connectors          | [0 RON(Already had this)]()                           |     
| [ RGB LED Module]()                                                                                          |        | [0 RON(Already had this)]()                                                                                                                                                                                                                               

## Software

| Library                                  | Description                        | Usage                              |
| ---------------------------------------- | ---------------------------------- | ---------------------------------- |
| [st7789] | Display driver for st7789 | Used for the display for Pico Explorer |
| [Rust](https://www.rust-lang.org/)       | Programming Language               | Programming Language               |
| [Embassy](https://embassy.dev/)          | Framework                          | Framework                          |
| [embedded-graphics]         | 2D graphics library                         | Used for drawing to the display           


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. https://how2electronics.com/fire-alarm-system-using-flame-sensor-raspberry-pi-pico/
2. https://www.youtube.com/watch?v=y1_scz9I0go
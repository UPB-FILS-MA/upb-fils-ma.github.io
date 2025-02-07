
# VitalTrack: Wearable Fitness and Health Monitor
A wearable device that tracks steps, heart rate, and location, with data visualization and smartphone integration.


:::info
**Author**: TAKDEER S M \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-takdeerSM
:::


## Description
The Fitness Tracker is designed to monitor physical activities, specifically tracking steps, heart rate, and geographical location. It is aimed at individuals seeking to enhance their physical health through detailed activity records and insights.

## Motivation
I chose this project to delve into the world of wearable technology and sensor integration, driven by an interest in health and fitness. The challenge of creating a device that assists in health management and fitness tracking using real-time data was highly appealing.

## Architecture
The system architecture consists of several key components:

Microcontroller: Serves as the central processing unit, managing sensor data and wireless communication.

Sensors: Includes an accelerometer for step tracking, a heart rate sensor for monitoring pulse, and a GPS module for location tracking.

Display/Indicators: A small display or LED indicators provide visual feedback and data display directly on the device.

Communication Module: Handles data transmission between the fitness tracker and a smartphone.

These components are interconnected, with the microcontroller at the core processing inputs from the sensors and sending information to the display and through the communication module.

## Log

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May



## Hardware

### Microcontroller: Raspberry Pi Pico W
Functionality: Acts as the central processing unit of the fitness tracker. It handles data collection from sensors, processes this data, and manages wireless communication.
Reason for Selection: The Raspberry Pi Pico W is selected for its compact size, low power consumption, integrated Wi-Fi for data transmission, and adequate processing power to handle multiple sensor data streams.

### Accelerometer
Functionality: Measures acceleration forces that can be used to calculate steps and movement. It detects changes in motion and orientation.
Reason for Selection: Essential for a fitness tracker as it enables the device to track steps accurately, which is a fundamental feature for activity monitoring.

### Heart Rate Sensor
Functionality: Monitors the heart rate of the user by detecting the pulse via optical or electrical methods.
Reason for Selection: Heart rate monitoring is crucial for tracking fitness and health metrics, particularly in exercises involving cardiovascular activity.

### GPS Module
Functionality: Provides precise location tracking capabilities, which allows the fitness tracker to record routes and distances traveled.
Reason for Selection: GPS functionality enhances the tracker by enabling outdoor sports enthusiasts to map their runs, hikes, or bike rides.

### Small Display or LED Indicators
Functionality: Displays real-time data such as step count, heart rate, and GPS maps. LED indicators can provide simple feedback like heart rate zones or connectivity status.
Reason for Selection: A display enriches the user interface by providing a visual readout of data, while LED indicators offer minimalistic, power-saving feedback.

## Schematics
Place your KiCAD schematics here.

## Bill of Materials

| Device                | Usage                     | Price                                               |
|-----------------------|---------------------------|-----------------------------------------------------|
| [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html)  | The microcontroller       | [35 RON](https://www.optimusdigital.ro/en/raspberry-pi-boards/12394-raspberry-pi-pico-w.html) |
| [Accelerometer](https://www.omega.com/en-us/resources/accelerometers)         | Step tracking             | [13 RON](https://www.optimusdigital.ro/ro/senzori-senzori-inertiali/97-modul-accelerometru-cu-3-axe-adxl345.html?gad_source=1&gclid=Cj0KCQjwudexBhDKARIsAI-GWYWJSj7gQn3KVIODZsQ75yYTZjC3MPmSpByy592yeU-X4ncr1yfCf5gaApJ9EALw_wcB) |
| Heart rate sensor     | Heart rate monitoring     | [5 RON](https://www.optimusdigital.ro/en/others/12594-heart-rate-sensor-module.html) |
| GPS module            | Location tracking         | [45 RON](https://www.optimusdigital.ro/en/gps/105-gy-neo6mv2-gps-module.html?gad_source=1&gclid=Cj0KCQjwudexBhDKARIsAI-GWYVr6IyMZX1Ynbjm0SX9Md8QoaR7rUFRcTRJWrdmEJVxdJ-6XjGoY2gaAk_XEALw_wcB) |
| Small display/LEDs    | Data display              | [10-20 RON](https://www.optimusdigital.ro/en/73-led-displays) |



## Software

| Library                   | Description                                | Usage                                             |
|---------------------------|--------------------------------------------|---------------------------------------------------|
| [Pico SDK](https://www.raspberrypi.com/documentation/pico-sdk/)                 | Official SDK for Raspberry Pi Pico.        | Fundamental for programming and hardware interaction. |
| [Sensor Libraries](https://www.arduino.cc/reference/en/libraries/category/sensors/)        | Libraries for interfacing with sensors.    | Handle data acquisition from accelerometer, heart rate sensor, and GPS. |
| [Display Driver Libraries](https://www.arduinolibraries.info/categories/display)  | Drivers for managing output to displays.   | Manage visual output on the device's display.     |
| [Communication Libraries](https://github.com/hathach/tinyusb)  | Libraries to enable Wi-Fi connectivity.    | Facilitate data transmission to smartphones or the cloud. |


## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Workout tracker](https://github.com/topics/workout-tracker)
2. [Initiative Tracker: First Full Practice Project](https://users.rust-lang.org/t/initiative-tracker-first-full-practice-project/97159)

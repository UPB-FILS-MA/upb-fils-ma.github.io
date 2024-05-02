# Project Name

PicoW Garden Monitor

**Author**: JERCĂu Hadasa-Ștefana \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-hadassahj

:::

## Description

PicoW Garden Monitor is a project designed to monitor and optimize the growth conditions of plants in gardens using the Raspberry Pi Pico microcontroller. It collects data on soil moisture, light exposure, and temperature to assess the health of plants and provide personalized care instructions for optimal growth. The project aims to include a web application too, hosted on a local server built with Rust, which would allow users to interact with the system, select plants, view real-time sensor data, and receive customized care instructions. By envisioning a locally hosted web interface as part of its development, PicoW Garden Monitor aims to offer a comprehensive solution for plant enthusiasts to cultivate healthy and thriving gardens.

## Motivation

The motivation behind PicoW Garden Monitor stems from a personal need to address common challenges faced in plant care. As someone who often forgets to water plants and struggles to find the optimal spot with the right amount of sunlight, there is a strong desire to find a solution that simplifies and enhances the gardening experience.

By developing the PicoW Garden Monitor, the goal is to create a reliable and intuitive tool that helps overcome these challenges. With real-time monitoring of soil moisture, light exposure, and temperature, the system provides timely reminders for watering and insights into the ideal environmental conditions for plant growth. This empowers users to nurture healthy and thriving gardens, even with busy schedules or limited gardening experience.

## Architecture 

Main Components:
    Sensors: Soil moisture sensor, light sensor, temperature sensor.
        - These sensors are strategically placed in the garden to capture essential environmental data.
        - The soil moisture sensor measures the moisture level in the soil, indicating when plants need watering.
        - The light sensor detects the intensity of sunlight, helping users identify optimal spots for different types of plants.
        - The temperature sensor monitors the ambient temperature, ensuring plants are not exposed to extreme conditions.
    Raspberry Pi Pico:
        - The Raspberry Pi Pico serves as the central hub of the system, responsible for interfacing with the sensors and facilitating communication with the Rust server.
        - It collects data from the sensors and sends this information to the Rust server for analysis.
        - Additionally, the Raspberry Pi Pico executes control logic based on the analyzed data, such as activating watering systems or adjusting light exposure.
    Rust Server:
        - The Rust server receives sensor data from the Raspberry Pi Pico and processes it to assess the health parameters of the plants.
        - Using algorithms and predefined thresholds, the server analyzes the data to determine if any corrective actions are needed.
        - It generates personalized care instructions based on the analysis results, considering factors such as soil moisture, light exposure, and temperature requirements.
    Web Interface:
        - The web interface provides users with a user-friendly dashboard to interact with the system.
        - Users can access the dashboard from their devices connected to the local network, such as smartphones, tablets, or computers.
        - Through the web interface, users can select specific plants, view live sensor data, and receive actionable insights and care recommendations.
        - The interface communicates with the Rust server to retrieve and display relevant information, ensuring a seamless user experience.
Connection:
    - The sensors are connected directly to the Raspberry Pi Pico microcontroller, which interfaces with them to collect data.
    - The Raspberry Pi Pico communicates with the Rust server over the local network, sending sensor data and receiving instructions.
    - The Rust server processes the incoming data, generates care instructions, and communicates with the web interface to provide users with real-time updates and insights.

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


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [link](https://example.com)
2. [link](https://example3.com)
...

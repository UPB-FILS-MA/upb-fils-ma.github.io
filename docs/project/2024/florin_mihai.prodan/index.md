# Crasher game
Time your bet before the airplane crashes.

:::info 

**Author**: PRODAN Florin Mihai Alexandru \
**GitHub Project Link**: https://github.com/UPB-FILS-MA/project-mihaiprodann

:::

## Description

"Crasher" is an engaging online game featuring an airplane that ascends on the screen while a multiplier increases. Players are challenged to decide the optimal moment to cash out their bet before the airplane crashes, abruptly stopping the multiplier.

## Motivation

This project was inspired by my past experiences with gambling, particularly one that combined elements of risk and strategy in a simple yet engaging format. As part of a project requirement, I decided to bring this game into a tangible form using the Raspberry Pi Pico W and other electronic components. The goal was to recreate the game's excitement and decision-making process in a new, interactive way, merging it with practical experience gained during the lab.


## Architecture 

In it's final stage, my project will look more or less like the following diagram:
![Aviator Game Control Panel](./PROTOTIP.png)

* The airplane shape will be made of LEDs of the indicated colors.
* The central LCD screen will display information about game status, current money amount, etc.
* By pressing the button, you will initiate a new game by betting the selected amount of money.
* With the potentiometer, you will select the amount of money you want to bet.
## Log

<!-- write every week your progress here -->

### Week 6 - 12 May

### Week 7 - 19 May

### Week 20 - 26 May

## Hardware

1. **Raspberry Pi Pico W**: Microcontroller board for running the game logic.
2. **LEDs**: Visual indicators representing multiplier values.
3. **LCD 1602 module**: Display for showing current odds and game information.
4. **Push-button switch**: Trigger for starting the game.
5. **Potentiometer**: Control for adjusting the bet amount.
6. **Breadboard**: Platform for prototyping and connecting components.
7. **Jumper wires**: Connectors for establishing electrical connections on the breadboard.

In the project, the Raspberry Pi Pico W runs the game logic, while LEDs visualize multiplier values, the LCD 1602 module displays game information, the push-button switch triggers the game, and the potentiometer adjusts the bet amount. These components are connected using a breadboard and jumper wires for prototyping and experimentation.

### Schematics

![KICad Schematics](./KicadScheme.png)

In the center of the schematics there is the Raspberry Pi Pico W. In it's left side, there are 3 shift registers (74HC595) which I use for the 24 leds that will form my airplane shape. In the right side of the Pico W, there is the LCD connection (actually the PCF8574 I2C module that I use to connect the 1602LCD with my Raspberry Pi Pico W), the 2 push buttons and the potentiometer.
### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->


| Device                           | Usage               | Price |
|----------------------------------|---------------------|-------|
| Raspberry Pi Pico W              | Microcontroller Core | [35 Lei](https://www.optimusdigital.ro/ro/placi-raspberry-pi/12394-raspberry-pi-pico-w.html) |
| LCD 1602 Display (I2C Interface) | Display Output       | [25 Lei](https://www.bitmi.ro/ecran-lcd1602-cu-modul-i2c-iic-10487.html?gad_source=1) |
| Breadboard 830 Tie Points MB-102 | Prototype Platform  | [10 Lei](https://www.bitmi.ro/breadboard-830-puncte-mb-102-10500.html?gad_source=1) |
| Set of 200 LEDs (3mm & 5mm)      | Visual Indicators    | [16 Lei](https://www.bitmi.ro/componente-electronice/set-200-led-uri-de-diferite-culori-3-mm-5-mm-10508.html) |
| Tactile Push Button              | User Input           | [6.5 Lei](https://ardushop.ro/ro/home/97-buton-mic-push-button-trough-hole.html?gad_source=1) (for 10) |
| 10K ohm Potentiometer            | Adjustable Input     | [12.85 Lei](https://ardushop.ro/ro/electronica/193-potentiometru-10k.html?gad_source=1) (for 5) |
| Knobs for Potentiometers         | Knob for Adjustment  | [10.65 Lei](https://ardushop.ro/ro/electronica/321-buton-pentru-poteniometru.html?gad_source=1) (for 5) |
| Dupont Wires M-F, 20cm           | Wire Connection      | [6 Lei](https://www.bitmi.ro/electronica/40-x-fire-dupont-tata-mama-20cm-10512.html) |
| Dupont Wires M-F, 30cm           | Wire Connection      | [7 Lei](https://www.bitmi.ro/electronica/40-fire-dupont-tata-mama-30cm-10504.html) |
| Dupont Wires F-F, 30cm           | Wire Connection      | [6 Lei](https://www.bitmi.ro/electronica/40-fire-dupont-mama-mama-30cm-10503.html) |
| Dupont Wires F-F, 20cm           | Wire Connection      | [6 Lei](https://www.bitmi.ro/electronica/40-x-fire-dupont-mama-mama-20cm-10509.html) |
| Dupont Wires M-M, 30cm           | Wire Connection      | [7 Lei](https://www.bitmi.ro/electronica/40-fire-dupont-tata-tata-30cm-10505.html) |
| Dupont Wires M-M, 20cm           | Wire Connection      | [7 Lei](https://www.bitmi.ro/electronica/40-x-fire-dupont-tata-tata-20cm-10511.html) |
| 220Ω Resistors                   | Current Limiting     | [9 Lei](https://ardushop.ro/ro/electronica/211-rezistenta-14w-1-buc.html#/83-valoare_rezistenta-220r) (for 30) |
| 330Ω Resistors                   | Current Limiting     | [9 Lei](https://ardushop.ro/ro/electronica/211-rezistenta-14w-1-buc.html#/85-valoare_rezistenta-330r) (for 30) |
| Male Headers                     | Connection Points    | [7.5 Lei](https://ardushop.ro/ro/electronica/70-40-x-bareta-pini-tata.html?gad_source=1) (for 10) |

**Total Cost:** 159.5 Lei


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |
| [micro-rand](https://crates.io/crates/micro_rand) | A tiny, no STD library for generating (pseudo) random numbers. | Used to generate when the airplane will crash |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Crasher game demo](https://www.youtube.com/watch?v=bzFpdu2o-o0])
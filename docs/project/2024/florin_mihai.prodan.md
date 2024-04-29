# Crasher casino game

:::info 

**Author**: PRODAN Florin Mihai Alexandru
**GitHub Project Link**: [link_to_github](https://github.com/UPB-FILS-MA/project-mihaiprodann)

:::

## Description

"Crasher" is an engaging online casino game featuring an airplane that ascends on the screen while a multiplier increases. Players are challenged to decide the optimal moment to cash out their bet before the airplane crashes, abruptly stopping the multiplier.

## Motivation

This project was inspired by my past experiences with casino games, particularly one that combined elements of risk and strategy in a simple yet engaging format. As part of a project requirement, I decided to bring this game into a tangible form using the Raspberry Pi Pico W and other electronic components. The goal was to recreate the game's excitement and decision-making process in a new, interactive way, merging it with practical experience gained during the lab.


## Architecture 

[todo]

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
| 74HC595 Shift Registers          | Expand GPIOs         | [20 Lei](https://ardushop.ro/ro/electronica/141-ic-shift-register-sn74hc595n-74hc595.html) (for 5) |
| Male Headers                     | Connection Points    | [7.5 Lei](https://ardushop.ro/ro/electronica/70-40-x-bareta-pini-tata.html?gad_source=1) (for 10) |

**Total Cost:** [179.5 Lei](#)


## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [st7789](https://github.com/almindor/st7789) | Display driver for ST7789 | Used for the display for the Pico Explorer Base |
| [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics) | 2D graphics library | Used for drawing to the display |

## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Crasher game demo](https://www.youtube.com/watch?v=bzFpdu2o-o0])
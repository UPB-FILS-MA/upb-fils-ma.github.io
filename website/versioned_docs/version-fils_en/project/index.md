---
sidebar_position: 1
---

# Description

The purpose of the project is to build a hardware device that runs software written in Rust.

## Deliverables
All the deliverables will be published in a GitHub repository that each student will be assigned to.

The repository contains:
- documentation - the reposiotry's `README.md` file
  - short description of functionality
  - requirements (hardware and software)
  - hardware design - use [KiCad EDA](https://www.kicad.org/) or a similar tool for the schematics
  - software design
    - detail design
    - functional diagram
- software source code

Students will have to build and showcase the hardware with the running software at PM Fair.

## Hardware Rules

1. Projects have to use a microcontroller (MCU) that is capable of running Rust code. Examples of MCUs are *nRF52*, *RP2040*, *ESP32* (RISC-V version). 
2. Usage of a development board is encouraged, but not required, a custom PCB can be built. Example of development boards are:
  - [Raspberry Pi Pico](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) (RP2040) or [Raspberry Pi Pico W](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) for WiFi
  - [Adafruit Trinkey QT2040](https://www.adafruit.com/product/5056) (RP2040)
  - [Arduino Nano RP2040 Connect](https://store.arduino.cc/products/arduino-nano-rp2040-connect) (RP2040) - ⚠️ [^arduino_nano_rp2040_connect]
  - [micro:bit v2](https://microbit.org/) (nRF52833)
  - [nRF52 DK](https://www.nordicsemi.com/Products/Development-hardware/nrf52-dk) (nRF52810)
  - [STM32 NUCLEO-F401RE](https://ro.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-F401RE?qs=sGAEpiMZZMuqBwn8WqcFUv%2FX0DKhApUpi46qP7WpjrffIid8Wo1rTg%3D%3D)
  - [ESP32-C3-DevKit-RUST-1](https://www.espressif.com/en/dev-board/esp32-c3-devkit-rust-1-en) (ESP32-C3) - ⚠️ [^esp32_riscv]
3. The hardware part may be designed either using a breadboard and jumper wires, a prototype board (solder breadboard) or a PCB.

## Software Rules
It has to run software written in Rust. Students can use:
- [embassy-rs](https://embassy.dev/)
- [RTIC](https://rtic.rs/2/book/en/)
- [bare metal](https://docs.rs/cortex-m-rt/latest/cortex_m_rt) - the cortex-r-rt crate
- [Tock](https://www.tockos.org)
- any other software framework that is written in Rust


## Project Rules

1. Copying schematics or source code from the Internet is not allowed.
2. Each project is considered individual work, students can ask the lab assistant (lab) about implementation details during the development period.
3. Students are strongly encouraged to ask the lab assistant questions about the project.
4. The presentation of all the milestones is mandatory.

## Requirements
1. *Complexity:* use knowledge of at least 3 labs, **excluding** [00 - Rust](https://embedded-rust-101.wyliodrin.com/docs/lab/00), [01 - Hardware Introduction](https://embedded-rust-101.wyliodrin.com/docs/lab/01) and [02 - Setting up the Raspberry Pi Pico + GPIO](https://embedded-rust-101.wyliodrin.com/docs/lab/02)
2. *Documentation:* Complete documentation of the implementation for both hardware and software.
3. *Functionality:* the hardware device has to be fully functional.

## Example Projects

### Examples of projects from past years
1. https://ocw.cs.pub.ro/courses/pm/prj2022
2. https://ocw.cs.pub.ro/courses/pm/prj2023

### Outstanding Projects
1. [POV - DAVIC picTronics](https://ocw.cs.pub.ro/courses/pm/prj2023/gpatru/376)
2. [Ryobo - Computer Vision & Object Following](https://ocw.cs.pub.ro/courses/pm/prj2023/gpatru/483)
3. [Plug & Play ChatGPT](https://ocw.cs.pub.ro/courses/pm/prj2023/ncaroi/plug)
4. [VENDING MACHINE](https://ocw.cs.pub.ro/courses/pm/prj2023/drtranca/vending.machine)

## Grading

| Part | Deadline | Points |
|--------|--------|--------|
| Documentation Milestone | Lab 9 | 1p |
| Hardware Milestone | Lab 11 | 1p |
| Software Milestone | Lab 12 | 1p |
| PM Faire | TBD | 2p |
| **Total** |  | **5p** |

## F.A.Q
**Q:** Can I use another programming language, not Rust?\
**A:** No, the main focus of the project is to learn to work with microcontrollers using Rust.

**Q:** Can I use a different framework than [embassy-rs](https://github.com/embassy-rs/embassy)?\
**A:** Yes, we suggest taking a look at [RTIC](https://rtic.rs/2/book/en/) or [Tock](https://github.com/tock/tock).

**Q:** What if the PCB arrives after the hardware milestone?\
**A:** You will only present the diagram for the hardware part and if there is a prototype using breadboard, but at the end of the project you must necessarily have the PCB printed and functional.

[^arduino_nano_rp2040_connect]: Some function of this board, like WiFi, might not be supported in Rust.
[^esp32_riscv]: ESP32 provides its own Rust SDK described in the [The Rust on ESP Book](https://docs.esp-rs.org/book/introduction.html).


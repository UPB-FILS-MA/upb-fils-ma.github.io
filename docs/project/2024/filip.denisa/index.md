# Project Name




:::info 

**Author**:
**GitHub Project Link**: 

:::

## Description




  
    


## Motivation



## Architecture 


![Diagram of the connections]()





  

## Log

<!-- write every week your progress here -->

### Week 6 - 12 May




### Week 7 - 19 May




### Week 7 - 19 May


### Week 20 - 26 May

## Hardware


  

  

  

### Schematics


Schematic of the project on KiCad Application.

![Schematic of the project with using KiCad]()




### Bill of Materials

<!-- Fill out this table with all the hardware components that you might need.

The format is 
```
| [Device](link://to/device) | This is used ... | [price](link://to/store) |

```

-->

| Device | Usage | Price |
|--------|--------|-------|
| [Rapspberry Pi Pico H](https://www.raspberrypi.com/documentation/microcontrollers/raspberry-pi-pico.html) | The microcontroller | [42 RON](https://www.optimusdigital.ro/ro/placi-raspberry-pi/12393-raspberry-pi-pico-h.html?search_query=pico+H&results=32) |




## Software

| Library | Description | Usage |
|---------|-------------|-------|
| [ag-lcd](https://github.com/mjhouse/ag-lcd) | Display Library | Used for I2C LCD Display |
| [embassy-rp](https://github.com/embassy-rs/embassy/tree/main/embassy-rp) | RP2040 Peripherals | Used for accessing the peripherals|
| [embassy-time](https://github.com/embassy-rs/embassy/tree/main/embassy-time) | Time Library | Used for Timeouts and Delays |
| [PWM](https://docs.embassy.dev/embassy-nrf/git/nrf52840/pwm/index.html) | Pulse-Width Modulation | Used for Buzzer |
| [embassy-executor](https://docs.embassy.dev/embassy-executor/git/std/index.html) | Executor for Rust Embedded Systems | Used for task scheduling and asynchronous programming |
| [embassy-usb-logger](https://docs.embassy.dev/embassy-usb-logger/git/default/index.html) | USB Logger for embassy | Used for logging messages over USB |
| [GPIO](https://docs.embassy.dev/embassy-stm32/git/stm32c011d6/gpio/index.html) | GPIO  | Used for interacting with GPIO Pins |
| [embassy-usb-logger](https://docs.embassy.dev/embassy-usb-logger/git/default/index.html) | USB Logger for embassy | Used for logging messages over USB |




## Links

<!-- Add a few links that inspired you and that you think you will use for your project -->

1. [Pinouts for RP](https://pinout.xyz/pinout/1_wire)
2. [Crates](https://crates.io)
3. [Rust for Embedded Systems](https://docs.rs)
4. [RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)
5. [Embassy Documents](https://embassy.dev/book/dev/index.html)
6. [Rust For Embedded Wyliodrin](https://embedded-rust-101.wyliodrin.com)

...

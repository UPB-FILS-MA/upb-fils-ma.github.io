---
sidebar_position: 2
slug: /tutorials/embassy
description: How to install the prerequisites for embassy-rs
---

# Embassy-rs Setup

Here, we will cover the steps needed in order to be able to compile and flash Rust applications for **RP2040**, the MCU (Microcontroller Unit) found in our **Raspberry Pi Pico W**s.

## Prerequisites

### Rust Toolchain

In order to install the tools needed to compile Rust code, follow the next steps, depending on your operating system.

#### Linux

Run the this command in terminal:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This downloads and runs `rustup-init.sh`, which in turn downloads and runs the correct version of the `rustup-init` executable for your platform.

:::info 

Before installing elf2uf2-rs, you need to install  `pkg-config` and `libudev`. You can get it by running the following in your terminal. 
```shell
sudo apt-get install pkg-config libudev-dev
```

:::

#### Windows

Download the respective executable:

* [RUSTUP-INIT.exe - 64bit](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
* [RUSTUP-INIT.exe - 32bit](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)

:::note
You may be prompted to install [Visual Studio C++ Build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). If so, follow the instructions from the previous link.
:::

The last step is to run `rustup --version` in terminal. If everything went well, you should see an output similar to this:

```shell
rustup 1.26.0 (5af9b9484 2023-04-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.76.0 (07dca489a 2024-02-04)`
```

### `elf2uf2-rs`

This tool is needed to be able to program the board over USB. In order to install it, run the following in your terminal:

```shell
cargo install elf2uf2-rs
```

Then, run `elf2uf2-rs --help`. If it was correctly installed, you should see something similar to this in your terminal:

```shell
Usage: elf2uf2-rs [OPTIONS] <INPUT> [OUTPUT]

Arguments:
  <INPUT>   Input file
  [OUTPUT]  Output file

Options:
  -v, --verbose  Verbose
  -d, --deploy   Deploy to any connected pico
  -s, --serial   Connect to serial after deploy
  -h, --help     Print help
```

### `probe-rs`

This tool is an embedded debugging and target interaction toolkit. It enables its user to program and debug microcontrollers via a debug probe.

```shell
cargo install probe-rs --features cli --locked
```

If you are on **Linux** you will also need to add this [`udev`](https://probe.rs/files/69-probe-rs.rules) file in `/etc/udev/rules.d`. Then, run:

```shell
udevadm control --reload # to ensure the new rules are used.

udevadm trigger # to ensure the new rules are applied to already added devices.
```

### VSCode Extension (Optional)

For a better experience, go ahead and install the **Debugger for probe-rs** extension in the Microsoft Extension Marketplace. This will make debugging the program running on the RP2040 as easy as debugging a Rust program running on your host machine.

## Flashing over USB

### Compiling

You will need to compile your executable specifically for the RP2040 chip. This chip is based on the **ARM Cortex M0+** architecture, so we will need to specify our target when compiling. We can do that in multiple ways:

* using a `.cargo/config.toml` file:

```toml
[build]
target = "thumbv6m-none-eabi"
```

* passing it as a parameter to Cargo:

```shell
cargo build --release --target thumbv6m-none-eabi
```

### Flashing

To flash a program to the Raspberry Pi Pico via USB, it needs to be in *USB mass storage device mode*. To put it in this mode, you need to **hold the `BOOTSEL` button down**  while connecting it to your PC. Connecting and disconnecting the USB can lead to the port getting damaged, so we conveniently attached a reset button on the breadboard included on the **Pico Explorer Base**. Now, to make it reflashable again, just press the two buttons simultaneously.

After connecting the board to your PC and compiling the program, locate the binary in the `target/thumbv6m-none-eabi/release/` folder then, run:

```shell
elf2uf2-rs -d -s /path/to/your/binary
```

* `-d` to automatically deploy to a mounted pico
* `-s` to open the pico as a serial device after deploy and print serial output
  
:::note
On `Windows`, you may need to run this command in a terminal that has **Admin Privileges**.
:::

## Debugging using Raspberry Pi Debug Probe

In order to be able to debug the program running on the board, we will need to connect the **Raspberry Pi Debug Probe** to our **Raspberry Pi Pico W**. Below, you have a picture of the debug kit provided:

![Raspberry Pi Debug probe](assets/the-probe.png)

To connect them, we will use the **3-pin debug to 0.1-inch header (female)** cable. First, carefully insert the **3-pin debug** head in the **right side** connector. Then you will also need to connect it to the Raspberry Pi Pico W. You will find attached the pinout, take a closer look at the bottom of the image:

![Raspberry Pi Pico W pinout](assets/picow-pinout.svg)

The connections must be:

| Wire | Raspberry Pi Pico W |
|-|-|
|TX (Orange)|SWCLK|
|GND (Black)|GND|
|RX (Yellow)|SWDIO|

:::warning
Do not forget to connect both the Debug Probe and Pico to your PC.
:::

Now, you can either debug using the command line by running:

```shell
probe-rs run --chip RP2040 path/to/your/binary
```

or you can use **Debug and Run** view in Visual Studio Code. You will need to modify the `programBinary` path in the `.vscode/launch.json` config file to point to your binary file.

## Building your first Embassy-rs project

In this section, we will briefly go over the steps you need to take in order to get your first project using **Rust** and **Embassy-rs** going.

### Creating your crate

The first step is to create your cargo package by running the following command in your terminal:

```shell
cargo new --vcs none embassy
```

* `--vcs none` because at the moment we do not want to use any code versioning (they are useful, but this is not the purpose of this tutorial)

### Crate settings

Because we are running in an embedded environment, our code needs to be *"tailored"* specifically for the microcontroller we intend to use. In our case, it is the **RP2040**, but these general steps apply for any chip, produced by any manufacturer.

#### No standard library

Due to the size constraints imposed on us (in our case, `2MB` of flash memory), the standard library has to go. We specify that by adding the `#![no_std]` attribute to the beginning of our `src/mains.rs` file.

#### No `main` function

Because we are using the **Embassy-rs** framework, we want to let it take care of the entry point of our program (because it has to do some complex operations, like allocating the `task-arena` and `executor` structures). For the moment, all we will need to do is add the `#![no_main]` attribute to `src/main.rs`.

#### Toolchain setting

Our chip is a **Cortex-M0+** that uses the **ThumbV6-M** architecture so we will need to specify the target triple we are compiling for. We will do that using a `rust-toolchain.toml` file, as it allows us to also set the **toolchain release channel** we will use, and the components we require.

An example of such file is this:

##### rust-toolchain.toml

```Toml
# This file is used to specify the version of the Rust toolchain that 
# should be used for your project.

[toolchain]
# The release to be used.
channel = "1.75"
# The targets for compilation that need to be added. This is used for 
# cross-compilation, as the executables we are producing need to be
# run on our boards.
targets = ["thumbv6m-none-eabi"]
# The additional componets to be installed along the Rust toolchain
components = ["rust-src", "rustfmt", "llvm-tools", "clippy"]
```

:::tip

Please make sure that you install the Rust ARMv6-M target (thumbv6m-none-eabi).

```bash
rustup target add thumbv6m-none-eabi
```

:::

#### Memory layout

We also need to take care of the memory layout of our program when writing code for a microcontroller. These can be found in the datasheet of all the microcontrollers. Bellow, you can find the memory layout for the **RP2040**:

##### `memory.x`

```linker-script
/* Memory regions for the linker script */
/* Address map provided by datasheet: https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf */
MEMORY {
    /* Define the memory region for the second stage bootloader */
    BOOT2 : ORIGIN = 0x10000000, LENGTH = 0x100

    /* Define the memory region for the application to be loaded next */
    FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100

    /* Define the memory region for SRAM */
    RAM   : ORIGIN = 0x20000000, LENGTH = 264K
}
```

To use the `memory.x` layout file, we will also need to use a build script. Rust facilitates that through the `build.rs` file. Bellow you will find an explained build script you can use.

##### `build.rs`

```rust
//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("./memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={{layout}}");

    // `--nmagic` is required if memory section addresses are not aligned to 0x10000,
    // for example the FLASH and RAM sections in your `memory.x`.
    println!("cargo:rustc-link-arg=--nmagic");

    // The `link.x` linker script provided by `cortex_m_rt` (minimal runtime for
    // Cortex-M microcontrollers used by embassy) will include our `memory.x` memory layout.
    println!("cargo:rustc-link-arg=-Tlink.x");

    // The `link-rp.x` linker script provided by `embassy_rp` that defines the
    // BOOT2 section.
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");

    // The `defmt.x` linker script provided by `defmt`.
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
```

#### Adding the Dependencies

At this step, we must add the dependencies we will use for our project. Bellow you will find the basics you will need for a minimal application, including an `usb_logger` to *"enable"* debugging over serial.

##### `embassy-executor`

This is an `async/await` executor designed for embedded. To add it as a dependency to your project, run:

```shell
cargo add embassy-executor --features arch-cortex-m,executor-thread,executor-interrupt,integrated-timers,task-arena-size-32768
```

* `arch-cortex-m` - feature to specify we are running on the cortex M architecture
* `executor-thread` - enable the thread-mode executor (using WFE/SEV in Cortex-M, WFI in other embedded archs)
* `executor-interrupt` - enable the interrupt-mode executor (available in Cortex-M only)
* `integrated-timers` - use the executor-integrated embassy-time timer queue.
* `task-arena-size-X` - sets the task arena size

We will also need to add the `cortex-m` and `cortex-m-rt` crates as dependencies, as the `#[executor::main]` attribute depends on the minimal startup code for the Cortex M microcontrollers found in this crates. To do that, run:

```shell
cargo add cortex-m
cargo add cortex-m-rt
```

##### `embassy-time`

This crate enables timekeeping, timeouts and delays. Add it by running:

```shell
cargo add embassy-time
```

#### `embassy-rp`

This crate is a **Hardware Abstraction Layer** for the **RP2040**. You can add it to your project like so:

```shell
cargo add embassy-rp --features time-driver,critical-section-impl
```

* `time-driver` - enable the timer for use with `embassy-time` with a `1MHz` tick rate.
* `critical-section-impl` - configure the critical section crate to use an implementation that is safe for multicore use on RP2040

#### `embassy-usb-logger`

USB implementation of the `log` crate. It allows the usage of `info!` macro and some more. To add it, run the following command:

```shell
cargo add log
cargo add embassy-usb-logger
```

### `probe-panic`

This crate adds a panic handler for the microchip that prints panic messages over **JTAG**, and in order to add it, run:

```shell
cargo add panic-probe
```

### The code

Here you can find a minimally explained code that prints `"Hello World!"` over the serial interface:

#### `main.rs`

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler as UsbInterruptHandler};
use embassy_time::Timer;
use log::info;
use panic_probe as _;

// Bind interrupts to their handlers.
bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => UsbInterruptHandler<USB>;
});

// Async task for USB logging.
#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize peripherals and USB driver.
    let rp_peripherals = embassy_rp::init(Default::default());
    let usb_driver = Driver::new(rp_peripherals.USB, Irqs);

    // Spawn the logger task
    spawner.spawn(logger_task(usb_driver)).unwrap();
    
    Timer::after_millis(1000).await;
    info!("Hello, world!");

    loop {
        Timer::after_millis(10).await;
    }
}
```

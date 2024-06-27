---
layout: section
---
# Tock
use as main the tool embedded systems course teaching

---
---
# Why Tock?

- *Applications* - it runs full applications that fault and print a debug message
  - over USB, a debugger is not needed
  - apps are simpler to write
- *OS Intrenals* - students cand easily understand the OS internals
  - it is easy to write a driver
- *Security* - easy way to introduce security in an embedded systems course
  - App IDs
  - System call filter
- *Development* - students can you several languages to write projects
- *No Dependencies* - there are not dependencies that break

---
layout: section
---
# TODOs
there are things to do to actually use Tock

---
---
# Connectivity
support for WiFi/Ethernet mostly

**Work in progress**
- Arduino Nano RP2040 [#2625](https://github.com/tock/tock/pull/2625)
- Ethernet for STM32 [#3695](https://github.com/tock/tock/pull/3695)
- PacketBuffers (Amalia)

**TODOs**
- Port the [RP2040 WiFi Driver](https://github.com/embassy-rs/embassy/tree/main/cyw43) to Tock
- TCP/IP stack implementation
  - smoltcp in userspace
  - smoltcp in the kernel

Thread is not great is you do need gateways that students do not have at home

---
---
# Fix the USB stack

The USB stack is broken, at least serial port jams frequently

**Issues**
- the issue [#4011](https://github.com/tock/tock/issues/4011)
- refactor the USB stack

**TODOs**
- add the USB IAD for MS Windows
- document how the stack works

---
---
# Configurator

The `main.rs` file is way to complicated, a `menuconfig` like system would be great

<div grid="~ cols-2 gap-5">

<div>

**Work in progress**
- Write a configurator (OxidOS / Irina)
- Tweedegolf is happy to help

**TODOs**
- a lot of feedback is needed

</div>

<img src="/tock/demo.gif" class="w-100">

</div>

---
---
# `async`/`.await` support for libtock-rs
it is easier to write asynchronous apps

**Work in progress**
- add Tock as a backend to `embassy-executor`
- define `async` APIs in `libtock-rs` [#949](https://github.com/tock/libtock-rs/issues/494)

**TODOs**
- might be tricky to add async, due to the way in which `scope` works

---
---
# Support the Rust Embedded HAL
so that users can add libraries to their applications

**Done**
- Embedded HAL [#540](https://github.com/tock/libtock-rs/pull/540)

**TODOs**
- implement the full embedded HAL

---
---
# Userspace drivers
safely expose devices to userspace

**Work in progress**
- Device Passthrough [#4020](https://github.com/tock/tock/issues/4020)
- Stub out device pass through support [#4044](https://github.com/tock/tock/pull/4044)

**TODOs**
- define some special API?

---
---
# Windows support

- using VMs for Tock is difficult due to bad support fom VM providers
  - VMWare Workstation might not be available
  - VirtualBox gets stuck
  - WSL2 has an issue with mapping USB ports

**TODOs**
- add support for building Tock in Windows
- use probe-rs to replace *openocd* and *JLink*
- linker scripts might be problematic

---
---
# Dev board Kit
everyone has different hardware platforms

**Requirements**
- be able to build it with off-the-shelf components
- cost under $50
- debugger!

**Work in progress**
- lab board
  - RP2040 as a debuger
  - Pico W SMD mounted (cheaper than bying the components)
  - buttons, LEDs, screen, buzzer and extension sockets

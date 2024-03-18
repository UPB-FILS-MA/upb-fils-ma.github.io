---
sidebar_position: 3
slug: /lab/03
---

# 03 - Exceptions & Interrupts

The purpose of this lab is to understand how exceptions and interrupts work and how they can be used, how to set a hard fault handler,
register interrupts and *use interrupts*  with embassy-rs.

## Exceptions

ARM Cortex-M0+ processors have 6 exceptions:

| Exception | Priority | Descriptions |
|-----------|----------|--------------|
| Reset | -15 | Triggered by the system at startup to start the software |
| NMI | -14 | Non Maskable Interrupts, an external interrupt that cannot be ignored, usually used for low latency peripheral needs the attention of the MCU |
| HardFault | -13 | Triggered by the MCU in case of a fault (div by 0, memory fault, ...) |
| SVC | -5 | Supervisor call, triggered usually by a process running on top of the embedded operating system when it wants to make a system call |
| PendSV | -2 | Used for pending system calls |
| SysTick | -1 | Triggered by a periodic timer, usually used by an embedded operating system for context switch |

:::note

The lower the priority number, the higher the priority of the exception is. 

:::

Higher priority exceptions can interrupt lower priority exception handlers.

### Register a HardFault handler

There are several reasons why a hard fault is triggered:
- **invalid memory** reads and writes
- *invalid* address of the *reset handler* (not the case for RP2040, as the real reset handler is in the Bootrom)
- *invalid* address of the *initial stack pointer* (not the case for RP2040, as the real reset handler is in the Bootrom)
- using the *svc* instruction *in the HardFault handler*
- using the *svc* instruction *NMI handler*

```rust
#[exception]
unsafe fn HardFault(frame: &FrameException) -> ! {

}
```

#### Triggering a hard fault

The easiest way of forcing a hard fault is to try to read or write to or from a memory location that is not valid.
For the RP2040, an invalid memory address is `0xf000_0000`.

```rust
// define an invalid memory address
const INVALID_ADDRESS: *const u32 = 0xf000_0000 as *const u32;

// write to it
unsafe {
    // this triggers a hard fault
    write_volatile(INVALID_ADDRESS, 0);
}
```

:::

### Register a SysTick Handler.

```rust
#[exception]
unsafe fn SysTick {

}
```

TODO how systick is configured (using pac)

## Interrupts

TODO: list RP2040's interrupts

### Register a pin interruopt

```rust
#[interrupt]
unsafe fn IO_BANK0_INT() {
    
}
```

## Inspect binaries

TODO rust-objdump to see sections and interleaved code

## Exercises

1. Connect an LED1 to pin 0 and an LED2 to pin 1. Use [KiCad](https://www.kicad.org/) to draw the schematics.
2. Write a program using PAC that blinks the LED2. Use `rust-objdump` to display the sections.
::: tip

Use the code from [Lab 02](/lab/02).

:::
3. Register a hard fault handler that blinks LED2.
   1. Generate a hard fault, you should see LED1 stop blinking (either on or off) and the LED2 blink
   2. Move the blinking LED2 code to the panic handler and make sure it is called when a hard fault is generated.
4. Register a SysTick function that toggles a LED2 every second. You should see both LEDs blink.
::: tip

Rust does not allow global mutable variables, so the use of a crate like `LazyCell` is needed.

```rust
use once_cell::sync::Lazy;

// T can be any type 
static VALUE: Lazy<Mutex<T>> = Lazy::new(|| {
    false
});

// read it
VALUE.lock().unwrap()

// write it
let value = VALUE.lock().unwrap();
*value = ...
```

:::
5. Register an interrupt on the pin connected to button A. Toggle an LED each time the button is pressed.
6. Use `embassy-rs` to obtain the same result.
::: tip

Spawning a new task might be a good idea.

:::
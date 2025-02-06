---
layout: section
---
# Timers

---
---
# Bibliography
for this section

**Raspberry Pi Ltd**, *[RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)*
   - Chapter 2 - *System Description*
     - Chapter 2.15 - *Clocks*
       - Subchapter 2.15.1
       - Subchapter 2.15.2
   - Chapter 4 - *Peripherals*
     - Chapter 4.6 - *Timer*

---
---
# Clocks
all peripherals and the MCU use a clock to execute at certain intervals

<div grid="~ cols-2 gap-5">

<div>

| Source | Usage |
|-|-|
| *external crystal (XOSC)* | a stable frequency is required, for instance when using USB |
| *internal ring (ROSC)* | low frequency, in between 1.8 - 12 MHz (varies) |

Embassy initializes the Raspberry Pi Pico with the clock source from the 12 MHz crystal.
```rust
let p = embassy_rp::init(Default::default());
```

</div>

<div align="center">
<img src="/timers/clocks.png" class="rounded w-140">
</div>

</div>

---
---
# Frequency divider
stabilizing the signal and adjusting it

1. divides down the clock signals used for the timer, giving reduced overflow rates
2. allows the timer to be clocked at a user desires the rate

<div align="center">
<img src="/timers/clock_pipeline.png" class="rounded w-140">

<img src="/timers/clock_divider.png" class="rounded w-140">
</div>

---
layout: two-cols
---
# Counter
increments a register at every clock cycle

| Registers | Description |
|-----------|-------------|
| `value` | the current value of the counter |
| `direction` | set to count UP or DOWN |
| `reset` | UP: the value at which the counter resets to `0` DOWN: the value to which the counter resets after getting to `0`  |

<style>
.two-columns {
    grid-template-columns: 2fr 3fr;
}
</style>

:: right ::

<div align="center">
<img src="/timers/counter.svg" class="rounded w-150">
</div>

---
layout: two-cols
---
# SysTick
ARM Cortex-M time counter

<img src="/timers/systick_registers.png" class="rounded w-140">
<v-clicks>

- decrements the value of `SYST_CVR` every μs
- when `SYST_CVR` becomes `0`: 
  - triggers the `SysTick` the exception
  - next clock cycle sets the value of `SYST_CVR` to `SYST_RVR`
- `SYST_CALIB` is the value of `SYST_RVR` for a 10ms interval (might not be available)

</v-clicks>

:: right ::

### `SYST_CSR` register
<img src="/timers/systick_csr_register.png" class="rounded">

<v-click>
$$
f = \frac{1}{SYST{\_}RVR} * 1,000,000 [Hz]_{SI}
$$
</v-click>

---
layout: two-cols
---

# SysTick
ARM Cortex-M peripheral

<img src="/timers/systick_registers.png" class="rounded w-140">

```rust{all|1,7,8|2,9|3,10,11}
const SYST_RVR: *mut u32 = 0xe000_e014 as *mut u32;
const SYST_CVR: *mut u32 = 0xe000_e018 as *mut u32;
const SYST_CSR: *mut u32 = 0xe000_e010 as *mut u32;

// fire systick every 5 seconds
let interval: u32 = 5_000_000;
unsafe {
    write_volatile(SYST_RVR, interval);
    write_volatile(SYST_CVR, 0);
    // set fields `ENABLE` and `TICKINT`
    write_volatile(SYST_CSR, 0b11);
}
```

:: right ::

### `SYST_CSR` register
<img src="/timers/systick_csr_register.png" class="rounded">

### Register `SysTick`  handler

```rust
#[exception]
unsafe fn SysTick() { 
    /* systick fired */ 
}
```


---
layout: two-cols
---
# Alarm
counter that triggers interrupts after a time interval

| Registers | Description |
|-----------|-------------|
| `value` | the current value of the counter |
| `direction` | set to count UP or DOWN |
| `reset` | UP: max value before `0` DOWN: value after `0`  |
| `alarm_x` | when `value` == `alarm_x`, triggers an interrupt, `x` in `1` .. `n` |

<style>
.two-columns {
    grid-template-columns: 2fr 3fr;
}
</style>

:: right ::

<div align="center">
<img src="/timers/alarm.svg" class="rounded w-150">
</div>

---

# RP2040's Timer


<div grid="~ cols-2 gap-5">

<div>

- stores a 64 bit number (`reset` is 2<sup>64-1</sup> )
- starts with `0` at (the peripheral's) reset
- increments the number every μs
- in practice fully monotonic (cannot over flow)
- allows 4 alarms that trigger interrupts
  - `TIMER_IRQ_0`
  - `TIMER_IRQ_1`
  - `TIMER_IRQ_2`
  - `TIMER_IRQ_3`
- `alarm_0` ... `alarm_3` registers are only 32 bits wide

</div>

<div align="center">
<img src="/timers/alarm.svg" class="rounded w-150">
</div>

</div>


---
layout: two-cols
---

# RP2040's Timer
read the number of elapsed μs since reset

<img src="/timers/timer_registers_1.png" class="rounded w-100">

### Reading the time elapsed since restart

```rust{all|1,5|2,6|4,7,8}
const TIMERLR: *const u32 = 0x4005_400c;
const TIMERHR: *const u32 = 0x4005_4008;

let time: u64 = unsafe {
    let low = read_volatile(TIMERLR);
    let high = read_volatile(TIMERHR);
    high as u64 << 32 | low
}
```

The **reading order maters** as reading `TIMELR` latches the value in `TIMEHR` (stops being updated) until `TIMEHR` is read. Works only in **single core**.

:: right ::

<div align="center">
    <img src="/timers/timer_registers_2.png" class="rounded w-100">
</div>


---
layout: two-cols
---

# Alarm
triggering an interrupt at an interval

```rust
#[interrupt]
unsafe fn TIMER_IRQ_0() { /* alarm fired */ }
```

```rust{all|1,10|2,11,12|3,4,13}
const TIMERLR: *const u32 = 0x4005_400c;
const ALARM0: *mut u32 = 0x4005_4010;
// + 0x2000 is bitwise set
const INTE_SET: *mut u32 = 0x4005_4038 + 0x2000;

// set an alarm after 3 seconds
let us = 3_0000_0000;

unsafe {
    let time = read_volatile(TIMERLR);
    // use `wrapping_add` as overflowing may panic
    write_volatile(ALARM0, time.wrapping_add(us));
    write_volatile(INTE_SET, 1 << 0);
};
```

- the alarm can be set only for the lower 32 bits
- maximum 72 minutes (use *RTC* for longer alarms)

:: right ::

<div align="center">
    <img src="/timers/timer_registers_2.png" class="rounded w-100">
</div>

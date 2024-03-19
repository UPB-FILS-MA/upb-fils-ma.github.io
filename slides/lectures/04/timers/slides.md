---
layout: section
---
# Timers

---
---
# Clocks
all peripherals and the MCU use a clock to execute at certain intervals

<div align="center">
<img src="/timers/clocks.png" class="rounded w-140">
</div>

---
---
# Frequency divider
stabilizing the signal and adjusting it

<img src="/timers/clock_pipeline.png" class="rounded w-140">

<img src="/timers/clock_divider.png" class="rounded w-140">

---
---
# Counter
increments a register at every clock cycle

---
layout: two-cols
---
# SysTick
ARM Cortex-M peripheral

<img src="/timers/systick_registers.png" class="rounded w-140">

- decrements the value of `SYST_CVR` every μs
- when `SYST_CVR` becomes `0`: 
  - triggers the `SysTick` the exception
  - next clock cycle sets the value of `SYST_CVR` to `SYST_RVR`

:: right ::

### `SYST_CSR` register
<img src="/timers/systick_csr_register.png" class="rounded">

---
layout: two-cols
---
# SysTick
ARM Cortex-M peripheral

<img src="/timers/systick_registers.png" class="rounded w-140">

```rust
const SYST_RVR: *mut u32 = 0xe000_0014;
const SYST_CVR: *mut u32 = 0xe000_0018;
// + 0x2000 is bitwise set
const SYST_CSR_SET: *mut u32 = 0xe000_0010 + 0x2000;

// fire systick every 5 seconds
let interval: u32 = 5_000_000;
unsafe {
    write_volatile(SYST_RVR, interval);
    write_volatile(SYST_CVR, 0);

    // set fields `ENABLE` and `TICKINT`
    write_volatile(SYST_CSR_SET, 1 << 1 | 1);
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
---
# Timer
of the RP2040

- stores a 64 bit number
- starts with 0 at (the peripheral's) reset
- increments the number every microsecond
- in practice fully monotonic (cannot over flow)
- allows 4 alarms that trigger interrupts
  - `TIMER_IRQ_0`
  - `TIMER_IRQ_1`
  - `TIMER_IRQ_2`
  - `TIMER_IRQ_3`

---
layout: two-cols
---

# Timer
registers

<img src="/timers/timer_registers_1.png" class="rounded w-100">

## Reading the time elapsed since restart

```rust{all|1,5|2,6|4,7,8}
const TIMERHR: *const u32 = 0x4005_4008;
const TIMERLR: *const u32 = 0x4005_400c;

let time: u64 = unsafe {
    let low = read_volatile(TIMERLR);
    let high = read_volatile(TIMERLR);
    high as u64 << 32 | low
}
```

The reading order maters.

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

```rust{all|3,4|1,10|2,11|3,4,12}
const TIMERLR: *const u32 = 0x4005_400c;
const ALARM0: *mut u32 = 0x4005_4010;
// + 0x2000 is bitwise set
const INTE_SET: *mut u32 = 0x4005_4038 + 0x2000;

// set an alarm after 3 seconds
let microseconds = 3_0000_0000;

unsafe {
    let time = read_volatile(TIMERLR);
    write_volatile(ALARM0, time + microseconds);
    write_volatile(INTE_SET, 1 << 0);
};
```

- the alarm can be set only for the lower 32 bits
- maximum 72 minutes (use *RTC* for longer alarms)

:: right ::

<div align="center">
    <img src="/timers/timer_registers_2.png" class="rounded w-100">
</div>

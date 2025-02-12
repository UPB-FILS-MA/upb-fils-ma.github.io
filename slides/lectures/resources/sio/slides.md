---
layout: section
---
# GPIO
General Purpose Input Output for RP2040

---
---
# Bibliography
for this section

**Raspberry Pi Ltd**, *[RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)*
   - Chapter 2 - *System Description*
     - Section 2.3 - *Processor subsystem*
       - Subsection 2.3.1 - *SIO*
         - Subsection 2.3.1.2 - *GPIO Control*
     - Section 2.4 - *Cortex-M0+* (except NVIC and MPU)
     - Section 2.19 - *GPIO* (except Interrupts)

---
---
# RP2040 GPIO Pins
GPIO pins are connected to the processor pins through three peripherals

<div align="center">
<img src="./rp2040_gpio.svg" class="h-80 rounded" />
</div>


---
layout: two-cols
---

# GPIO

<div align="center">
<img src="./rp2040_chip.png" class="h-80 rounded" />
</div align="center">

*SIO*: Set the pin as Input or Output\
*IO Bank (GPIO)*: Use the correct MUX function (F5)\
*PAD*: Set the pin input and output parameters

::right::

## Peripherals

|  |  |
|------|-------------|
| SIO  | Single Cycle Input/Output, is able to control the GPIO pins |
| GPIO | Multiplexes the functions of the GPIO pins |

<img src="./pin_functions.png" class="rounded">
<arrow x1="520" y1="220" x2="714" y2="270" color="#0060df" width="2" arrowSize="1" />


---
layout: two-cols
---
# SIO Registers

<img src="./sio_registers.png" class="rounded">

- Input
  - set GPIO_OE bit x to 0
  - read GPIO_IN bit x
- Ouput
  - set GPIO_OE bit x to 1
  - write GPIO_OUT bit x

:: right ::

##### GPIO_OE
<img src="./sio_gpio_oe.png" class="rounded">

##### GPIO_IN
<img src="./sio_gpio_in.png" class="rounded">

##### GPIO_OUT
<img src="./sio_gpio_out.png" class="rounded">

---
layout: two-cols
---

# SIO Input

<img src="./sio_registers.png" class="rounded">

##### GPIO_OE
<img src="./sio_gpio_oe.png" class="rounded">

:: right ::

##### GPIO_IN
<img src="./sio_gpio_in.png" class="rounded">

```rust{all|4,8|4,9|10,11|4,12|5,7,13,14}
use core::ptr::read_volatile;
use core::ptr::write_volatile;

const GPIO_OE: *mut u32 = 0xd000_0020 as *mut u32;
const GPIO_IN: *const u32= 0xd000_0004 as *const u32;

let value = unsafe { 
    // write_volatile(GPIO_OE, !(1 << pin));
    let gpio_oe = read_volatile(GPIO_OE);
    // set bin `pin` of `gpio_oe` to 0 (input)
    gpio_oe = gpio_oe & !(1 << pin);
    write_volatile(GPIO_OE, gpio_oe);
    read_volatile(GPIO_IN) >> pin & 0b1
};
```


---
layout: two-cols
---

# SIO Input

<img src="./sio_registers.png" class="rounded">

##### GPIO_OE_SET
<img src="./sio_gpio_oe_clr.png" class="rounded">

:: right ::

##### GPIO_IN
<img src="./sio_gpio_in.png" class="rounded">

```rust{all|4,8,9|5,7,10,11}
use core::ptr::read_volatile;
use core::ptr::write_volatile;

const GPIO_OE_CLR: *mut u32= 0xd000_0028 as *mut u32;
const GPIO_IN: *const u32= 0xd000_0004 as *const u32;

let value = unsafe { 
	// set bit `pin` of `GPIO_OE` to 0 (input)
    write_volatile(GPIO_OE_CLR, 1 << pin);
    read_volatile(GPIO_IN) >> pin & 0b1
};
```


---
layout: two-cols
---

# SIO Output

<img src="./sio_registers.png" class="rounded">

##### GPIO_OE_CLR
<img src="./sio_gpio_oe_clr.png" class="rounded">

:: right ::

##### GPIO_OUT
<img src="./sio_gpio_out.png" class="rounded">

```rust{all|4,8,9|5,10|5,11|12|5,13}
use core::ptr::read_volatile;
use core::ptr::write_volatile;

const GPIO_OE_SET: *mut u32= 0xd000_0024 as *mut u32;
const GPIO_OUT: *mut u32 = 0xd000_0010 as *mut u32;

unsafe {
  // set bit `pin` of GPIO_OE to 1 (output)
  write_volatile(GPIO_OE_SET, 1 << pin);
  // write_volatile(GPIO_OUT, (value & 0b1) << pin);
  let gpio_out = read_volatile(GPIO_OUT);
  gpio_out = gpio_out | (value & 0b1) << pin;
  write_volatile(GPIO_OUT, gpio_out);
};
```


---
layout: two-cols
---

# SIO Output
efficient

<img src="./sio_registers.png" class="rounded">

##### GPIO_OUT_SET
<img src="./sio_gpio_out_set.png" class="rounded">

:: right ::

##### GPIO_OUT_CLR
<img src="./sio_gpio_out_clr.png" class="rounded">

```rust{all|4,9|5,6,10-13|14}
use core::ptr::read_volatile;
use core::ptr::write_volatile;

const GPIO_OE_SET: *mut u32= 0xd000_0024 as *mut u32;
const GPIO_OUT_SET:*mut u32= 0xd000_0014 as *mut u32;
const GPIO_OUT_CLR:*mut u32= 0xd000_0018 as *mut u32;

unsafe { 
    write_volatile(GPIO_OE_SET, 1 << pin);
    let reg = match value {
      0 => GPIO_OUT_CLR,
      _ => GPIO_OUT_SET
    };
    write_volatile(reg, 1 << pin);
};
```


---
layout: two-cols
---
# IO Bank
<img src="./gpio_mux.png" class="rounded">

<img src="./gpio_status_ctrl.png" class="rounded">

- set `FUNCSEL` to `5` (*SIO*)

:: right ::

##### GPIOx_CTRL
Offset: 0x004, 0x00c, ... 0x0ec (0x4 + 8*x)
<img src="./gpio_ctrl_register.png" class="rounded">

---
layout: two-cols
---

# IO Bank Input

<img src="./gpio_status_ctrl.png" class="rounded">

```rust{all|4,8|8,11|5,12|6,10,13,14}
use core::ptr::read_volatile;
use core::ptr::write_volatile;

const GPIOX_CTRL: u32 = 0x4001_4004;
const GPIO_OE_CLR: *mut u32= 0xd000_0028 as *mut u32;
const GPIO_IN: *const u32= 0xd000_0004 as *const u32;

let gpio_ctrl = (GPIOX_CTRL + 8 * pin) as *mut u32;

let value = unsafe { 
    write_volatile(gpio_ctrl, 5);
    write_volatile(GPIO_OE_CLR, 1 << pin);
    read_volatile(GPIO_IN) >> pin & 0b1
};
```

:: right ::

##### GPIOx_CTRL
Offset: 0x004, 0x00c, ... 0x0ec (0x4 + 8*x)
<img src="./gpio_ctrl_register.png" class="rounded">


---
layout: two-cols
---

# IO Bank Output

<img src="./gpio_status_ctrl.png" class="rounded">

```rust{all|4,9|9,11|5,12|6,7,13-16|17}
use core::ptr::read_volatile;
use core::ptr::write_volatile;

const GPIOX_CTRL: u32 = 0x4001_4004;
const GPIO_OE_SET: *mut u32= 0xd000_0024 as *mut u32;
const GPIO_OUT_SET:*mut u32= 0xd000_0014 as *mut u32;
const GPIO_OUT_CLR:*mut u32= 0xd000_0018 as *mut u32;

let gpio_ctrl = (GPIOX_CTRL + 8 * pin) as *mut u32;
unsafe { 
    write_volatile(gpio_ctrl, 5);
    write_volatile(GPIO_OE_SET, 1 << pin);
    let reg = match value {
      0 => GPIO_OUT_CLR,
      _ => GPIO_OUT_SET
    };
    write_volatile(reg, 1 << pin);
};
```

:: right ::

##### GPIOx_CTRL
Offset: 0x004, 0x00c, ... 0x0ec (0x4 + 8*x)
<img src="./gpio_ctrl_register.png" class="rounded">


---
layout: two-cols
---
# Pad Control

<img src="./gpio_pads.png" class="rounded">

<img src="./gpio_pad_registers.png" class="rounded">

:: right ::
##### GPIOx Register
Offset: 0x004, 0x008, ... 0x078 (0x4 + 4*x)
<img src="./gpio_pad_ctrl_2.png" class="rounded">
<img src="./gpio_pad_ctrl_1.png" class="rounded">

---
layout: two-cols
---

# Input
read the value from pin `x`

- set the `FUNCSEL` field of `GPIOx_CTRL` to `5`
- set the `GPIO_OE_CLR` bit `x` to `1`
- read the `GPIO_IN` bit `x`
- *adjust the `GPIOx` fields to set the pull up/down resistor*

<img src="./pin_input.png" class="w-50 rounded">

:: right ::

# Output
write a value to pin `x`

- set the `FUNCSEL` field of `GPIOx_CTRL` to `5`
- set the `GPIO_OE_SET` bit `x` to `1`
- if the value 
  - is `0`, set the `GPIO_OUT_CLR` bit `x` to `1`
  - is `1`, set the `GPIO_OUT_SET` bit `x` to `1` 
- *adjust the `GPIOx` fields to set the output current*

<img src="./pin_output.png" class="w-50 rounded">

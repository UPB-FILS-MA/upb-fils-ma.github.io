---
layout: section
---
# embassy-rs
[Embedded Asynchronous](https://embassy.dev/)

---
---
# embassy-rs

- framework
- uses the rust-embedded-hal
- Features
  - Real-time
  - Low power
  - Networking
  - Bluetooth
  - USB
  - Bootloader and DFU

---
---
# GPIO Input

```rust {all|5|8,9,18|10|6,11-17}
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use gpio::{Input, Pull};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let pin = Input::new(p.PIN_3, Pull:Up);

    if pin.is_high() {

    } else {

    }
}
```

The `main` function is called by the embassy-rs framework, so it can exit.

---
---
# GPIO Output

```rust {all|5|8,9,18|10|6,11-13}
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use gpio::{Level, Output};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut pin = Output::new(p.PIN_2, Level::Low);

    pin.set_high();
}
```

The `main` function is called by the embassy-rs framework, so it can exit.

---
---
# Processor Exceptions
what happens if something does not work as required

![Exceptions](/exceptions/exceptions.svg)

---
---
# ARM Cortex-M0+ Exceptions
what happens if something does not work as required

![Exceptions](/exceptions/cortex-m.svg)

---
---
# Exception (HardFault) Handling
ARM Cortex-M0+ has one **actual expection**, *HardFault*

```mermaid
flowchart LR
    F(Fetch Next 
    Instruction)
    F --> S2{Success?}
    S2 -- Yes --> A
    S2 -- No --> V
    A(Execute 
    Instruction) --> B{Success?}
    R --> F
    B -- Yes --> R2{Return 
    from 
    Exception?}
    R2 -- Yes --> R(Restore/Pop State)
    R2 -- No --> F
    B -- No --> V{Running 
    HardFault 
    Handler?}
    %% V -- Executing Double Fault --> R3(Triple Fault/Reset)
    %% F2 --> S
    V -- No --> S(Save/Push State)
    S --> E(Jump to Exception 
    Handler)
    E --> F
    S3(PowerUp) -- Reset Exception --> E
    V -- Yes --> F2(Lockup/Reset)

    classDef memory fill:#B0E3E6,stroke:#0E8088
    classDef instruction fill:#B1DDF0,stroke:#10739E
    classDef processor fill:#FFE6CC,stroke:#D79B00
    classDef exception fill:#F8CECC,stroke:#B85450
    classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
    classDef start fill:#00ef00

    class A instruction
    class F,R,S memory
    class B,V,S2,R2 processor
    class E exception
    class R3,F2 error
    class S3 start
```

- the exception table of RP2040 at address 0x1000_0100 (start of the boot area + 4 bytes)
- the processor generates a *Reset* exception when it starts

---
---

# ARM Cortex-M0+ Interrupts
some hardware device notifies the MCU

<div align="center">
<img src="/exceptions/cortex-m-nvic.svg" class="rounded w-170">
</div>

---
---
# Interrupt Handling
ARM Cortex-M0+

```mermaid
flowchart LR
    F(Fetch Next Instruction) --> I{Higher
    Priority 
    IRQ?}
    I -- Yes --> S
    I -- No --> A
    A(Execute Instruction) --> R2{Return 
    from 
    ISR?}
    R2 -- Yes --> I2{IRQ}
    I2 -- Yes --> E
    I2 -- No --> R(Restore/Pop
    State)
    R2 -- No --> I3{Higher
    Priority
    IRQ
    }
    I3 -- Yes --> S
    I3 -- No --> F
    R --> I3
    S(Save/Push 
    State)
    S --> E(Jump to ISR)
    E --> I3

    classDef memory fill:#B0E3E6,stroke:#0E8088
    classDef instruction fill:#B1DDF0,stroke:#10739E
    classDef processor fill:#FFE6CC,stroke:#D79B00
    classDef exception fill:#F8CECC,stroke:#B85450

    class A instruction
    class F,R,S memory
    class B,V,S2,R2,I,I2,I3 processor
    class E exception
    class R3,F2 error
    class S3 start
```

<div grid="~ cols-2 gap-2">

| | |
|-|-|
| *IRQ* | Interrupt Request |
| *ISR* | Interrupt Service Routine |

<div>

- the interrupt vector (table) of RP3040 starts at address 0x1000_0040 (after the exceptions table with 15 interrupts)
- ARM Cortex-M0+ has a maximum of 32 interrupt requests (IRQs)

</div>
</div>

---
---
# Exceptions are Software Interrupt Requests
with a negative IRQ number and a higher priority

```mermaid
flowchart LR
    F(Fetch Next 
    Instruction) --> I{Higher
    Priority 
    IRQ?}
    I -- Yes --> S
    I -- No --> A
    I -- Is HardFault --> H{In 
    HardFault 
    or NMI 
    ISR?}
    H -- Yes --> L(Lockup
    or
    Reset)
    H -- No --> S
    A(Execute 
    Instruction) --> R2{Return 
    from 
    ISR?}
    R2 -- Yes --> I2{IRQ?}
    I2 -- Yes --> E
    I2 -- No --> R(Restore/Pop 
    State)
    R2 -- No --> I3{Higher
    Priority
    IRQ?
    }
    R --> I3
    I3 -- HardFault
    or SVC --> H
    I3 -- Yes --> S
    I3 -- No --> F
    S(Save/Push 
    State)
    S3(PowerUp) -- Reset Exception --> E
    S --> E(Jump to ISR)
    E --> I3

    classDef memory fill:#B0E3E6,stroke:#0E8088
    classDef instruction fill:#B1DDF0,stroke:#10739E
    classDef processor fill:#FFE6CC,stroke:#D79B00
    classDef exception fill:#F8CECC,stroke:#B85450
    classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
    classDef start fill:#00ef00

    class A instruction
    class F,R,S memory
    class B,V,S2,R2,I,I2,I3,H processor
    class E exception
    class L error
    class S3 start
```

<div grid="~ cols-2 gap-2">

<div>

- Reset (-14)
- HardFault (-13)
- SVC (-5)
- PendSV (-2)
- SysTick (-1)
</div>
  
![RP2040 Interrupts](/exceptions/interrupts.png)

</div>

---
---
# Boot
how the ARM Cortex-M0+ starts

```mermaid
flowchart LR
    S(PowerUp) --> R(Read Interrupt 
    Vector/Table from
    @start_address)
    R -- Error --> L(Lockup
    or
    Reset)
    R -- Success --> S2(Set Stack 
    Pointer)
    S2 -- Invalid --> L
    S2 --> E(Jump to Reset 
    Exception Handler)
    E -- Invalid --> L
    E --> F(Fetch Instruction)

    classDef memory fill:#B0E3E6,stroke:#0E8088
    classDef instruction fill:#B1DDF0,stroke:#10739E
    classDef processor fill:#FFE6CC,stroke:#D79B00
    classDef exception fill:#F8CECC,stroke:#B85450
    classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
    classDef start fill:#00ef00

    class A,E instruction
    class R,S2,F memory
    class B,V,S2,R2,I,I2,I3,H processor
    class L error
    class S start
```

- the *start_address* for RP2040 is 0x1000_0100
- RP2040 has another boot loader that it loads from 0x1000_0000


---
layout: two-cols
---
# Boot

<style>
.two-columns {
    grid-template-columns: 2fr 1fr;
}
</style>

The RP2040 boot process

```mermaid
flowchart LR
    S(PowerUp) --> I
    subgraph Internal Boot Loader
        I{BOOTSEL
        Pressed} -- Yes --> U
        I -- NO --> R(Read 
        Boot Loader 
        @x1000_0000)
        R -- Error --> U(Show USB 
        Drive)
    end
    R -- Success --> E2(Load
    Interrupt
    Vector)
    E2 -- Fault --> E3(Jump to 
    HardFault 
    Handler)
    E3 -- Fault --> L(Lockup
    or
    Reset)
    E2 --> S2(Set Stack 
    Pointer)
    S2 --> E(Jump to 
    Reset 
    Exception 
    Handler)
    E -- Invalid --> E3
    E --> F(Fetch 
    Instruction)

    classDef memory fill:#B0E3E6,stroke:#0E8088
    classDef instruction fill:#B1DDF0,stroke:#10739E
    classDef processor fill:#FFE6CC,stroke:#D79B00
    classDef exception fill:#F8CECC,stroke:#B85450
    classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
    classDef start fill:#00ef00

    class A,E,E2,S2 instruction
    class R,F memory
    class B,V,R2,I,I2,I3,H,U processor
    class L error
    class E3 exception
    class S start
```

The internal boot loader cannot be overwritten and assures that bricking the device is difficult.

:: right ::

<div align="center">
<img src="/exceptions/flash_address.svg" class="rounded w-70">
</div>

---
---
# Set Fault Handler
bare metal, pac or embassy-rs


```rust {displayLines:false}
// defined by the cortex-m-rt crate
pub struct ExceptionFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    xpsr: u32,
}
```

`HardFault` never returns

```rust{all|1|2,4|3}
#[exception]
unsafe fn HardFault(_frame: &ExceptionFrame) -> ! {
    panic!("HardFault {:?}", frame);
}
```

---
---
# Set SysTick Handler
bare metal, PAC or embassy-rs

```rust{all|1|2,4|3}
#[exception]
unsafe fn SysTick() {
    // execute at a fixed interval   
}
```

---
---
# Set Interrupt Handlers
bare metal, PAC

*embassy-rs already defined the interrupts as it needs them*

<div align="center">
<img src="/exceptions/interrupts.png" class="w-140 rounded">
</div>

```rust{all|1|2,4|3}
#[interrupt]
unsafe fn IO_IRQ_BANK0 {
    // so some work when a pin interrupt triggers
}
```

---
---
# Use interrupts in embassy-rs
embassy-rs registers interrupt handlers and exposes a high level API

<div align="center">
<img src="/exceptions/interrupts.png" class="w-90 rounded">
</div>

```rust{all|4|9,10,11}
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut button = Input::new(p.PIN_20, Pull::None);

    loop {
        info!("Waiting for the button press");

        // waits for interrupt (sent by button)
        // IO_IRQ_BANK0
        button.wait_for_high().await;

        info!("Button was pressed");
    }
}
```

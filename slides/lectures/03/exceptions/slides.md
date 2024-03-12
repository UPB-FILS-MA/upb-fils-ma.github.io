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

- the exception table is *usually* at address 0x0000_0004 (start of the boot area + 4 bytes)
- the processor generates a *Reset* exception when it starts

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
    I2 -- No --> R(Restore/Pop State)
    R2 -- No --> I3{Higher
    Priority
    IRQ
    }
    I3 -- Yes --> S
    I3 -- No --> F
    R --> I3
    S(Save/Push State)
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

- the interrupt vector (table) is *usually* at address 0x0000_0040 (after the exceptions table with 15 interrupts)
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
    S(PowerUp) --> R(Read Exception 
    Table from
    @0x0000_0000)
    R -- Error --> L(Lockup
    or
    Reset)
    R -- Success --> S2(Set Stack 
    Pointer)
    S2 -- Invalid --> L
    S2 --> E(Jump to Reset 
    Exception Handler)
    E -- Invalid --> L

    classDef memory fill:#B0E3E6,stroke:#0E8088
    classDef instruction fill:#B1DDF0,stroke:#10739E
    classDef processor fill:#FFE6CC,stroke:#D79B00
    classDef exception fill:#F8CECC,stroke:#B85450
    classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff
    classDef start fill:#00ef00

    class A instruction
    class R,S2 memory
    class B,V,S2,R2,I,I2,I3,H processor
    class E exception
    class L error
    class S start
```

---
---

# Exception Handling
x86

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
    Exception 
    Handler}
    V -- Yes --> F2(Double Fault)
    V -- Executing Double Fault --> R3(Triple Fault/Reset)
    F2 --> S
    V -- No --> S(Save/Push State)
    S --> E(Jump to 
    Exception Handler)
    E --> F

    classDef memory fill:#B0E3E6,stroke:#0E8088
    classDef instruction fill:#B1DDF0,stroke:#10739E
    classDef processor fill:#FFE6CC,stroke:#D79B00
    classDef exception fill:#F8CECC,stroke:#B85450
    classDef error fill:#ff0000,stroke:#ae0000,color:#ffffff

    class A instruction
    class F,R,S memory
    class B,V,S2,R2 processor
    class E,F2 exception
    class R3 error
```

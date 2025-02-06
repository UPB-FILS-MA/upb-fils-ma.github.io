---
layout: section
---
# Operating System
the purpose of and OS

---
---
# Bibliography
for this section

**Andrew Tanenbaum**, *Modern Operating Systems (4th edition)*
   - Chapter 1 - *Memory Management*
     - Subchapter 1 - *Introduction*
       - Subchapter 1.1 - *What is an operating system?*
       - Subchapter 1.6 - *System calls*
       - Subchapter 1.7 - *Operating system structure*

---
layout: two-cols
---
# Operating System
the main role

<style>
.two-columns {
    grid-template-columns: 2fr 3fr;
}
</style>

**Allow Portability**
- provides a hardware independent API
- applications should run on any hardware

**Resources Management and Isolation**
- allow applications to access resources
- prevent applications from accessing hardware directly
- isolate applications

:: right ::

<img src="/os/os.svg" class="w-120 rounded" />

---
layout: two-cols
---
# Desktop and Server Operating Systems
abstractions

<style>
.two-columns {
    grid-template-columns: 2fr 3fr;
}
</style>

**Actions**
- **process** and **threads**
- use the *Processor* and *Accelerators* (GPU, Neural Engine, etc)

**Data**
- everything is a file
- peripherals are viewed as files (*POSIX*)
  - `/sys/class/gpio/gpio5/direction`
  - `/sys/class/gpio/gpio5/value`

:: right ::

<img src="/os/abstractions.svg" class="w-120 rounded" />

---
layout: two-cols
---
# Embedded Operating Systems
<div></div>

<style>
.two-columns {
    grid-template-columns: 2fr 3fr;
}
</style>

**Actions**
- **process** or **threads**
- use the *Processor* and *Accelerators* (Crypto Engines, Neural Engine, etc)

**Peripheral**
- provide a hardware independent API
- prevent processes from accessing the peripheral

*usually* the applications and the kernel are compiled together into a **single binary**

:: right ::

<img src="/os/embedded_os.svg" class="w-120 rounded" />

---
---
# Scheduling Type
could a process stop the whole system?

**Preemptive**
- processes can be suspended by the scheduler
- a misbehaving process cannot stop the system

**Cooperative**
- processes **cannot be suspended** by the kernel
- a misbehaving process **can stop** the system


---
---
# Kernel Types
from the **kernel and drivers** point of view

<div grid="~ cols-3 gap-5">

<div>

**Monolothic**

<img src="/os/monolithic.svg" class="w-73 rounded" />

  - all drivers in the kernel
  - Windows, Linux, MacOS


</div>

<div>

**Microkernel**

<img src="/os/microkernel.svg" class="w-80 rounded" />

  - all drivers are applications
  - Minix

</div>

<div>

**Unikernel**

<img src="/os/unikernel.svg" class="w-75 m-1 rounded" />

  - the kernel is bundled with all the drivers and one single application
  - Unikraft/Linux
  - Most of the microcontroller RTOSes


</div>

</div>


---
layout: two-cols
---
# System Call
the OS API

<style>
.two-columns {
    grid-template-columns: 2fr 3fr;
}

.overlap{
    top: -90px;
    position: relative;
    left: 190px;
    border: 1px dashed;
    padding: 3px;
}
</style>

**accessing a peripheral** can be **performed** only **by the OS**

The application:

<v-clicks>

1. puts values in the registers
2. triggers an exception 
   - `svc` instruction for ARM 

</v-clicks>

The OS:

<v-clicks>

1. looks at the registers and determines what the required action is
2. performs the action
3. puts the return values into the registers

</v-clicks>

:: right ::

<img src="/os/system_call.svg" class="w-120 rounded" />

<img src="/os/exceptions.svg" class="w-80 rounded overlap" />

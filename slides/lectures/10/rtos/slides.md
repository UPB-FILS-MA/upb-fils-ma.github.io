---
layout: section
---
# Embedded Operating Systems
aka Real-Time Operating Systems (RTOS)

---
---
# Bibliography
for this section

**Alexandru Radovici, Ioana Culic**, *Getting Started with Secure Embedded Systems*
   - Chapter 2 - *Embedded systems software development*

---
---
# Embedded Operating Systems

- small OSes that run on microcontrollers
- most of the times called *Real Time OS* (*RTOS*)
- applications are similar to *threads* (are considered friendly)
- the whole system is compiled into a single binary
- similar to frameworks

---
---
# Real Time?
upper bound

- **real time** means **performing** an action **always** in a **deterministic** amount of **time**
- the amount of time can be large
- **low latency** means that the amount if time must be small

The industry often uses real time interchangeably low latency.

---
---
# Most Used

| OS | Owner | Description |
|----|-------|-------------|
| FreeRTOS | Amazon | Oldest RTOS, heavily used in the industry. |
| SafeRTOS | High Integrity Systems | Certified for functional safety, based on FreeRTOS. |
| Zephyr | Linux Foundation | *Linux'es little brother*, has an API inspired by Linux, is getting traction. |

<div grid="~ cols-2 gap-5">

<img src="/rtos/freertos.png" class="w-50 rounded">
<img src="/rtos/zephyros.png" class="w-50 rounded">

</div>

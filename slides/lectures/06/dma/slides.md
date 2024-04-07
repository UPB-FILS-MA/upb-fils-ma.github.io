---
layout: section
---

# DMA
Direct Memory Access

---
---
# Bibliography
for this section

**Raspberry Pi Ltd**, *[RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)*
   - Chapter 2 - *System Description*
     - Chapter 2.5 - *DMA*


---
layout: two-cols
---
# DMA

<style>
.two-columns {
    grid-template-columns: 3fr 4fr;
}
</style>

- offloads the MCU from doing **memory to memory** operations
- due to MMIO, usually implies **transfers from and to peripherals**
- raises an interrupt when a transfer is done

:: right ::

<img src="/dma/dma.svg" class="rounded">

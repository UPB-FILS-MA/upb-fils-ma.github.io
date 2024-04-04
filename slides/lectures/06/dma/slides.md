---
layout: section
---
# DMA
Direct Memory Access

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
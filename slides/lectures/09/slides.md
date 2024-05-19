---
theme: seriph
# background: https://source.unsplash.com/collection/94734566/1920x1080
class: text-center
highlighter: shiki
lineNumbers: true
info: |
  ## Memory Protection
drawings:
  persist: false
defaults:
  foo: true
transition: slide-left
title: MA - 09 - Memory Protection
mdc: true
layout: cover
themeConfig:
  primary: '#0060df'
download: true
exportFilename: ma-09
background:
---

# Memory Protection
Lecture 9

---
---

# Memory Protection

- Memory Protection Unit
- Memory Management Unit

<!-- mpu -->

---
src: ./mpu/slides.md
---

<!-- mmu -->

---
src: ./mmu/slides.md
---

---
layout: two-cols
---
# Microcontroller (MCU)
Integrated in embedded systems for certain tasks

- low operating frequency (MHz)
- a lot of I/O ports
- controls hardware
- does not require an Operating System
- costs $0.1 - $25
- uses **Memory Protection Unit**

<img src="/pico.jpg" class="m-5 h-30 rounded">


:: right ::

# Microprocessor (CPU)
General purpose, for PC & workstations

- high operating frequency (GHz)
- limited number of I/O ports
- usually requires an Operating System
- costs $75 - $500
- uses **Memory Management Unit**

<img src="/pi5.jpg" class="m-5 h-50 rounded">

---
---
# Conclusion
we discussed about

- Memory Protection Unit
- Memory Management Unit

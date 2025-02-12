---
theme: seriph
# background: https://source.unsplash.com/collection/94734566/1920x1080
class: text-center
highlighter: shiki
lineNumbers: true
info: |
  ## Memory Mapped IO for GPIO
drawings:
  persist: false
defaults:
  foo: true
transition: slide-left
title: MA - 02 - Memory Mapped IO
mdc: true
layout: cover
themeConfig:
  primary: '#0060df'
download: true
exportFilename: ma-02
background:
---

# Memory Mapped IO used for GPIO
Lecture 2

---
---

# GPIO for RP2040

- Memory Mapped I/O
  - GPIO Peripheral
- Embedded Rust Stack
- embassy-rs

<!-- MMIO -->

---
src: ../../resources/mmio/slides.md
---

<!-- Bitwise -->

---
src: ../../resources/bitwise/rust.md
---

<!-- SIO -->

---
src: ../../resources/sio/slides.md
---

<!-- rust-embedded -->

---
src: ../../resources/rust-embedded/slides.md
---

<!-- embassy-rs -->

---
src: ../../resources/embassy-rs/slides.md
---

---
---
# Conclusion
we talked about

- Memory Mapped IO
- RP2040 GPIO
  - Single Cycle IO
  - IO Bank
  - Pad
- The Rust embedded standard stack
- Bare metal Rust
- The embassy-rs framework

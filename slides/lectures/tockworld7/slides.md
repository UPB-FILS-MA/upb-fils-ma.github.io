---
theme: seriph
# background: https://source.unsplash.com/collection/94734566/1920x1080
class: text-center
highlighter: shiki
lineNumbers: true
info: |
  ## Teaching Embedded Systems with Tock
drawings:
  persist: false
defaults:
  foo: true
transition: slide-left
title: Teaching Embedded Systems with Tock
mdc: true
layout: cover
themeConfig:
  primary: '#0060df'
download: true
exportFilename: tockworld7
background:
---

# Teaching Embedded Systems with Tock
Alexandru Radovici\
Politehnica University of Bucharest

---

# Embedded Systems in Rust
we taught an [embedded systems](https://embedded-rust-101.wyliodrin.com) course fully in Rust

<div grid="~ cols-2 gap-3">

<div>

## Students learned

- how hardware works
- how to actually build their own hardware device
- the Rust programming Language

## We used
- the `embassy` framework
- `async` Rust
- Rust Embedded `async` HAL

</div>

<div>

<div align="center">
  <img src="logo.svg">
</div>

</div>

</div>

**90** second year **students** built **70 projects** using the Raspberry Pi Pico and Rust

<!-- Subjects -->

---
src: ./subjects.md
---

<!-- The Good -->

---
src: ./good/slides.md
---

<!-- Issues -->

---
src: ./issues/slides.md
---

<!-- Tock -->

---
src: ./tock/slides.md
---

---
---
# Conclusion
Tock could be the standard for embedded systems courses

- There is a lof of work to do
- We have 5 interns for the summer that will work on this
- Try to teach common courses or at least parts of them

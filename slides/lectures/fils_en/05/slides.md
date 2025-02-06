---
theme: seriph
# background: https://source.unsplash.com/collection/94734566/1920x1080
class: text-center
highlighter: shiki
lineNumbers: true
info: |
  ## Asynchronous Development
drawings:
  persist: false
defaults:
  foo: true
transition: slide-left
title: MA - 05 - Asynchronous Development
mdc: true
layout: cover
themeConfig:
  primary: '#0060df'
download: true
exportFilename: ma-05
background:
---

# Asynchronous Development
Lecture 5

---
---

# Asynchronous Development

- Concurrency
- Asynchronous Executor
- `Future`s
- Communication between tasks

<!-- Concurrency -->

---
src: ./concurrency/slides.md
---

<!-- Executor -->

---
src: ./executor/slides.md
---

<!-- Future -->

---
src: ./future/slides.md
---

<!-- Communication -->

---
src: ./communication/slides.md
---

---
---
# Conclusion
we talked about

- Preemptive & Cooperative Concurrency
- Asynchronous Executor
- `Future`s and how Rust rewrites `async` function
- Communication between tasks

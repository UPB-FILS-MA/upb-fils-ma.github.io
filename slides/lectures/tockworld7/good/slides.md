---
layout: section
---
# The Good
what worked well

---
---
# The Good
why did we use embassy

- `embassy` looks pretty simple to use
- the Raspberry Pi Pico is very well supported
  - has WiFi
- the *Rust Embedded HAL* is emplemented, in theory, students could you any crates
- allows the writing of *multi-threaded* applications
  - easier to do than writing state machines

---
---
# `async`/`.await` worked out great
some say *do not use `async`/`.await` for beginners


- initially told students to just write `.await` at the end
- explained how `async` Rust works

<center>

![executor](/good/executor.svg)

</center>

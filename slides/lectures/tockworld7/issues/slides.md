---
layout: section
---
# Issues
these are some of the issues that we faced

---
---
# `panic`
debugging was almost impossible

- RP2040 has no debugger (the debugger is more expensive than the actual chip)
- we used the USB logger for prints
- when `embassy` panics, everything stops, no output, maybe an LED blink
  - if RP2040 has WiFi, not even that, the LED is via SPI

---
---
# No release plan
embassy is a *one person show*

**there is no release plan**

*throughout the course, at some point, running `cargo build` on the repo would fail due to version incompatibilities between embassy's own crates*

<center>

![version](/issues/version.png)

</center>

- talked to Dario, wrote `embassy` for himself, not sure he wants to fully support it

---
---
# Breaking changes
with no major version increase

1. we sumitted a PR and renamed the `PWM_CHANNEL` to `PWM_SLICE`
   - got accepted immediatly
   - public doc changed immediatly
   - no major version increase

2. the `Pin` type changed throughout the semester
   - depeding on when students downloade embassy, they had to use it differently
   - libraries would fail
   - no git tag for the most recent working release

3. WiFi only worked with the git creates

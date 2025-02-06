---
layout: section
---
# Signals
Analog and Digital

---
---
# Signals
Analog vs Digital

<div grid="~ cols-2">

<div>

- *analog signals* are *real* signals
- *digital signals* are *a numerical representation* of an analog signal
- hardware usually works with two-level digital signals

#### Exceptions
- \>= 100Mbit Ethernet
- WiFi
- SSD storage

</div>

![AD](./a_d.png)

</div>

---
---

# Why use digital?
in computing

<div grid="~ cols-2">

<div>

Signal that we *want* to generate with an output pin

![Digital Step](./digital_step.svg)

</div>

<div>
<v-click>

Signal that what we actually generate

![Analog Step](./analog_step.svg)
</v-click>
</div>

</div>

---
---
# Noise Margin

<div align="center">

![Noise](./noise.svg)

</div>

---

# Prevent Errors
using digital signals

<div grid="~ cols-2">

<div>

- use higher voltage
  - high noise margin 
  - higher power consumption ...
- lower noise by using better electronic circuits
- every device *samples and regenerates* the signal

</div>

![RP2040](./rp2040_chip.png)

</div>

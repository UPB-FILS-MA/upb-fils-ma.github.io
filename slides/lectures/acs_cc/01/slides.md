---
theme: seriph
# background: https://source.unsplash.com/collection/94734566/1920x1080
class: text-center
highlighter: shiki
lineNumbers: true
info: |
  ## Introduction to microprocessors
drawings:
  persist: false
defaults:
  foo: true
transition: slide-left
title: MA - 01 - Introduction
mdc: true
layout: cover
themeConfig:
  primary: '#0060df'
download: true
exportFilename: pm_cc-01
background:
---

# Introduction into PM
Lecture 1

---

# Welcome
to the *Proiectarea cu Microprocesoare* engineering class  

<v-click>


### You will learn

- how hardware works
- how to actually build your own hardware device
- the Rust programming Language
- a little bit of low level C

</v-click>

<v-click>


<br>

### We expect
- to come to class
- ask a lot of questions
- maybe some work at home

</v-click>


<br> 

<v-click>

### 2025 is an experiment - we will keep it chill

</v-click>

<!--
DR intro
-->

---
src: ./dr_intro.md
---


<!-- Team -->
---
src: ./team.md
---

<!-- Admin -->
---
layout: section
---



# Outline

---

# Outline

<div grid="~ cols-2 gap-5">
<div>

## Lectures
  - 12 lectures
  - 1 Q&A lecture for the project

## Labs
  - 12 labs

## Project
- Build a hardware device running software written in Rust or C on a microcontroller-based board
- The cost for the hardware is around 150 RON
- Presented at PM Fair during the last week of the semester

</div>

<img src="/img/pm_fair.png" class="w-120">

</div>

---

# Structura Punctaj

## 1 punct activitate laborator 
## 1 punct lucrare laborator (colocviu final)
<br>

## 3 puncte proiect
<br> 

## 3 puncte examen final

<br> 

## 2 puncte activitate curs (teste anuntate) 

<br> 

<i>

## Bonus 
### + 0.75 bonus top 30 de proiecte din an (top 7%)
### + 0.75 bonus top 10 proiecte din an 

</i>

---

# Extra

## Bonus pentru rezultate la concursuri & activități
până la 1 punct pentru rezulate in top la concursuri de profil tehnic
până la 0,5 puncte bonus pentru implicare în activități studențesti de voluntariat
> mail în presesiune cu Subject: [Bonus_PM] Nume_Prenume_32xCC

## Echivalari

Pana la 3 puncte pentru rezultate la concursuri tehnice. 
<br> 

Exemple:
### ACM (top 50% la world finals);
### Innovation Labs (SemiFinale);
### Suceava Hard and Soft (top 50% echipe);

---

# Proiect

## Structura punctaj

### Documentatie / Hard / Soft
### PM-fair

<br> 

## Alegerea temei

> Tema trebuie să fie aprobată de către asistentul de laborator


<br>

## Nu se aprobă teme banale!

<i> 

(Ceas digital, termometru digital)

</i>

## Ca reguli de referință:
> Nu poate să fie mai simplu decât un laborator de PM 
<br>

> Nu poate să fie bazat pe un tutorial youtube de 15 minute

<!--
AGC
-->

---
src: ../../resources/agc/slides.md
---

---

# Where we are now

<br> <br> <br> 

<img src="/img/where.png" class="w-full mx-auto block">

---

# Embedded Systems

## In general, they have a dedicated function.
<br>

## Common constraints:

### Real-time requirements
### Fixed response time:
#### - Control (e.g., constant-time sampling)
#### - Safety (response within a limited time upon detection)
### Limited resources (processing power/memory)
### Robustness requirements (aka high uptime)

---

# Example

<img src="/img/car.png" class="w-full mx-auto block">

---

# Example controller

<br>

[NXP S32ZE](https://www.nxp.com/products/processors-and-microcontrollers/s32-automotive-platform/s32z-and-s32e-real-time-processors/s32e2-safe-and-secure-high-performance-real-time-processors-with-actuation-support:S32E2)

<br>

[STM32H](https://www.st.com/en/microcontrollers-microprocessors/stm32h757zi.html)

---

# Example ENTy

<img src="/img/enty.png" class="w-full mx-auto block">

---

# Example Companies

<br> 

<div style="column-count: 3; column-gap: 20px; font-size: 24px;">
    <p>NXP </p>
    <p>Infineon </p>
    <p>Microchip </p>
    <p>EPG </p>
    <p>Renault </p>
    <p>Continental </p>
    <p>Viavi </p>
    <p>Siemens </p>
    <p>Emerson </p>
    <p>GE </p>
    <p>Honeywell </p>
    <p>Thales </p>
   <p> Hella </p>
   <p> Bosch </p>
 
</div>

<!--
Processor
-->

---
src: ../../resources/processor/slides.md
---

---
src: dr_arhitecture.md
---

<!-- MCUs -->

---
src: ../../resources/mcu/slides.md
---

<!-- RP2040 -->

---
src: ../../resources/rp2040/slides.md
---

---
---
# Conclusion
we talked about

- How a processor functions
- Microcontrollers (MCU) / Microprocessors (CPU)
- Microcontroller architectures
- ARM Cortex-M
- RP2040

---
src: avr.md
---


---
src: dr_software.md
---

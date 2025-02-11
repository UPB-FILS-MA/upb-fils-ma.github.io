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

# Introduction
Lecture 1

---

# Welcome
to the *Microprocessor Architecture* engineering class

## You will learn

- how hardware works
- how to actually build your own hardware device
- the Rust programming Language

## We expect
- to come to class
- ask a lot of questions

<!-- Team -->
---
src: ./team.md
---

<!-- Admin -->

---
src: ../../resources/admin/slides.md
---

<!-- Subjects -->

---
src: ./subjects.md
---

<!-- AGC -->

---
src: ../../resources/agc/slides.md
---

<!-- Processor -->

---
src: ../../resources/processor/slides.md
---

<!-- MCUs -->

---

# Hello World on AVR in C 

```c

#include <avr/io.h>
#include <util/delay.h>

#define F_CPU 12000000UL //MCU clock frequency 

int main()
{
    DDRC = (1 << PC0); //Set pin 0 of PORT C as output 
    //DDRC = Data Direction Register for PORT C
    while(1)
    {
        PORTC ^= (1 << PC0); //Toggle pin 0 of PORT C (XOR)
        _delay_ms(500);
    }
}

```

> Note: the above code can toggle an LED on / off every 500ms

---

# Let's go lower level 
```c

//00000000 <__vectors>: 
//__vectors(): 
0: 0c 94 3e 00 jmp 0x7c  ; 0x7c <__ctors_end>  //reset
4: 0c 94 48 00 jmp 0x90  ; 0x90 <__bad_interrupt> 
8: 0c 94 48 00 jmp 0x90  ; 0x90 <__bad_interrupt> 
c: 0c 94 48 00 jmp 0x90  ; 0x90 <__bad_interrupt> 
10: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
14: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
18: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
1c: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
20: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
24: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
28: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 

................................................

60: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
64: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
68: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
6c: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
70: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
74: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt> 
78: 0c 94 48 00 jmp 0x90 ; 0x90 <__bad_interrupt>
0000007c <__ctors_end>:


```

---

# Next code

```c

//__trampolines_start(): 
   7c: 11 24 	eor r1, r1    ; r1 = 0           //program jumps here at reset
   7e: 1f be 	out 0x3f, r1  ; SREG = r1 
   80: cf ef 	ldi r28, 0xFF ; 255 
   82: d8 e0 	ldi r29, 0x08 ; 8 
   84: de bf 	out 0x3e, r29 ; SPH = 0x8        //stack pointer on the last RAM address - 0x08FF for 328P
   86: cd bf 	out 0x3d, r28 ; SPL = 0xFF.      //stack Pointer High and Low - to get a 16b address on a 8bit MCU
   88: 0e 94 4a 00 	call 0x94 	  ; 0x94 <main> 
   8c: 0c 94 59 00 	jmp 0xb2 	  ; 0xb2 <_exit> 00000090 

//<__bad_interrupt>: __vector_22(): 
   90: 0c 94 00 00 	jmp 0 ; 0x0 <__vectors>.   //any interrupt triggers a reset


```

---

# We get to the code

```c

   94: 38 9a sbi 0x07, 0 ; DDRC = 0x01         //DDRC |= (1 << PC0); 
     
   96: 91 e0 ldi r25, 0x01 ; r25 = 1 
   98: 88 b1 in r24, 0x08  ; r24 = PORTC       //from here PORTC ^= (1 << PC0); 
   9a: 89 27 eor r24, r25  ; r24 = r24 ^ 1 
   9c: 88 b9 out 0x08, r24 ; PORTC = r24 

   9e: 2f e9 ldi r18, 0x9F  ; 159             //from here _delay_ms(): 
   a0: 36 e8 ldi r19, 0x86  ; 134 
   a2: 81 e0 ldi r24, 0x01  ; 1 
   a4: 21 50 subi r18, 0x01 ; 1 
   a6: 30 40 sbci r19, 0x00 ; 0 
   a8: 80 40 sbci r24, 0x00 ; 0 
   aa: e1 f7 brne .-8       ; 0xa4 <main+0x10> 
   ac: 00 c0 rjmp .+0       ; 0xae <main+0x1a> 
   ae: 00 00 nop b0: f3 cf rjmp .-26 ; 0x98 <main+0x4> //jumps back to the loop (98)


```

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

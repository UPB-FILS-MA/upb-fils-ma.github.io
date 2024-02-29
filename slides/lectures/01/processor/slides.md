---
layout: section
---
# What is a microprocessor?

---
layout: two-cols
---
# Microprocessor
controls hardware

- low operating frequency (MHz)
- a lot of I/O ports
- runs simple software
- controls hardware
- does not require an Operating System

<img src="/processor/pico.jpg" class="m-10 h-30">


:: right ::

# Computer
computes

- high operating frequency (GHz)
- limited number of I/O ports
- performs a lot of computations
- usually requires an Operating System

<img src="/processor/pi5.jpg" class="m-10 h-50">

---
# How a microprocessor (MCU) works
This is a simple processor

![Processor](/processor/processor.svg)

---

# 8 bit processor
a simple 8 bit processor with a text display

![8 Bit Processor](/processor/8-bit-processor.svg)

---
layout: two-cols
---
# Programming 
in Rust

![8 Bit Processor](/processor/8-bit-processor.svg)

<v-click>

```rust
use eight_bit_processor::print;

static hello: &str = "Hello World!";

#[start]
fn start() {
    print(hello);
}
```

</v-click>

:: right ::

## MCU's Language - Assembly

<v-click>

```asm
	JMP start
hello: DB "Hello World!" ; Variable
       DB 0	; String terminator
start:
	MOV C, hello    ; Point to var 
	MOV D, 232	; Point to output
	CALL print
        HLT             ; Stop execution
print:			; print(C:*from, D:*to)
	PUSH A
	PUSH B
	MOV B, 0
.loop:
	MOV A, [C]	; Get char from var
	MOV [D], A	; Write to output
	INC C
	INC D  
	CMP B, [C]	; Check if end
	JNZ .loop	; jump if not

	POP B
	POP A
	RET
```

</v-click>

---
layout: fact
---
# Demo
a working example for the previous code

[Start](https://schweigi.github.io/assembler-simulator/)
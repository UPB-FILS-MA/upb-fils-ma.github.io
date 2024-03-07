---
layout: section
---
# MMIO
Memory Mapped Input Output

---
---
# 8 bit processor
a simple 8 bit processor with a text display

![8 Bit Processor](/mmio/8-bit-processor.svg)
---
layout: two-cols
---
# STM32L0x2
A real MCU

| | |
|-|-|
| Cortex-M0+ Peripherals | MCU's *settings* and internal peripherals, available at the same address on all M0+ |
| Peripherals | GPIO, UART, SPI, I2C, US, etc |
| Flash | The storage space |
| SRAM | RAM memory |
| @0x0000_0000 | Alias for SRAM or Flash |

::right::

![Exceptions](/cortex-m0-plus/stm32_mmio.png)

---
layout: two-cols
---
# System Control Registers
@0xe000_0000

Compute the actual address 
- 0xe000_0000 + Offset

Examples:
- SYST_CSR: **0xe000_e010** (*0xe000_0000 + 0xe010*)
- CPUID: **0xe000_ed04** (*0xe000_0000 + 0xed04*)


```rust{all|1|3-4|6|7-9}
use core::ptr::read_volatile;
    
const SYS_CTRL: usize = 0xe000_0000;
const CPUID: usize = 0xed04;

let cpuid_reg = (SYS_CTRL + CPUID) as *const u32;
unsafe {
    read_volatile(cpuid_reg)
}
```

::right::

![SysCtrl Registers](/mmio/sysctrl_registers.png)

---
layout: two-cols
---
# Read the CPUID
About the MCU

```rust{all|1|3-4|6|7-9|11|12|13|14|15}
use core::ptr::read_volatile;

const SYS_CTRL: usize = 0xe000_0000;
const CPUID: usize = 0xed04;

let cpuid_reg = (SYS_CTRL + CPUID) as *const u32;
let cpuid_value = unsafe {
    read_volatile(cpuid_reg)
};

let variant = (cpuid_value >> 24) & 0b1111_1111;
let architecture = (cpuid_value >> 16) & 0b1111;
let part_no = (cpuid_value >> 4) & 0b11_1111_1111;
let revision = (cpuid_value >> 0) & 0b1111;
// use the values
```

::right::

## CPUID Register
Offset: 0xed04

![CPUID Register](/mmio/cpuid_register.png)

---
layout: two-cols
---
# AIRCR
Application Interrupt and Reset Control Register

```rust{all|1,2|4,5|10-13|8,17|7,15|7,16|19-21}
use core::ptr::read_volatile;
use core::ptr::write_volatile;

const SYS_CTRL: usize = 0xe000_0000;
const AIRCR: usize = 0xed0c;

const VECTKEY: u32 = 16;
const SYSRESETREQ: u32 = 2;

let aircr_register = (SYS_CTRL + AIRCR) as *mut u32;
let mut aircr_value = unsafe { 
    read_volatile(aircr_register) 
};

aircr_value = aircr_value & (0x0000 << VECTKEY); 
aircr_value = aircr_value | (0x05fa << VECTKEY);
aircr_value = aircr_value | (1 << SYSRESETREQ);

unsafe {
    write_volatile(aircr_register, aircr_value);
}
```

::right::

## AIRCR Register

Offset: 0xed0c

![AIRCR Register 1](/mmio/aircr_register_1.png)
![AIRCR Register 2](/mmio/aircr_register_2.png)

---
layout: two-cols
---
# Reads and Writes
they do stuff

- Read
  - reads the value of a register
  - might ask the peripheral to do something
  
- Write
  - reads the value to a register
  - might ask the peripheral to do something
    - SYSRESETREQ

::right::

## AIRCR Register

Offset: 0xed0c

![AIRCR Register 1](/mmio/aircr_register_1.png)
![AIRCR Register 2](/mmio/aircr_register_2.png)

---
---
# SVD XML File
System View Description

```xml{all|3|4,21|4,5,21|4,6,21|4,7-9,20,21|4,7-8,12-17,20,21}
<device schemaVersion="1.1"
	xmlns:xs="http://www.w3.org/2001/XMLSchema-instance" xs:noNamespaceSchemaLocation="CMSIS-SVD.xsd">
	<name>RP2040</name>
	<peripherals>
		<name>PPB</name>
		<baseAddress>0xe0000000</baseAddress>
		<register>
			<name>CPUID</name>
			<addressOffset>0xed00</addressOffset>
			<resetValue>0x410cc601</resetValue>
			<fields>
				<field>
					<name>IMPLEMENTER</name>
					<description>Implementor code: 0x41 = ARM</description>
					<bitRange>[31:24]</bitRange>
					<access>read-only</access>
				</field>
				<!-- rest of the fields of the register -->
			</fields>
		</register>
	</peripherals>
</device>
```

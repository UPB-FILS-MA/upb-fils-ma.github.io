---
layout: section
---
# Memory Management
MMU

---
layout: two-cols
---
# Memory Management
memory access defined page by page

- uses *logical addresses*
- **translates** to *physical addresses*

The processor works in at least two modes:
- **supervisor** mode 
  - restricts access to some registers 
  - accesses virtual addresses through Memory Protection (*if machine mode exists*)
- **user** mode 
  - allows only ALU and memory load ans store
  - accesses memory access through the Memory Management Unit (*MMU*)

:: right ::

<img src="/mmu/mmu.svg" class="w-120">

---
layout: two-cols
---
# Paging
the memory *unit* is the page

- Physical Memory (*RAM*) is divided in **frames**
- Logical Memory is divided in **pages**
- *page* = *frame* = **4 KB** (usually)

*logical addresses* are translated to *physical addresses* using a **page table**

the **page table** is located in the **physical memory**
  - <span color="red">each memory access requires at least memory 2 accesses</span>

<style>
.two-columns {
    grid-template-columns: 3fr 6fr;
}
</style>

:: right ::

<img src="/mmu/paging.svg" class="w-170">

---
---
# Address Translation
page to frame

<div align="center">
    <img src="/mmu/page_translation.svg" class="w-170">
</div>

---
---
# Translation Lookaside Buffer
caching address translation

<div align="center">
    <img src="/mmu/tlb.svg" class="w-170">
</div>

---
---
# Page Table Entry 
for x86 - 32 bits

this is one entry of the page table
- **P** - is the page's frame present in RAM?
- **R/W** - read only or read write access
- **U/S** - can the page be accessed in user mode?
- **D** and **A** - has this page been written since the OS has reset these bits?
- **AVL** - bits available for the OS to use, ignored by MMU

<div align="center">
    <img src="/mmu/page_entry_x86.svg">
</div>

---
---
# Page Table Entry
for x86 - 32 bits with PAE

this is one entry of the page table using Physical Address Extension (*PAE*)
- **XD** - eXecute Disable (aka *DEP*), if set triggers a fault if an instruction is read from the page
- **PK** - Protection Keys, allows user mode to set protection (64 bit only)

<div align="center">
    <img src="/mmu/page_entry_x86pae.svg">
</div>

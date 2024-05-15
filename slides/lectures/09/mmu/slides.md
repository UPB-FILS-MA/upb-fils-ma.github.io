---
layout: section
---
# Memory Management
MMU

---
---
# Bibliography
for this section

1. **Andrew Tanenbaum**, *Modern Operating Systems (4th edition)*
   - Chapter 3 - *Memory Management*
     - Subchapter 3.3 - *Virtual Memory*

2. **Philipp Oppermann**, [*Writing an OS in Rust*](https://os.phil-opp.com)
   - [*Introduction to Paging*](https://os.phil-opp.com/paging-introduction/) 
   - [*Paging Implementation*](https://os.phil-opp.com/paging-implementation/)

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
layout: two-cols
---
# Address Translation
page to frame

the logic address is divided in two parts:
- *page index*
- *offset* withing the page

the MMU translates every logic address into a physical address using a *page table*

<style>
.two-columns {
    grid-template-columns: 2fr 6fr;
}
</style>

:: right ::

<div align="center">
    <img src="/mmu/page_translation.svg" class="w-170">
</div>

---
layout: two-cols
---
# Translation Lookaside Buffer (TLB)
caching address translation

<style>
.two-columns {
    grid-template-columns: 2fr 6fr;
}
</style>

the **page table** is **stored in RAM**

each memory access **requires 2 accesses**
1. read the page table entry to translate the address
2. the requested access

:: right ::

<div align="center">
    <img src="/mmu/tlb.svg" class="w-170 rounded">
</div>

---
layout: two-cols
---

# Page Directory
caching address translation

<style>
.two-columns {
    grid-template-columns: 2fr 6fr;
}
</style>

$$ size_{table} = \frac{size_{ram}}{size_{page}}  $$

- each table entry is 4B
- the address space is 4GB (for 32 bits processors)

$$ size_{table\_32\_bits} = \frac{2^{32}}{4 \times 2^{10}}  $$

$$ size_{table\_32\_bits} = 4 MB $$

RAM was counted in MB when paging started being used

:: right ::

<div align="center">
    <img src="/mmu/page_directory.svg" class="w-170">
</div>

<div align="center">
two levels, page directory and table, usually used for 32 bits systems
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

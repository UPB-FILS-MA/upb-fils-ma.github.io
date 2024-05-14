---
layout: section
---
# Tock OS
An embedded operating system designed for running multiple concurrent, mutually distrustful applications on low-memory and low-power microcontrollers.

---
---
# Tock OS
an embedded operating systems that works like a desktop or server one

<div grid="~ cols-3 gap-5">

<div style="grid-column-start: 1; grid-column-end: 3;">

- A **preemptive** embedded OS (runs on MCUs)
  - Cortex-M
  - RISC-V
- Uses memory protection (**MPU required**)
- Has separate **kernel and user space**
  - most embedded OS have the one piece software philosophy
- Runs untrusted apps in user space
- **Microkernel** architecture
- Kernel (and drivers) written in Rust
- Apps written in C/C++ or Rust (any language that can be compiled)

</div>

<img src="/tock/tock.svg" class="rounded" />

</div>

---
---
# Tock System Calls

0. Yield
1. Subscribe
2. Command
3. ReadWriteAllow
4. ReadOnlyAllow
5. Memop
6. Exit

---

# 0: Yield

Yield transitions the current process from the Running to the Yielded state.

```rust
// waits for the next upcall
// The process will not execute again until another upcall re-schedules the
// process.
yield()

// does not wait for the next upcall
// If a process has no enqueued upcalls, the
// process immediately re-enters the Running state.
yield_no_wait()
```

## Return

*yield*: None 

*yield_no_wait*:
  - 1 - *upcall* ran
  - 0 - there was no queued *upcall* function to execute

---

# 1: Subscribe

Subscribe assigns upcall functions to be executed in response to various
events.

```rust {lines: false}
subscribe(driver: u32, subscribe_number: u32, upcall: u32, userdata: u32) -> Result<Upcall, (Upcall, ErrorCode)>
```

<div grid="~ cols-2 gap-3">

<div>

**Arguments**

 - `driver`: integer specifying which driver to call
 - `subscribe_number`: event number
 - `upcall`: function's pointer to call upon event
```c {lines: false}
void upcall(int arg1, int arg2, int arg3, void* userdata)
```
 - `userdata`: value that will be passed back, usually a pointer

</div>

<div>

**Return**

- The previously registered upcall or `TOCK_NULL_UPCALL`
- Errors
   - `NODEVICE` if `driver` does not refer to a valid kernel driver.
   - `NOSUPPORT` if the driver exists but doesn't support the `subscribe_number`.

</div>
</div>

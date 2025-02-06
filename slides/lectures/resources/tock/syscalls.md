<!--

---

### 2: Command

Command instructs the driver to perform a specific action.

```rust
command(driver: u32, command_number: u32, argument1: u32, argument2: u32) -> CommandReturn
```

#### Arguments

 - `driver`: An integer specifying which driver to call.
 - `command_number`: An integer specifying the requested command.
 - `argument1`: A command-specific argument.
 - `argument2`: A command-specific argument.

- One Tock convention with the Command syscall is that command number 0 will
always return a value of 0 or greater if the driver is supported.

#### Return
- 3 usize numbers
- Errors
 - `NODEVICE` if `driver` does not refer to a valid kernel driver.
 - `NOSUPPORT` if the driver exists but doesn't support the `command_number`.
 - Other return codes based on the specific driver.

---

### 3 and 4: AllowRead(Write/Only)

Allow shares memory buffers between the kernel and application.
- null pointer revokes sharing a region.

```rust
allow_readwrite(driver: u32, allow_number: u32, pointer: usize, size: u32) -> Result<ReadWriteAppSlice, (ReadWriteAppSlice, ErrorCode)>
allow_readonly(driver: u32, allow_number: u32, pointer: usize, size: u32) -> Result<ReadWriteAppSlice, (ReadWriteAppSlice, ErrorCode)>
```

#### Arguments

 - `driver`: An integer specifying which driver should be granted access.
 - `allow_number`: A driver-specific integer specifying the purpose of this
   buffer.
 - `pointer`: A pointer to the start of the buffer in the process memory space.
 - `size`: An integer number of bytes specifying the length of the buffer.

#### Return
- The previous allowed pointer or NULL
- Errors
 - `NODEVICE` if `driver` does not refer to a valid kernel driver.
 - `NOSUPPORT` if the driver exists but doesn't support the `allow_number`.
 - `INVAL` the buffer referred to by `pointer` and `size` lies completely or
partially outside of the processes addressable RAM.
 - Other return codes based on the specific driver.

---

### 5: Memop

Memop expands the memory segment available to the process, allows the process to
retrieve pointers to its allocated memory space, provides a mechanism for
the process to tell the kernel where its stack and heap start, and other
operations involving process memory.

```rust
memop(op_type: u32, argument: u32) -> [[ VARIES ]] as u32
```

#### Arguments

 - `op_type`: An integer indicating whether this is a `brk` (0), a `sbrk` (1),
   or another memop call.
 - `argument`: The argument to `brk`, `sbrk`, or other call.

Each memop operation is specific and details of each call can be found in
the [memop syscall documentation](syscalls/memop.md).

#### Return

- Dependent on the particular memop call.

---

### 6: Exit

The process signals the kernel that it has no more work to do and can be stopped or that it asks
the kernel to restart it.

```rust
tock_exit(completion_code: u32)
tock_restart(completion_code: u32)
```

##### Return

None 

---
## Workpoint 8 .top_image[![Work In Progress](../images/work_in_progress.png)]

Enable the system calls trace in `kernel/src/config.rs`. 
1. Upload an app that prints a text on the shell using `printf`. 
  - How many system calls is printf using?
  - What is the driver number of the driver used for printing?

2. Upload the blink app and look at the system calls trace.
  - What is the leds matrix's driver number?

---

-->
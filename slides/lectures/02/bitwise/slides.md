---
layout: section
---
# Bitwise Ops
How to set and clear bits

---
---
# Set bit
set the `1` on position `bit` of `register`

```rust{all|2|3|4|1,5,6}
fn set_bit(register: usize, bit: u8) -> usize {
    // assume register is 0b1000, bit is 3
    //   1 << 3 is 0b0100
    //   0b1000 | 0b0100 is 0b1100
    register | 1 << bit   
}
```

###  Set multiple bits

```rust
fn set_bits(register: usize, bits: usize) -> usize {
    // assume register is 0b1000, bits is 0b0111
    //   0b1000 | 0b0111 is 0b1111
    register | bits   
}
```

---
---
# Clear bit
set the `0` on position `bit` of `register`

```rust{all|2|3|4|4,5|1,6,7}
fn clear_bit(register: usize, bit: u8) -> usize {
    // assume register is 0b1100, bit is 3
    //   1 << 3 is 0b0100
    //   !(1 << 3) is 0b1011
    //   0b1100 & 0b1011 is 0b1000
    register & !(1 << bit)
}
```

###  Clear multiple bits

```rust
fn clear_bits(register: usize, bits: usize) -> usize {
    // assume register is 0b1111, bits is 0b0111
    //   ~bits = 0b1000
    //   0b1111 & 0b1000 is 0b1000
    register & !bits   
}
```

---
---
# Flip bit
flip the bit on position `bit` of `register`

```rust{all|2|3|4|1,5,6}
fn flip_bit(register: usize, bit: u8) -> usize {
    // assume register is 0b1000, bit is 3
    //   1 << 3 is 0b0100
    //   0b1100 ^ 0b0100 is 0b1000
    register ^ 1 << bit
}
```

###  Flip multiple bits

```rust
fn flip_bits(register: usize, bits: usize) -> usize {
    // assume register is 0b1000, bits is 0b0111
    //   0b1000 ^ 0b0111 is 0b1111
    register ^ bits   
}
```
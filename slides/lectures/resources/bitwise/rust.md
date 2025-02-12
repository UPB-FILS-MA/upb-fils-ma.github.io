---
layout: section
---
# Bitwise Ops
How to set and clear bits

---

# Set bit
## set the `1` on position `bit` of `register`

```rust {all|2|3|4|1,5,6}
fn set_bit(register: usize, bit: u8) -> usize {
    // assume register is 0b1000, bit is 2
    //   1 << 2 is 0b0100
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

# Clear bit
### Set the `0` on position `bit` of `register`

```rust {all|2|3|4|4,5|1,6,7}
fn clear_bit(register: usize, bit: u8) -> usize {
    // assume register is 0b1100, bit is 2
    //   1 << 2 is 0b0100
    //   !(1 << 3) is 0b1011
    //   0b1100 & 0b1011 is 0b1000
    register & !(1 << bit)
}
```

###  Clear multiple bits

```rust
fn clear_bits(register: usize, bits: usize) -> usize {
    // assume register is 0b1111, bits is 0b0111
    //   !bits = 0b1000
    //   0b1111 & 0b1000 is 0b1000
    register & !bits   
}
```

---

# Flip bit

### Flip the bit on position `bit` of `register`

```rust {all|2|3|4|1,5,6}
fn flip_bit(register: usize, bit: u8) -> usize {
    // assume register is 0b1000, bit is 2
    //   1 << 2 is 0b0100
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

---

# Let's see a combined operation for value extraction

- We presume an 32 bits ID = `0b1100_1010_1111_1100_0000_1111_0110_1101`
- And want to extract a portion  <font face="PT Mono">0b<b>1100_1010_1111</b>_1100_0000_1111_0110_1101</font>

```rust {1|8|1,8,9|3-5,12-14|all}
const MASK: u32 = 0b0000_0000_0000_0000_0000_1111_1111_1111;

fn print_binary(label: &str, num: u32) {
    println!("{}: {:032b}", label, num);
}

fn main() {
    let large_id: u32 = 0b1100_1010_1111_1100_0000_1111_0110_1101;
    let extracted_bits = (large_id >> 20) & MASK;
 
   // Print values in binary
    print_binary("Original_", large_id);
    print_binary("Mask_____", MASK);
    print_binary("Extracted", extracted_bits);
}
/* RESULT
Original_: 11001010111111000000111101101101
Mask_____: 00000000000000000000111111111111
Extracted: 00000000000000000000110010101111 */

```

---

# With nice formating

```rust {2-13|3|3,4|4-10|5-9|3,4,11|,3,4,11,12|all}
const MASK: u32 = 0b0000_0000_0000_0000_0000_1111_1111_1111;
fn format_binary(num: u32) -> String {
    (0..32).rev()
        .map(|i| {
            if i != 0 && i % 4 == 0 {
                format!("{}_", (num >> i) & 1)
            } else {
                format!("{}", (num >> i) & 1)
            }
        })
        .collect::<Vec<_>>()
        .join("")
}
fn print_binary(label: &str, num: u32) { println!("{}: {}", label, format_binary(num));}
fn main() {
    let large_id: u32 = 0b1100_1010_1111_1100_0000_1111_0110_1101;
    let extracted_bits = (large_id >> 20) & MASK;
    print_binary("Original_", large_id);
    print_binary("Extracted", extracted_bits);
}
/* RESULTS:
Original_: 1100_1010_1111_1100_0000_1111_0110_1101
Extracted: 0000_0000_0000_0000_0000_1100_1010_1111 */
```

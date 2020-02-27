# Rust 34C04 EEPROM Driver

This is a platform agnostic Rust driver for the 34c04 series serial EEPROM,
based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Read a single byte from a memory address.
- Read a byte array starting on a memory address.

- Write a byte to a memory address.
- Write a byte array (up to a memory page) to a memory address.

## The devices

These devices provides a number of bits of serial electrically erasable and
programmable read only memory (EEPROM) organized as a number of words of 8 bits
each. The devices' cascadable feature allows up to 8 devices to share a common
2-wire bus. The devices are optimized for use in many industrial and commercial
applications where low power and low voltage operation are essential.

|Device | Memory bits | 8-bit words | Page size |
|-------:|------------:|------------:|----------:|
|  34c04 |      4 Kbit |         512 |  16 bytes |

## Usage examples

```rust
let address = eeprom34c04::SlaveAddr::A2A1A0(false, true, true);

let mut eeprom = eeprom34c04::Eeprom34c04::new_34c04(i2c_setup, address);

let memory_address = 0x0F;

let data = 0xF0;

eeprom.write_byte(memory_address, data).unwrap();

delay.delay_ms(5u16);

let read_data = eeprom.read_byte(memory_address).unwrap();
```

# Inspired

These drivers are heavily influenced by this work:
https://github.com/eldruin/eeprom24x-rs.git

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.
   

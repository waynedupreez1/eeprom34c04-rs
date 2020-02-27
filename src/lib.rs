//! This is a platform agnostic Rust driver for the 34c04 series serial EEPROM,
//! based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Read a single byte from a memory address.
//! - Read a byte array starting on a memory address.

//! - Write a byte to a memory address. See: [`write_byte()`].
//! - Write a byte array (up to a memory page) to a memory address.
//!
//! ## The devices
//!
//! These devices provides a number of bits of serial electrically erasable and
//! programmable read only memory (EEPROM) organized as a number of words of 8 bits
//! each. The devices' cascadable feature allows up to 8 devices to share a common
//! 2-wire bus. The devices are optimized for use in many industrial and commercial
//! applications where low power and low voltage operation are essential.
//!
//! | Device | Memory bits | 8-bit words | Page size |
//! |-------:|------------:|------------:|----------:|
//! |  34c04 |      4 Kbit |         512 |  16 bytes |
//!
//! ## Usage examples (see also examples folder)
//! 
//! let address = eeprom34c04::SlaveAddr::A2A1A0(false, true, true);
//! let mut eeprom = eeprom34c04::Eeprom34c04::new_34c04(i2c_setup, address);
//! let memory_address = 0x0F;
//! let data = 0xF0;
//!
//! eeprom.write_byte(memory_address, data).unwrap();
//!
//! delay.delay_ms(5u16);
//! 
//! let read_data = eeprom.read_byte(memory_address).unwrap();

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs, unsafe_code)]

extern crate embedded_hal as hal;

mod error;

/// Possible slave addresses
/// This is the setting for A2 A1 A0
/// 
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddr {
    /// fdfg
    A2A1A0(bool, bool, bool)
}

/// EEPROM34c04 struct
/// This defines the struct holding the eeprom instance data
/// 
/// i2c = The concrete I²C device implementation.
/// address = The I²C device address A2 A1 A0
/// rw_func_bits = 0x1010A2A1A0 used for read and write operations
/// last_addr_w = last address written to
/// last_addr_r = last address read from
/// 
#[derive(Debug)]
pub struct Eeprom34c04<I2C> {
    i2c: I2C,
    rw_func_bits: u8,
    last_addr_w: u32,
    last_addr_r: u32,
}

mod eeprom34c04;

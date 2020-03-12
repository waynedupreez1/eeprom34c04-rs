/// Error Types for library
/// 
/// Wayne du Preez
/// 

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Too much data passed for a write
    TooMuchData,
    /// Memory address is out of range
    InvalidAddr,
    /// Quad Convertion Failed
    InvalidAddrConvert,
    /// Invalid data array multiple
    InvalidDataArrayMultiple,
}
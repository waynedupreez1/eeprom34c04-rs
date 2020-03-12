use hal::blocking::i2c::{Write, WriteRead, Read};
use Eeprom34c04;
use SlaveAddr;
use super::error;

//Constants

const RW_FUNC_BITS: u8 = 0b1010000;

// Page Address Functions bits:)
const PA_FUNC_BITS: u8 = 0b0110110;


impl<I2C, E> Eeprom34c04<I2C> 
    where I2C: Write<Error = E> + WriteRead<Error = E>,  
    {
    /// Create a new instance of a 34c00 device
    pub fn new_34c04(i2c: I2C, address: SlaveAddr) -> Self {      
        //Converts adress bits and ors to read_write function
        let rw_func_bits = match address {
            SlaveAddr::A2A1A0(a2, a1, a0) => {
                RW_FUNC_BITS | ((a2 as u8) << 2) | ((a1 as u8) << 1) | a0 as u8
            }
        };    
        
        Eeprom34c04 {
            i2c: i2c,
            rw_func_bits: rw_func_bits,
            last_addr_w: 0,
            last_addr_r: 0,           
        }
    }
}


/// Common methods
impl<I2C> Eeprom34c04<I2C> {
    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

/// Common methods
impl<I2C, E> Eeprom34c04<I2C>
    where I2C: Write<Error = E> + WriteRead<Error = E> + Read<Error = E> {
    /// Write a single byte in an address.
    ///
    /// After writing a byte, the EEPROM enters an internally-timed write cycle
    /// to the nonvolatile memory.
    /// During this time all inputs are disabled and the EEPROM will not
    /// respond until the write is complete.
    pub fn write_byte(&mut self, address: u32, data: u8) -> Result<(), error::Error<E>> {
        
        addr_in_bounds(address)?;
        let (page_addr, mem_addr) = addr_convert(address)?;

        self.last_addr_w = address;

        let spa_dont_care = [0; 2];
        self.i2c.write(page_addr, &spa_dont_care).map_err(error::Error::I2C)?;        
        
        let array = [mem_addr, data];
        self.i2c.write(self.rw_func_bits, &array).map_err(error::Error::I2C)?;

        Ok(())
    }

    /// Read a single byte from an address.
    pub fn read_byte(&mut self, address: u32) -> Result<u8, error::Error<E>> {

        addr_in_bounds(address)?;
        let (page_addr, mem_addr) = addr_convert(address)?;

        self.last_addr_r = address;

        let spa_dont_care = [0; 2];
        self.i2c.write(page_addr, &spa_dont_care).map_err(error::Error::I2C)?;
               
        let memaddr = [mem_addr];
        let mut data = [0; 1];

        self.i2c.write_read(self.rw_func_bits, &memaddr, &mut data).map_err(error::Error::I2C).and(Ok(data[0]))
    }

    /// Read a multiple bytes from an address.
    /// 
    /// 
    pub fn read_byte_array(&mut self, address: u32, data: &mut [u8]) -> Result<(), error::Error<E>> {

        addr_in_bounds(address)?;

        addr_in_bounds_page_wr(address, data.len() as u32)?;

        let (page_addr, mem_addr) = addr_convert(address)?;

        self.last_addr_r = address;

        let spa_dont_care = [0; 2];
        self.i2c.write(page_addr, &spa_dont_care).map_err(error::Error::I2C)?;
                      
        let memaddr = [mem_addr];

        //Dummy read write else the sequential 
        //reading only reads the first value correctly
        let mut dummy_data = [0; 1];
        self.i2c.write_read(self.rw_func_bits, &memaddr, &mut dummy_data).map_err(error::Error::I2C)?;


        self.i2c.write_read(self.rw_func_bits, &memaddr, data).map_err(error::Error::I2C)
    }

    /// Write multiple bytes to address.
    /// 
    /// Maximum allowed data to be written to eeprom in 1 go is 16 bytes
    /// 
    /// The function will allow the following byte array sizes to be passed
    /// 1. 2 bytes
    /// 2. 4 bytes
    /// 3. 8 bytes
    /// 4. 16 bytes
    /// If you pass anything else the InvalidDataArrayMultiple will be returned
    /// 
    pub fn write_byte_array(&mut self, address: u32, data_array: &[u8]) -> Result<(), error::Error<E>> {

        //Only allowed up to 16 bytes to be written
        if data_array.len() > 16 { return Err(error::Error::TooMuchData) };
        
        addr_in_bounds(address)?;

        addr_in_bounds_page_wr(address, data_array.len() as u32)?;

        let (page_addr, mem_addr) = addr_convert(address)?;

        self.last_addr_w = address;

        let spa_dont_care = [0; 2];
        self.i2c.write(page_addr, &spa_dont_care).map_err(error::Error::I2C)?;

        
        match data_array.len() {
            2 => {
                let array = [mem_addr, data_array[0], data_array[1] ];
                self.i2c.write(self.rw_func_bits, &array).map_err(error::Error::I2C)?;
            }

            4 => {
                let array = [mem_addr, data_array[0], data_array[1], data_array[2], data_array[3] ];
                self.i2c.write(self.rw_func_bits, &array).map_err(error::Error::I2C)?;
            }

            8 => {
                let array = [mem_addr, data_array[0], data_array[1], data_array[2], data_array[3], 
                                        data_array[4], data_array[5], data_array[6], data_array[7] ];
                self.i2c.write(self.rw_func_bits, &array).map_err(error::Error::I2C)?;
            }

            16 => {
                let array = [mem_addr, data_array[0],  data_array[1], data_array[2], data_array[3], 
                                        data_array[4], data_array[5], data_array[6], data_array[7],
                                        data_array[8], data_array[9], data_array[10],data_array[11],
                                        data_array[12],data_array[13],data_array[14],data_array[15] ];
                self.i2c.write(self.rw_func_bits, &array).map_err(error::Error::I2C)?;
            }

            _ => { return Err(error::Error::InvalidDataArrayMultiple) }
        }
        
        Ok(())
    }
    

    /// Previously read address
    pub fn previous_read_addr(&self) -> u32 {

        self.last_addr_r
    }

    /// Previously read address
    pub fn previous_write_addr(&self) -> u32 {

        self.last_addr_w
    }
}

//Private

/// When doing multi byte reads and writes we have to ensure we
/// are far away from the ends of the particular memory quad
/// we are operating in
/// 
fn addr_in_bounds_page_wr<E>(address: u32, data_size: u32) -> Result<(), error::Error<E>> {

    let (page_addr, mem_addr) = addr_convert(address)?;

    //If we are in memory quad 0 or 2 then the adress can be a max value of 0x7F
    if (mem_addr >> 7) == 0 {  
        
        if (mem_addr as u32 + data_size) <= 0x7F { return Ok(()) }
        else { return Err(error::Error::TooMuchData) };
    };

    //If we are in memory quad 1 or 3 then the adress can be a max value of 0xFF
    if (mem_addr as u32 + data_size) <= 0xFF { return Ok(()) }
    else { return Err(error::Error::TooMuchData) }
}
 
/// Check if the adress requested is in bounds
/// The maximum adress can be 1FF = 511 = 0000 0001 1111 1111 
/// for this 512 byte eeprom
/// 
fn addr_in_bounds<E>(address: u32) -> Result<(), error::Error<E>> {
    let val = address >> 9;

    if val == 0 {Ok(())}
    else {Err(error::Error::InvalidAddr)}
}

/// This converts the adress as given by a 16 bit value decribed in address_in_bounds
/// to the appropriate memory quadrant 0/1 (page_address 0 ), or 2/3 (page_address 1)
/// tuple.0 = page address
/// tuple.1 = memory adress ranging from 0 - 255
/// 
/// Lower memory
/// Quadrant 0 can save bytes 0   - 127
/// Quadrant 1 can save bytes 128 - 255
/// 
/// Upper memory
/// Quadrant 2 can save bytes 0   - 127
/// Quadrant 3 can save bytes 128 - 255
/// 
fn addr_convert<E>(address: u32) -> Result<(u8, u8), error::Error<E>> {
    
    //In quad 0 
    if (address >> 7) == 0 {        
        return Ok((PA_FUNC_BITS, address as u8))
    };

    //In quad 1 
    if (address >> 8) == 0 {        
        return Ok((PA_FUNC_BITS, address as u8))
    };

    //In quad 2
    //Mask the top bit and rotate
    let new_addr = address & 0b011111111;
    
    if (new_addr >> 7) == 0 {        
        return Ok((PA_FUNC_BITS | 1, new_addr as u8))
    };


    //In quad 3
    let new_addr = address & 0b011111111;
    
    if (new_addr >> 8) == 0 {        
        return Ok((PA_FUNC_BITS | 1, new_addr as u8))
    };

    Err(error::Error::InvalidAddrConvert)
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn call_address_in_bounds_with_condition_address_equal_0x100_result_should_pass() {

        let addr = 0x100;

        let result = addr_in_bounds::<error::Error<u8>>(addr).is_ok();

        assert_eq!(result, true);
    }

    #[test]
    fn call_address_in_bounds_with_condition_address_equal_0x200_result_should_fail() {

        let addr = 0x200;

        let result = addr_in_bounds::<error::Error<u8>>(addr).is_err();

        assert_eq!(result, true);
    }

    #[test]
    fn call_address_convert_with_condition_address_equal_0x7F_result_tuple_0_is_PA_FUNC_BITS_tuple_1_is_0x7F() {

        let addr = 0x7F;
        let quad = PA_FUNC_BITS;

        let result = addr_convert::<error::Error<u8>>(addr).unwrap();

        println!("{:?}", result);       
        
        assert_eq!(result.0, quad);
        assert_eq!(result.1, addr as u8);
    }

    #[test]
    fn call_address_convert_with_condition_address_equal_0xFF_result_tuple_0_is_PA_FUNC_BITS_tuple_1_is_0xFF() {

        let addr = 0xFF;
        let quad = PA_FUNC_BITS;

        let result = addr_convert::<error::Error<u8>>(addr).unwrap();

        println!("{:?}", result);       
        
        assert_eq!(result.0, quad);
        assert_eq!(result.1, addr as u8);
    }

    #[test]
    fn call_address_convert_with_condition_address_equal_0x17F_result_tuple_0_is_PA_FUNC_BITS_ored_1_tuple_1_is_0x7F() {

        let addr = 0x17F;
        let quad = PA_FUNC_BITS | 1;

        let result = addr_convert::<error::Error<u8>>(addr).unwrap();

        println!("{:?}", result);       
        
        assert_eq!(result.0, quad);
        assert_eq!(result.1, addr as u8);
    }

    #[test]
    fn call_address_convert_with_condition_address_equal_0x1FF_result_tuple_0_is_PA_FUNC_BITS_ored_1_tuple_1_is_0xFF() {

        let addr = 0x1FF;
        let quad = PA_FUNC_BITS | 1;

        let result = addr_convert::<error::Error<u8>>(addr).unwrap();

        println!("{:?}", result);       
        
        assert_eq!(result.0, quad);
        assert_eq!(result.1, addr as u8);
    }

    #[test]
    fn call_addr_in_bounds_page_wr_with_condition_address_0x7F_add_8_result_error() {

        let quad0_addr_max = 0x7F;
        let addr = quad0_addr_max; 

        let data_len = 8u32;

        let result = addr_in_bounds_page_wr::<u8>(addr, data_len).is_err();
       
        println!("{:?}", result);
        
        assert_eq!(result, true);
    }

    #[test]
    fn call_addr_in_bounds_page_wr_with_condition_address_0xFF_add_8_result_error() {

        let quad0_addr_max = 0xFF;
        let addr = quad0_addr_max; 

        let data_len = 8u32;

        let result = addr_in_bounds_page_wr::<u8>(addr, data_len).is_err();
       
        println!("{:?}", result);
        
        assert_eq!(result, true);
    }

    #[test]
    fn call_addr_in_bounds_page_wr_with_condition_address_0x77_add_8_result_no_error() {

        let quad0_addr_max = 0x77;
        let addr = quad0_addr_max; 

        let data_len = 8u32;

        let result = addr_in_bounds_page_wr::<u8>(addr, data_len).is_ok();
       
        println!("{:?}", result);
        
        assert_eq!(result, true);
    }
}

# CRC Zoo

This crate provides a collection of Cyclic Redundancy Check (CRC) algorithms, including CRC5, CRC8, CRC16 and CRC32.

Implementation is generated using https://pycrc.org/ using the bit-by-bit algorithm, which does not 
use a lookup table, and is most suitable for checking small amounts of data.

The Zoo is collected from and verified against https://crccalc.com/

## CRC5
```
crc5()
```

## CRC8
```
crc8()
crc8_cdma2000()
crc8_darc()
crc8_dvb_s2()
crc8_ebu()
crc8_i_code()
crc8_itu()
crc8_maxim()
crc8_rohc()
crc8_wcdma()
```

## CRC16
```
crc16_ccitt_false()
crc16_arc()
crc16_aug_ccitt()
crc16_buypass()
crc16_cdma2000()
crc16_dds_110()
crc16_dect_r()
crc16_dect_x()
crc16_dnp()
crc16_en_13757()
crc16_genibus()
crc16_maxim()
crc16_mcrf4xx()
crc16_riello()
crc16_t10_dif()
crc16_teledisk()
crc16_tms37157()
crc16_usb()
crc16_a()
crc16_kermit()
crc16_modbus()
crc16_x_25()
crc16_xmodem()
```

## CRC32
```
crc32()
crc32_bzip2()
crc32c()
crc32d()
crc32_mpeg2()
crc32_posix()
crc32q()
crc32_jamcrc()
crc32_xfer()
```

# Usage

Calling a predefined function
```rust
use crczoo::{crc8, calculate_crc8};

assert_eq!(crc8(&"123456789".to_owned().into_bytes()), 0xF4);
```

Specifying the polynomial parameters
```rust
pub fn calculate_crc8(data: &[u8], poly: u8, init: u8, ref_in: bool, ref_out: bool, xor_out: u8) -> u8;

assert_eq!(calculate_crc8(&"123456789".to_owned().into_bytes(), 0x07, 0x00, false, false, 0x00), 0xF4);
```

# CRC Explained

Example using CRC to detect errors in byte stream

```rust
use crczoo::crc8;

// first this is the data to protect
let mut data = "123456789".to_owned().into_bytes();
// here we obtain the checksum
let checksum = crc8(&data);
data.push(checksum);
// with the checksum appended to the original byte stream, the CRC should yield 0
assert_eq!(crc8(&data), 0);

// now let's introduce 1 bit error: i.e. '0' = 0x30, '1' = 0x31
let mut data = "023456789".to_owned().into_bytes();
data.push(checksum);
// non zero value means there is error!
assert!(crc8(&data) != 0);

// rinse and repeat, introduce 2 bits error this time
let mut data = "023456799".to_owned().into_bytes();
data.push(checksum);
// non zero value means there is error!
assert!(crc8(&data) != 0);
```

As explained in https://users.ece.cmu.edu/~koopman/crc/ , different polynomials have different error 
detection capability. Normally a good CRC should be able to detect all 1 and 2 bit errors for byte 
stream up to a certain length, and then the error detection capability degrades when more bits are 
flipped and the byte stream becomes longer.

# CRC Parametrization Explained

> From http://www.sunshine2k.de/articles/coding/crc/understanding_crc.html#ch7

Following standard parameters are used to define a CRC algorithm instance:

Name: a name which is used to identify in literature e.g. CRC-8/CDMA2000, CRC-16/CCITT.

Width (in bits): Defines the width of the result CRC value (n bits). Simultaneously, also the 
width of the generator polynomial is defined (n+1 bits). Most common used widths are 8, 16 and 32 
bit. In practice, even quite big (80 bit) or uneven (5 bit or 31 bit) widths are used.

Polynomial: Used generator polynomial value. There are different ways to represent a generator polynomial in hexadecimal, but the most common is to discard the most significant bit as it is always 1.

Initial Value: The value used to initialize the CRC value.

Input reflected: If this value is TRUE, each input byte is reflected before being used in the 
calculation. Reflected means that the bits of the input byte are used in reverse order. So this 
also means that bit 0 is treated as the most significant bit and bit 7 as least significant.

Example with byte 0x82 = b10000010: Reflected(0x82) = Reflected(b10000010) = b01000001 = 0x41.

Result reflected: If this value is TRUE, the final CRC value is reflected before being returned. 
The reflection is done over the whole CRC value, so e.g. a CRC-32 value is reflected over all 32 
bits.

Final XOR value: The Final XOR value is xored to the final CRC value before being returned. This 
is done after the 'Result reflected' step. Obviously a Final XOR value of 0 has no impact.

Check value (Optional): This value is not required but often specified to help validating an 
implementation. This is the CRC value of input string "123456789" or as byte array: 
[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39].

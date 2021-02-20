# CRC Zoo

This crate provides a collection of Cyclic Redundancy Check (CRC) algorithms, including CRC5, CRC8, CRC16 and CRC32.

Implementation is generated using https://pycrc.org/ using the bit-by-bit algorithm.

The Zoo is collected from https://crccalc.com/

CRC5
```
crc5()
```

CRC8
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

CRC16
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

CRC32
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

```rust
use crczoo::crc8;

assert_eq!(crc8(&"123456789".to_owned().into_bytes()), 0xF4);
assert_eq!(calculate_crc8(&"123456789".to_owned().into_bytes(), 0x07, 0x00, false, false, 0x00), 0xF4);
```

# CRC Parametrization Explained

> From http://www.sunshine2k.de/articles/coding/crc/understanding_crc.html#ch7

Following standard parameters are used to define a CRC algorithm instance:

Name: Well, a CRC instance has to be identified somehow, so each public defined CRC parameter set 
has a name like e.g. CRC-16/CCITT.

Width (in bits): Defines the width of the result CRC value (n bits). Simultaneously, also the 
width of the generator polynomial is defined (n+1 bits). Most common used widths are 8, 16 and 32 
bit. But thereotically all widths beginning from 1 are possible. In practice, even quite big (80 
bit) or uneven (5 bit or 31 bit) widths are used.

Polynomial: Used generator polynomial value. Note that different respresentations exist, see 
chapter 7.2.

Initial Value: The value used to initialize the CRC value / register. In the examples above, 
always zero is used, but it could be any value.

Input reflected: If this value is TRUE, each input byte is reflected before being used in the 
calculation. Reflected means that the bits of the input byte are used in reverse order. So this 
also means that bit 0 is treated as the most significant bit and bit 7 as least significant.

Example with byte 0x82 = b10000010: Reflected(0x82) = Reflected(b10000010) = b01000001 = 0x41.

Result reflected: If this value is TRUE, the final CRC value is reflected before being returned. 
The reflection is done over the whole CRC value, so e.g. a CRC-32 value is reflected over all 32 
bits.

Final XOR value: The Final XOR value is xored to the final CRC value before being returned. This 
is done after the 'Result reflected' step. Obviously a Final XOR value of 0 has no impact.

Check value [Optional]: This value is not required but often specified to help to validate the 
implementation. This is the CRC value of input string "123456789" or as byte array: 
[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39].

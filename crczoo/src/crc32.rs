pub fn calculate_crc32(data: &[u8], poly: u32, init: u32, ref_in: bool, ref_out: bool, xor_out: u32) -> u32 {
    let mut crc = init;
    for d in data.iter() {
        let c = if ref_in { (*d).reverse_bits() } else { *d };
        let mut i = 0x80;
        while i > 0 {
            let mut bit = (crc & 0x80000000) != 0;
            if (c & i) != 0 {
                bit = !bit;
            }
            crc <<= 1;
            if bit {
                crc ^= poly;
            }
            i >>= 1;
        }
    }
    if ref_out {
        crc = crc.reverse_bits();
    }
    crc ^ xor_out
}

pub const CRC32_POLY: u32 = 0x04C11DB7;
pub fn crc32(data: &[u8]) -> u32 {
    calculate_crc32(data, CRC32_POLY, 0xFFFFFFFF, true, true, 0xFFFFFFFF)
}

pub const CRC32_BZIP2_POLY: u32 = 0x04C11DB7;
pub fn crc32_bzip2(data: &[u8]) -> u32 {
    calculate_crc32(data, CRC32_BZIP2_POLY, 0xFFFFFFFF, false, false, 0xFFFFFFFF)
}

pub const CRC32C_POLY: u32 = 0x1EDC6F41;
pub fn crc32c(data: &[u8]) -> u32 {
    calculate_crc32(data, CRC32C_POLY, 0xFFFFFFFF, true, true, 0xFFFFFFFF)
}

pub const CRC32D_POLY: u32 = 0xA833982B;
pub fn crc32d(data: &[u8]) -> u32 {
    calculate_crc32(data, CRC32D_POLY, 0xFFFFFFFF, true, true, 0xFFFFFFFF)
}

pub const CRC32_MPEG2_POLY: u32 = 0x04C11DB7;
pub fn crc32_mpeg2(data: &[u8]) -> u32 {
    calculate_crc32(data, CRC32_MPEG2_POLY, 0xFFFFFFFF, false, false, 0x00000000)
}

pub const CRC32_POSIX_POLY: u32 = 0x04C11DB7;
pub fn crc32_posix(data: &[u8]) -> u32 {
    calculate_crc32(data, CRC32_POSIX_POLY, 0x00000000, false, false, 0xFFFFFFFF)
}

pub const CRC32Q_POLY: u32 = 0x814141AB;
pub fn crc32q(data: &[u8]) -> u32 {
    calculate_crc32(data, CRC32Q_POLY, 0x00000000, false, false, 0x00000000)
}

pub const CRC32_JAMCRC_POLY: u32 = 0x04C11DB7;
pub fn crc32_jamcrc(data: &[u8]) -> u32 {
    calculate_crc32(data, CRC32_JAMCRC_POLY, 0xFFFFFFFF, true, true, 0x00000000)
}

pub const CRC32_XFER_POLY: u32 = 0x000000AF;
pub fn crc32_xfer(data: &[u8]) -> u32 {
    calculate_crc32(data, CRC32_XFER_POLY, 0x00000000, false, false, 0x00000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_sequence() -> Vec<u8> {
        "123456789".to_owned().into_bytes()
    }

    fn check_sequence_long() -> Vec<u8> {
        "helloworldchris!".to_owned().into_bytes()
    }

    #[test]
    fn test_crc32() {
        assert_eq!(crc32(&check_sequence()), 0xCBF43926);
    }

    #[test]
    fn test_crc32_long() {
        assert_eq!(crc32(&check_sequence_long()), 0x8783E15A);
    }

    #[test]
    fn test_crc32_long_long() {
        let mut data = check_sequence_long();
        data.append(&mut check_sequence_long());
        assert_eq!(crc32(&data), 0x2C2014D4);
    }

    #[test]
    fn test_crc32_long_long_long() {
        let mut data = check_sequence_long();
        data.append(&mut check_sequence_long());
        data.append(&mut check_sequence_long());
        assert_eq!(crc32(&data), 0xEDA8F8B7);
    }

    #[test]
    fn test_crc32_bzip2() {
        assert_eq!(crc32_bzip2(&check_sequence()), 0xFC891918);
    }

    #[test]
    fn test_crc32c() {
        assert_eq!(crc32c(&check_sequence()), 0xE3069283);
    }

    #[test]
    fn test_crc32d() {
        assert_eq!(crc32d(&check_sequence()), 0x87315576);
    }

    #[test]
    fn test_crc32_mpeg2() {
        assert_eq!(crc32_mpeg2(&check_sequence()), 0x0376E6E7);
    }

    #[test]
    fn test_crc32_mpeg2_long() {
        assert_eq!(crc32_mpeg2(&check_sequence_long()), 0xD13D2899);
    }

    #[test]
    fn test_crc32_posix() {
        assert_eq!(crc32_posix(&check_sequence()), 0x765E7680);
    }

    #[test]
    fn test_crc32q() {
        assert_eq!(crc32q(&check_sequence()), 0x3010BF7F);
    }

    #[test]
    fn test_crc32_jamcrc() {
        assert_eq!(crc32_jamcrc(&check_sequence()), 0x340BC6D9);
    }

    #[test]
    fn test_crc32_xfer() {
        assert_eq!(crc32_xfer(&check_sequence()), 0xBD0BE338);
    }
}
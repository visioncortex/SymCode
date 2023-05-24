pub fn calculate_crc5(data: &[u8], poly: u8, init: u8, ref_in: bool, ref_out: bool, xor_out: u8) -> u8 {
    let mut crc = init;
    for d in data.iter() {
        let c = if ref_in { (*d).reverse_bits() } else { *d };
        let mut i = 0x80;
        while i > 0 {
            let mut bit = (crc & 0x10) != 0;
            if (c & i) != 0 {
                bit = !bit;
            }
            crc <<= 1;
            if bit {
                crc ^= poly;
            }
            i >>= 1;
        }
        crc &= 0x1f;
    }
    if ref_out {
        crc = crc.reverse_bits() >> 3;
    }
    crc ^ xor_out
}

pub const CRC5_POLY: u8 = 0x5;
pub fn crc5(data: &[u8]) -> u8 {
    calculate_crc5(data, CRC5_POLY, 0x1f, true, true, 0x1f)
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
    fn test_crc5() {
        assert_eq!(crc5(&check_sequence()), 0x19);
    }

    #[test]
    fn test_crc5_long() {
        assert_eq!(crc5(&check_sequence_long()), 0xa);
    }

    #[test]
    fn test_crc5_long_long() {
        let mut data = check_sequence_long();
        data.append(&mut check_sequence_long());
        assert_eq!(crc5(&data), 0x1b);
    }

    #[test]
    fn test_crc5_long_long_long() {
        let mut data = check_sequence_long();
        data.append(&mut check_sequence_long());
        data.append(&mut check_sequence_long());
        assert_eq!(crc5(&data), 0xc);
    }
}

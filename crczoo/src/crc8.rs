pub fn calculate_crc8(data: &[u8], poly: u8, init: u8, ref_in: bool, ref_out: bool, xor_out: u8) -> u8 {
    let mut crc = init;
    for d in data.iter() {
        let c = if ref_in { (*d).reverse_bits() } else { *d };
        let mut i = 0x80;
        while i > 0 {
            let mut bit = (crc & 0x80) != 0;
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

pub const CRC8_POLY: u8 = 0x07; 
pub fn crc8(data: &[u8]) -> u8 {
    calculate_crc8(data, CRC8_POLY, 0x00, false, false, 0x00)
}

pub const CRC8_CDMA2000_POLY: u8 = 0x9B;
pub fn crc8_cdma2000(data: &[u8]) -> u8 {
    calculate_crc8(data, CRC8_CDMA2000_POLY, 0xFF, false, false, 0x00)
}

pub const CRC8_DARC_POLY: u8 = 0x39;
pub fn crc8_darc(data: &[u8]) -> u8 {
    calculate_crc8(data, CRC8_DARC_POLY, 0x00, true, true, 0x00)
}

pub const CRC8_DVB_S2_POLY: u8 = 0xD5;
pub fn crc8_dvb_s2(data: &[u8]) -> u8 {
    calculate_crc8(data, CRC8_DVB_S2_POLY, 0x00, false, false, 0x00)
}

pub const CRC8_EBU_POLY: u8 = 0x1D;
pub fn crc8_ebu(data: &[u8]) -> u8 {
    calculate_crc8(data, CRC8_EBU_POLY, 0xFF, true, true, 0x00)
}

pub const CRC8_I_CODE_POLY: u8 = 0x1D;
pub fn crc8_i_code(data: &[u8]) -> u8 {
    calculate_crc8(data, CRC8_I_CODE_POLY, 0xFD, false, false, 0x00)
}

pub const CRC8_ITU_POLY: u8 = 0x07;
pub fn crc8_itu(data: &[u8]) -> u8 {
    calculate_crc8(data, CRC8_ITU_POLY, 0x00, false, false, 0x55)
}

pub const CRC8_MAXIM_POLY: u8 = 0x31;
pub fn crc8_maxim(data: &[u8]) -> u8 {
    calculate_crc8(data, CRC8_MAXIM_POLY, 0x00, true, true, 0x00)
}

pub const CRC8_ROHC_POLY: u8 = 0x07;
pub fn crc8_rohc(data: &[u8]) -> u8 {
    calculate_crc8(data, CRC8_ROHC_POLY, 0xFF, true, true, 0x00)
}

pub const CRC8_WCDMA_POLY: u8 = 0x9B;
pub fn crc8_wcdma(data: &[u8]) -> u8 {
    calculate_crc8(data, CRC8_WCDMA_POLY, 0x00, true, true, 0x00)
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
    fn test_crc8() {
        assert_eq!(crc8(&check_sequence()), 0xF4);
    }

    #[test]
    fn test_crc8_long() {
        assert_eq!(crc8(&check_sequence_long()), 0x4E);
    }

    #[test]
    fn test_crc8_long_long() {
        let mut data = check_sequence_long();
        data.append(&mut check_sequence_long());
        assert_eq!(crc8(&data), 0xD2);
    }

    #[test]
    fn test_crc8_long_long_long() {
        let mut data = check_sequence_long();
        data.append(&mut check_sequence_long());
        data.append(&mut check_sequence_long());
        assert_eq!(crc8(&data), 0xED);
    }

    #[test]
    fn test_crc8_demo() {
        let mut data = "123456789".to_owned().into_bytes();
        let checksum = crc8(&data);
        data.push(checksum);
        assert_eq!(crc8(&data), 0);

        let mut data = "023456789".to_owned().into_bytes();
        data.push(checksum);
        assert!(crc8(&data) != 0);

        let mut data = "023456799".to_owned().into_bytes();
        data.push(checksum);
        assert!(crc8(&data) != 0);
    }

    #[test]
    fn test_crc8_cdma2000() {
        assert_eq!(crc8_cdma2000(&check_sequence()), 0xDA);
    }

    #[test]
    fn test_crc8_darc() {
        assert_eq!(crc8_darc(&check_sequence()), 0x15);
    }

    #[test]
    fn test_crc8_dvb_s2() {
        assert_eq!(crc8_dvb_s2(&check_sequence()), 0xBC);
    }

    #[test]
    fn test_crc8_ebu() {
        assert_eq!(crc8_ebu(&check_sequence()), 0x97);
    }

    #[test]
    fn test_crc8_i_code() {
        assert_eq!(crc8_i_code(&check_sequence()), 0x7E);
    }

    #[test]
    fn test_crc8_itu() {
        assert_eq!(crc8_itu(&check_sequence()), 0xA1);
    }

    #[test]
    fn test_crc8_maxim() {
        assert_eq!(crc8_maxim(&check_sequence()), 0xA1);
    }

    #[test]
    fn test_crc8_rohc() {
        assert_eq!(crc8_rohc(&check_sequence()), 0xD0);
    }

    #[test]
    fn test_crc8_wcdma() {
        assert_eq!(crc8_wcdma(&check_sequence()), 0x25);
    }
}
pub fn calculate_crc16(data: &[u8], poly: u16, init: u16, ref_in: bool, ref_out: bool, xor_out: u16) -> u16 {
    let mut crc = init;
    for d in data.iter() {
        let c = if ref_in { (*d).reverse_bits() } else { *d };
        let mut i = 0x80;
        while i > 0 {
            let mut bit = (crc & 0x8000) != 0;
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

pub const CRC16_CCITT_FALSE_POLY: u16 = 0x1021;
pub fn crc16_ccitt_false(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_CCITT_FALSE_POLY, 0xFFFF, false, false, 0x0000)
}

pub const CRC16_ARC_POLY: u16 = 0x8005;
pub fn crc16_arc(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_ARC_POLY, 0x0000, true, true, 0x0000)
}

pub const CRC16_AUG_CCITT_POLY: u16 = 0x1021;
pub fn crc16_aug_ccitt(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_AUG_CCITT_POLY, 0x1D0F, false, false, 0x0000)
}

pub const CRC16_BUYPASS_POLY: u16 = 0x8005;
pub fn crc16_buypass(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_BUYPASS_POLY, 0x0000, false, false, 0x0000)
}

pub const CRC16_CDMA2000_POLY: u16 = 0xC867;
pub fn crc16_cdma2000(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_CDMA2000_POLY, 0xFFFF, false, false, 0x0000)
}

pub const CRC16_DDS_110_POLY: u16 = 0x8005;
pub fn crc16_dds_110(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_DDS_110_POLY, 0x800D, false, false, 0x0000)
}

pub const CRC16_DECT_R_POLY: u16 = 0x0589;
pub fn crc16_dect_r(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_DECT_R_POLY, 0x0000, false, false, 0x0001)
}

pub const CRC16_DECT_X_POLY: u16 = 0x0589;
pub fn crc16_dect_x(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_DECT_X_POLY, 0x0000, false, false, 0x0000)
}

pub const CRC16_DNP_POLY: u16 = 0x3D65;
pub fn crc16_dnp(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_DNP_POLY, 0x0000, true, true, 0xFFFF)
}

pub const CRC16_EN_13757_POLY: u16 = 0x3D65;
pub fn crc16_en_13757(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_EN_13757_POLY, 0x0000, false, false, 0xFFFF)
}

pub const CRC16_GENIBUS_POLY: u16 = 0x1021;
pub fn crc16_genibus(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_GENIBUS_POLY, 0xFFFF, false, false, 0xFFFF)
}

pub const CRC16_MAXIM_POLY: u16 = 0x8005;
pub fn crc16_maxim(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_MAXIM_POLY, 0x0000, true, true, 0xFFFF)
}

pub const CRC16_MCRF4XX_POLY: u16 = 0x1021;
pub fn crc16_mcrf4xx(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_MCRF4XX_POLY, 0xFFFF, true, true, 0x0000)
}

pub const CRC16_RIELLO_POLY: u16 = 0x1021;
pub fn crc16_riello(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_RIELLO_POLY, 0xB2AA, true, true, 0x0000)
}

pub const CRC16_T10_DIF_POLY: u16 = 0x8BB7;
pub fn crc16_t10_dif(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_T10_DIF_POLY, 0x0000, false, false, 0x0000)
}

pub const CRC16_TELEDISK_POLY: u16 = 0xA097;
pub fn crc16_teledisk(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_TELEDISK_POLY, 0x0000, false, false, 0x0000)
}

pub const CRC16_TMS37157_POLY: u16 = 0x1021;
pub fn crc16_tms37157(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_TMS37157_POLY, 0x89EC, true, true, 0x0000)
}

pub const CRC16_USB_POLY: u16 = 0x8005;
pub fn crc16_usb(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_USB_POLY, 0xFFFF, true, true, 0xFFFF)
}

pub const CRC16_A_POLY: u16 = 0x1021;
pub fn crc16_a(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_A_POLY, 0xC6C6, true, true, 0x0000)
}

pub const CRC16_KERMIT_POLY: u16 = 0x1021;
pub fn crc16_kermit(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_KERMIT_POLY, 0x0000, true, true, 0x0000)
}

pub const CRC16_MODBUS_POLY: u16 = 0x8005;
pub fn crc16_modbus(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_MODBUS_POLY, 0xFFFF, true, true, 0x0000)
}

pub const CRC16_X_25_POLY: u16 = 0x1021;
pub fn crc16_x_25(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_X_25_POLY, 0xFFFF, true, true, 0xFFFF)
}

pub const CRC16_XMODEM_POLY: u16 = 0x1021;
pub fn crc16_xmodem(data: &[u8]) -> u16 {
    calculate_crc16(data, CRC16_XMODEM_POLY, 0x0000, false, false, 0x0000)
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
    fn test_crc16_ccitt_false() {
        assert_eq!(crc16_ccitt_false(&check_sequence()), 0x29B1);
    }

    #[test]
    fn test_crc16_arc() {
        assert_eq!(crc16_arc(&check_sequence()), 0xBB3D);
    }

    #[test]
    fn test_crc16_aug_ccitt() {
        assert_eq!(crc16_aug_ccitt(&check_sequence()), 0xE5CC);
    }

    #[test]
    fn test_crc16_buypass() {
        assert_eq!(crc16_buypass(&check_sequence()), 0xFEE8);
    }

    #[test]
    fn test_crc16_cdma2000() {
        assert_eq!(crc16_cdma2000(&check_sequence()), 0x4C06);
    }

    #[test]
    fn test_crc16_dds_110() {
        assert_eq!(crc16_dds_110(&check_sequence()), 0x9ECF);
    }

    #[test]
    fn test_crc16_dect_r() {
        assert_eq!(crc16_dect_r(&check_sequence()), 0x007E);
    }

    #[test]
    fn test_crc16_dect_x() {
        assert_eq!(crc16_dect_x(&check_sequence()), 0x007F);
    }

    #[test]
    fn test_crc16_dnp() {
        assert_eq!(crc16_dnp(&check_sequence()), 0xEA82);
    }

    #[test]
    fn test_crc16_en_13757() {
        assert_eq!(crc16_en_13757(&check_sequence()), 0xC2B7);
    }

    #[test]
    fn test_crc16_genibus() {
        assert_eq!(crc16_genibus(&check_sequence()), 0xD64E);
    }

    #[test]
    fn test_crc16_maxim() {
        assert_eq!(crc16_maxim(&check_sequence()), 0x44C2);
    }

    #[test]
    fn test_crc16_mcrf4xx() {
        assert_eq!(crc16_mcrf4xx(&check_sequence()), 0x6F91);
    }

    #[test]
    fn test_crc16_riello() {
        assert_eq!(crc16_riello(&check_sequence()), 0x63D0);
    }

    #[test]
    fn test_crc16_t10_dif() {
        assert_eq!(crc16_t10_dif(&check_sequence()), 0xD0DB);
    }

    #[test]
    fn test_crc16_teledisk() {
        assert_eq!(crc16_teledisk(&check_sequence()), 0x0FB3);
    }

    #[test]
    fn test_crc16_tms37157() {
        assert_eq!(crc16_tms37157(&check_sequence()), 0x26B1);
    }

    #[test]
    fn test_crc16_usb() {
        assert_eq!(crc16_usb(&check_sequence()), 0xB4C8);
    }

    #[test]
    fn test_crc16_usb_long() {
        assert_eq!(crc16_usb(&check_sequence_long()), 0x6A39);
    }

    #[test]
    fn test_crc16_usb_long_long() {
        let mut data = check_sequence_long();
        data.append(&mut check_sequence_long());
        assert_eq!(crc16_usb(&data), 0x0701);
    }

    #[test]
    fn test_crc16_usb_long_long_long() {
        let mut data = check_sequence_long();
        data.append(&mut check_sequence_long());
        data.append(&mut check_sequence_long());
        assert_eq!(crc16_usb(&data), 0xF8BF);
    }

    #[test]
    fn test_crc16_a() {
        assert_eq!(crc16_a(&check_sequence()), 0xBF05);
    }

    #[test]
    fn test_crc16_kermit() {
        assert_eq!(crc16_kermit(&check_sequence()), 0x2189);
    }

    #[test]
    fn test_crc16_modbus() {
        assert_eq!(crc16_modbus(&check_sequence()), 0x4B37);
    }

    #[test]
    fn test_crc16_x_25() {
        assert_eq!(crc16_x_25(&check_sequence()), 0x906E);
    }

    #[test]
    fn test_crc16_xmodem() {
        assert_eq!(crc16_xmodem(&check_sequence()), 0x31C3);
    }
}

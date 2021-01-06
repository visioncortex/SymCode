pub fn check(mut msg: u64, divisor: u16) -> u16 {
    let mut mask = 1u64 << 63;
    let mut d = (divisor as u64) << 48;

    while mask != 0x8000 {
        if (msg & mask) > 0 {
            msg ^= d;
        }
        d >>= 1;
        mask >>= 1;
    }

    (msg & 0xFFFF) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_output() {
        assert_eq!(check(0xE4DCD2F82E3060AF, 0x8005), 0x3958); 
    }
}
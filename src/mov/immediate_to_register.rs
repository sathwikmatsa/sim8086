use crate::instruction::{Data, Instruction, Register, WithData, WithRegField, WithWideField};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ImmediateToRegister {
    pub reg: Register,
    pub data: Data,
}

impl fmt::Display for ImmediateToRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mov {}, {}", self.reg, u16::from(&self.data))
    }
}

impl WithWideField for ImmediateToRegister {
    const WIDE_MASK_MATCH: u8 = 0b00001000;
}

impl WithRegField for ImmediateToRegister {
    const RIGHT_SHIFT_BY: u8 = 0;
}

impl WithData for ImmediateToRegister {}

impl Instruction for ImmediateToRegister {
    fn opcode_mask() -> u8
    where
        Self: Sized,
    {
        const OPCODE_MASK: u8 = 0b11110000;
        return OPCODE_MASK;
    }

    fn opcode_match() -> u8
    where
        Self: Sized,
    {
        const OPCODE_MATCH: u8 = 0b10110000;
        return OPCODE_MATCH;
    }

    fn new<'a, I>(first_byte: u8, byte_stream: &mut I) -> Self
    where
        I: Iterator<Item = &'a u8>,
    {
        let reg = Self::extract_reg(first_byte, first_byte);
        let data = Self::extract_data(first_byte, byte_stream);
        Self { reg, data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_match() {
        assert!(ImmediateToRegister::match_opcode(0b10110101));
        assert!(!ImmediateToRegister::match_opcode(0b10010101));
    }

    #[test]
    fn not_wide() {
        let bytes: [u8; 2] = [0b10110000, 0b00000001];
        assert_eq!(
            ImmediateToRegister::new(bytes[0], &mut bytes[1..].into_iter()),
            ImmediateToRegister {
                reg: Register::AL,
                data: Data::U8(1)
            }
        );
    }

    #[test]
    fn wide() {
        let bytes: [u8; 3] = [0b10111000, 0b00000000, 0b00000001];
        assert_eq!(
            ImmediateToRegister::new(bytes[0], &mut bytes[1..].into_iter()),
            ImmediateToRegister {
                reg: Register::AX,
                data: Data::U16(256)
            }
        );
    }
}

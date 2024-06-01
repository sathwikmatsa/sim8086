use std::fmt;

use crate::instruction::{Data, Instruction, WithData, WithRMField, WithWideField, RM};

#[derive(Debug, PartialEq)]
pub struct ImmediateToRegisterMemory {
    pub rm: RM,
    pub data: Data,
}

impl fmt::Display for ImmediateToRegisterMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mov {}, {}", self.rm, self.data)
    }
}

impl WithWideField for ImmediateToRegisterMemory {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl WithData for ImmediateToRegisterMemory {}

impl WithRMField for ImmediateToRegisterMemory {}

impl Instruction for ImmediateToRegisterMemory {
    fn opcode_mask() -> u8
    where
        Self: Sized,
    {
        const OPCODE_MASK: u8 = 0b11111110;
        return OPCODE_MASK;
    }

    fn opcode_match() -> u8
    where
        Self: Sized,
    {
        const OPCODE_MATCH: u8 = 0b11000110;
        return OPCODE_MATCH;
    }

    fn new<'a, I>(first_byte: u8, mut byte_stream: &mut I) -> Self
    where
        I: Iterator<Item = &'a u8>,
    {
        let second_byte = byte_stream
            .next()
            .expect("extract second instruction byte")
            .to_owned();
        let rm = Self::extract_rm(first_byte, second_byte, &mut byte_stream);
        let data = Self::extract_data(first_byte, byte_stream);
        Self { rm, data }
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::{Data, Register};

    use super::*;

    #[test]
    fn opcode_match() {
        let bytes: [u8; 1] = [0b11000110];
        assert!(ImmediateToRegisterMemory::match_opcode(bytes[0]));
    }

    #[test]
    fn immediate_to_register_wide() {
        let bytes: [u8; 4] = [0b11000111, 0b11000011, 0b00000100, 0b00000001];
        assert_eq!(
            ImmediateToRegisterMemory::new(bytes[0], &mut bytes[1..].into_iter()),
            ImmediateToRegisterMemory {
                rm: RM::Reg(Register::BX),
                data: Data::U16(260)
            }
        )
    }

    #[test]
    fn immediate_to_register_not_wide() {
        let bytes: [u8; 3] = [0b11000110, 0b11000011, 0b00000100];
        assert_eq!(
            ImmediateToRegisterMemory::new(bytes[0], &mut bytes[1..].into_iter()),
            ImmediateToRegisterMemory {
                rm: RM::Reg(Register::BL),
                data: Data::U8(4)
            }
        )
    }

    #[test]
    fn immediate_to_mem_no_disp() {
        let bytes: [u8; 3] = [0b11000110, 0b00000011, 0b00000101];
        assert_eq!(
            ImmediateToRegisterMemory::new(bytes[0], &mut bytes[1..].into_iter()),
            ImmediateToRegisterMemory {
                rm: RM::Mem(crate::instruction::EffectiveAddress::BP_DI(None)),
                data: Data::U8(5)
            }
        )
    }

    #[test]
    fn immediate_to_mem_direct_address() {
        let bytes: [u8; 6] = [
            0b11000111, 0b00000110, 0b00000100, 0b00000000, 0b00000000, 0b00000001,
        ];
        assert_eq!(
            ImmediateToRegisterMemory::new(bytes[0], &mut bytes[1..].into_iter()),
            ImmediateToRegisterMemory {
                rm: RM::Mem(crate::instruction::EffectiveAddress::DirectAddress(4)),
                data: Data::U16(256)
            }
        )
    }

    #[test]
    fn immediate_to_mem_8bit_disp() {
        let bytes: [u8; 4] = [0b11000110, 0b01000110, 0b00000100, 0b00000000];
        assert_eq!(
            ImmediateToRegisterMemory::new(bytes[0], &mut bytes[1..].into_iter()),
            ImmediateToRegisterMemory {
                rm: RM::Mem(crate::instruction::EffectiveAddress::BP(Data::U8(4))),
                data: Data::U8(0)
            }
        )
    }

    #[test]
    fn immediate_to_mem_16bit_disp() {
        let bytes: [u8; 6] = [
            0b11000111, 0b10000100, 0b00000100, 0b00000000, 0b00000000, 0b00000001,
        ];
        assert_eq!(
            ImmediateToRegisterMemory::new(bytes[0], &mut bytes[1..].into_iter()),
            ImmediateToRegisterMemory {
                rm: RM::Mem(crate::instruction::EffectiveAddress::SI(Some(Data::U16(4)))),
                data: Data::U16(256)
            }
        )
    }
}

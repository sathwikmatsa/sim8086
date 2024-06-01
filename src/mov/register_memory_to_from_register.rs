use std::fmt;

use crate::instruction::{
    Instruction, Register, WithDestField, WithRMField, WithRegField, WithWideField, RM,
};

#[derive(Debug, PartialEq)]
pub struct RegisterMemoryToFromRegister {
    pub reg: Register,
    pub rm: RM,
    pub is_reg_dest: bool,
}

impl fmt::Display for RegisterMemoryToFromRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_reg_dest {
            write!(f, "mov {}, {}", self.reg, self.rm)
        } else {
            write!(f, "mov {}, {}", self.rm, self.reg)
        }
    }
}

impl WithDestField for RegisterMemoryToFromRegister {
    const DEST_MASK_MATCH: u8 = 0b00000010;
}

impl WithWideField for RegisterMemoryToFromRegister {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl WithRegField for RegisterMemoryToFromRegister {
    const RIGHT_SHIFT_BY: u8 = 3;
}

impl WithRMField for RegisterMemoryToFromRegister {}

impl Instruction for RegisterMemoryToFromRegister {
    fn opcode_mask() -> u8
    where
        Self: Sized,
    {
        const OPCODE_MASK: u8 = 0b11111100;
        return OPCODE_MASK;
    }

    fn opcode_match() -> u8
    where
        Self: Sized,
    {
        const OPCODE_MATCH: u8 = 0b10001000;
        return OPCODE_MATCH;
    }

    fn new<'a, I>(first_byte: u8, byte_stream: &mut I) -> Self
    where
        I: Iterator<Item = &'a u8>,
    {
        let second_byte = byte_stream
            .next()
            .expect("extract second instruction byte")
            .to_owned();
        let reg = Self::extract_reg(first_byte, second_byte);
        let is_reg_dest = Self::is_dest_in_reg_field(first_byte);
        let rm = Self::extract_rm(first_byte, second_byte, byte_stream);
        Self {
            reg,
            rm,
            is_reg_dest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_match() {
        let bytes: [u8; 1] = [0b10001000];
        assert!(RegisterMemoryToFromRegister::match_opcode(bytes[0]));
    }

    #[test]
    fn reg_to_reg_wide() {
        let bytes: [u8; 2] = [0b10001001, 0b11000011];
        assert_eq!(
            RegisterMemoryToFromRegister::new(bytes[0], &mut bytes[1..].into_iter()),
            RegisterMemoryToFromRegister {
                reg: Register::AX,
                rm: RM::Reg(Register::BX),
                is_reg_dest: false
            }
        )
    }

    #[test]
    fn reg_to_reg_not_wide() {
        let bytes: [u8; 2] = [0b10001000, 0b11000011];
        assert_eq!(
            RegisterMemoryToFromRegister::new(bytes[0], &mut bytes[1..].into_iter()),
            RegisterMemoryToFromRegister {
                reg: Register::AL,
                rm: RM::Reg(Register::BL),
                is_reg_dest: false
            }
        )
    }

    #[test]
    fn reg_to_mem_no_disp() {
        let bytes: [u8; 2] = [0b10001000, 0b00010011];
        assert_eq!(
            RegisterMemoryToFromRegister::new(bytes[0], &mut bytes[1..].into_iter()),
            RegisterMemoryToFromRegister {
                reg: Register::DL,
                rm: RM::Mem(crate::instruction::EffectiveAddress::BP_DI(None)),
                is_reg_dest: false
            }
        )
    }

    #[test]
    fn reg_to_mem_direct_address() {
        let bytes: [u8; 4] = [0b10001001, 0b00010110, 0b00000001, 0b00000000];
        assert_eq!(
            RegisterMemoryToFromRegister::new(bytes[0], &mut bytes[1..].into_iter()),
            RegisterMemoryToFromRegister {
                reg: Register::DX,
                rm: RM::Mem(crate::instruction::EffectiveAddress::DirectAddress(1)),
                is_reg_dest: false
            }
        )
    }

    #[test]
    fn reg_to_mem_8bit_disp() {
        let bytes: [u8; 3] = [0b10001000, 0b01001110, 0b00000010];
        assert_eq!(
            RegisterMemoryToFromRegister::new(bytes[0], &mut bytes[1..].into_iter()),
            RegisterMemoryToFromRegister {
                reg: Register::CL,
                rm: RM::Mem(crate::instruction::EffectiveAddress::BP(2)),
                is_reg_dest: false
            }
        )
    }

    #[test]
    fn reg_to_mem_16bit_disp() {
        let bytes: [u8; 4] = [0b10001001, 0b10011000, 0b00000000, 0b00000001];
        assert_eq!(
            RegisterMemoryToFromRegister::new(bytes[0], &mut bytes[1..].into_iter()),
            RegisterMemoryToFromRegister {
                reg: Register::BX,
                rm: RM::Mem(crate::instruction::EffectiveAddress::BX_SI(Some(256))),
                is_reg_dest: false
            }
        )
    }

    #[test]
    fn mem_16bit_disp_to_reg() {
        let bytes: [u8; 4] = [0b10001011, 0b10011000, 0b00000000, 0b00000001];
        assert_eq!(
            RegisterMemoryToFromRegister::new(bytes[0], &mut bytes[1..].into_iter()),
            RegisterMemoryToFromRegister {
                reg: Register::BX,
                rm: RM::Mem(crate::instruction::EffectiveAddress::BX_SI(Some(256))),
                is_reg_dest: true
            }
        )
    }
}

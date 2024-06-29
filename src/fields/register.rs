use enum_stringify::EnumStringify;

use super::Operand;

#[derive(EnumStringify, Debug, PartialEq, Copy, Clone)]
#[rustfmt::skip]
#[enum_stringify(case ="lower")]
pub enum Register {
    AX, BX, CX, DX,
    AH, BH, CH, DH,
    AL, BL, CL, DL,
    SP, BP, SI, DI,
}

impl Register {
    pub fn is_wide(&self) -> bool {
        matches!(
            self,
            Self::AX | Self::BX | Self::CX | Self::DX | Self::SP | Self::BP | Self::SI | Self::DI
        )
    }
}

impl From<Register> for Operand {
    fn from(val: Register) -> Self {
        Operand::Register(val)
    }
}

#[rustfmt::skip]
pub fn register_from_u8(reg: u8, wide: bool) -> Register {
    match reg {
        0b000 => if wide { Register::AX } else { Register::AL },
        0b001 => if wide { Register::CX } else { Register::CL },
        0b010 => if wide { Register::DX } else { Register::DL },
        0b011 => if wide { Register::BX } else { Register::BL },
        0b100 => if wide { Register::SP } else { Register::AH },
        0b101 => if wide { Register::BP } else { Register::CH },
        0b110 => if wide { Register::SI } else { Register::DH },
        0b111 => if wide { Register::DI } else { Register::BH },
        _ => unreachable!()
    }
}

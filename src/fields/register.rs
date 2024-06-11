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

impl From<Register> for Operand {
    fn from(val: Register) -> Self {
        Operand::Register(val)
    }
}

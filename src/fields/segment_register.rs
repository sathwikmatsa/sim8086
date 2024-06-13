use enum_stringify::EnumStringify;

use super::Operand;

#[derive(EnumStringify, Debug, PartialEq, Copy, Clone)]
#[enum_stringify(case = "lower")]
pub enum SegmentRegister {
    ES,
    CS,
    SS,
    DS,
}

impl From<SegmentRegister> for Operand {
    fn from(val: SegmentRegister) -> Self {
        Operand::SR(val)
    }
}

pub fn sr_from_u8(sr: u8) -> SegmentRegister {
    match sr {
        0b00 => SegmentRegister::ES,
        0b01 => SegmentRegister::CS,
        0b10 => SegmentRegister::SS,
        0b11 => SegmentRegister::DS,
        _ => unreachable!(),
    }
}

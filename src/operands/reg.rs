use crate::{
    extractors::{WithRegField, WithWideField},
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct Reg;

impl WithRegField for Reg {
    const RIGHT_SHIFT_BY: u8 = 0;
}

impl WithWideField for Reg {
    // no wide bit; implicitly deals with 16-bit registers
    const WIDE_MASK_MATCH: u8 = 0;
}

impl InstructionDecoder for Reg {
    fn decode(
        &self,
        first_byte: u8,
        _byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let reg = Self::extract_reg(first_byte, first_byte).into();
        Inst::with_operand(op, reg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{Operand, Register};

    const DECODER: Reg = Reg;

    #[test]
    fn reg() {
        let bytes: [u8; 1] = [0b01010011];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Push),
            Inst::with_operand(Operation::Push, Operand::Register(Register::BX))
        )
    }
}

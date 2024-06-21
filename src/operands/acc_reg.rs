use crate::{
    disasm::{WithRegField, WithWideField},
    fields::{Operand, Operation, Register},
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct AccReg;

impl WithRegField for AccReg {
    const RIGHT_SHIFT_BY: u8 = 0;
}

impl WithWideField for AccReg {
    // no wide bit; implicitly deals with 16-bit registers
    const WIDE_MASK_MATCH: u8 = 0;
}

impl InstructionDecoder for AccReg {
    fn decode(
        &self,
        first_byte: u8,
        _byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let reg = Self::extract_reg(first_byte, first_byte).into();
        Inst::with_operands(op, Operand::Register(Register::AX), reg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::Operand;

    const DECODER: AccReg = AccReg;

    #[test]
    fn reg() {
        let bytes: [u8; 1] = [0b10010110];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::XCHG),
            Inst::with_operands(
                Operation::XCHG,
                Operand::Register(Register::AX),
                Operand::Register(Register::SI),
            )
        )
    }
}

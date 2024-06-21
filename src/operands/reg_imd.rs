use crate::{
    disasm::{WithData, WithRegField, WithWideField},
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct RegImd;

impl WithWideField for RegImd {
    const WIDE_MASK_MATCH: u8 = 0b00001000;
}

impl WithRegField for RegImd {
    const RIGHT_SHIFT_BY: u8 = 0;
}

impl WithData for RegImd {}

impl InstructionDecoder for RegImd {
    fn decode(
        &self,
        first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let reg = Self::extract_reg(first_byte, first_byte).into();
        let data = Self::extract_data(first_byte, byte_stream).into();
        Inst::with_operands(op, reg, data)
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{Data, Operand, Register};

    use super::*;

    const DECODER: RegImd = RegImd;

    #[test]
    fn not_wide() {
        let bytes: [u8; 2] = [0b10110000, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Mov),
            Inst::with_operands(
                Operation::Mov,
                Operand::Register(Register::AL),
                Operand::Immediate(Data::U8(1))
            )
        );
    }

    #[test]
    fn wide() {
        let bytes: [u8; 3] = [0b10111000, 0b00000000, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Mov),
            Inst::with_operands(
                Operation::Mov,
                Operand::Register(Register::AX),
                Operand::Immediate(Data::U16(256))
            )
        );
    }
}

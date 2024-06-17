use crate::{
    extractors::{WithData, WithWideField},
    fields::{Operation, Register},
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct AccImd;

impl WithWideField for AccImd {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl WithData for AccImd {}

impl InstructionDecoder for AccImd {
    fn decode(
        &self,
        first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let wide = Self::is_wide(first_byte);
        let data = Self::extract_data(first_byte, byte_stream).into();
        let acc = if wide { Register::AX } else { Register::AL }.into();
        Inst::with_operands(op, acc, data)
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{Data, Operand};

    use super::*;

    const DECODER: AccImd = AccImd;

    #[test]
    fn not_wide() {
        let bytes: [u8; 2] = [0b00111100, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Cmp),
            Inst::with_operands(
                Operation::Cmp,
                Operand::Register(Register::AL),
                Operand::Immediate(Data::U8(1))
            )
        );
    }

    #[test]
    fn wide() {
        let bytes: [u8; 3] = [0b00111101, 0b00000001, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Cmp),
            Inst::with_operands(
                Operation::Cmp,
                Operand::Register(Register::AX),
                Operand::Immediate(Data::U16(257))
            )
        );
    }
}

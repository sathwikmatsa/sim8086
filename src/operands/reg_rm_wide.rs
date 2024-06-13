use crate::{
    extractors::{WithRMField, WithRegField, WithWideField},
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct RegRMW;

impl WithRMField for RegRMW {}

impl WithRegField for RegRMW {
    const RIGHT_SHIFT_BY: u8 = 3;
}

impl WithWideField for RegRMW {
    // no w bit; implicitly deals with 16-bit registers
    const WIDE_MASK_MATCH: u8 = 0;
}

impl InstructionDecoder for RegRMW {
    fn decode(
        &self,
        first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let second_byte = byte_stream
            .next()
            .expect("extract second instruction byte")
            .to_owned();

        let reg = Self::extract_reg(first_byte, second_byte).into();
        let rm = Self::extract_rm(first_byte, second_byte, byte_stream).into();
        Inst {
            operation: op,
            first: Some(reg),
            second: Some(rm),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{Operand, Register};

    const DECODER: RegRMW = RegRMW;

    #[test]
    fn lea() {
        let bytes: [u8; 2] = [0b10001101, 0b11000011];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::LEA),
            Inst {
                first: Some(Operand::Register(Register::AX)),
                second: Some(Operand::Register(Register::BX)),
                operation: Operation::LEA
            }
        )
    }
}

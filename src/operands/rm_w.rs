use crate::{
    extractors::{WithRMField, WithWideField},
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct RMW;

impl WithRMField for RMW {}

impl WithWideField for RMW {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl InstructionDecoder for RMW {
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

        let rm = Self::extract_rm(first_byte, second_byte, byte_stream).into();
        Inst {
            operation: op,
            first: Some(rm),
            second: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{Operand, Register};

    const DECODER: RMW = RMW;

    #[test]
    fn inc() {
        let bytes: [u8; 2] = [0b11111111, 0b11000011];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::INC),
            Inst {
                first: Some(Operand::Register(Register::BX)),
                second: None,
                operation: Operation::INC
            }
        )
    }
}

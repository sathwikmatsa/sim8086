use crate::{
    extractors::{WithRMField, WithVField, WithWideField},
    fields::{Data, Operation, Register},
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct RMVW;

impl WithRMField for RMVW {}

impl WithWideField for RMVW {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl WithVField for RMVW {
    const V_MASK_MATCH: u8 = 0b00000010;
}

impl InstructionDecoder for RMVW {
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
        let v = Self::is_v_set(first_byte);
        Inst::with_operands(
            op,
            rm,
            if v {
                Register::CL.into()
            } else {
                Data::U8(1).into()
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{Operand, Register};

    const DECODER: RMVW = RMVW;

    #[test]
    fn one() {
        let bytes: [u8; 2] = [0b11010001, 0b11100011];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::SHL),
            Inst::with_operands(
                Operation::SHL,
                Operand::Register(Register::BX),
                Operand::Immediate(Data::U8(1))
            )
        )
    }

    #[test]
    fn cl() {
        let bytes: [u8; 2] = [0b11010011, 0b11100011];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::SHL),
            Inst::with_operands(
                Operation::SHL,
                Operand::Register(Register::BX),
                Operand::Register(Register::CL)
            )
        )
    }
}

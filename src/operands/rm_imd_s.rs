use crate::{
    disasm::{WithDataS, WithRMField, WithSignField, WithWideField},
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
    ByteStream,
};

#[derive(Default)]
pub struct RMImdS;

impl WithRMField for RMImdS {}

impl WithSignField for RMImdS {}

impl WithDataS for RMImdS {}

impl WithWideField for RMImdS {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl InstructionDecoder for RMImdS {
    fn decode(&self, first_byte: u8, byte_stream: &mut ByteStream, op: Operation) -> Inst {
        let second_byte = byte_stream
            .next()
            .expect("extract second instruction byte")
            .to_owned();
        let rm = Self::extract_rm(first_byte, second_byte, byte_stream).into();
        let data = Self::extract_data(first_byte, byte_stream).into();
        Inst::with_operands(op, rm, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{Data, Operand, Register};

    const DECODER: RMImdS = RMImdS;

    #[test]
    fn immediate_to_register_sign() {
        let bytes: [u8; 3] = [0b10000010, 0b11000011, 0b00000100];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Add
            ),
            Inst::with_operands(
                Operation::Add,
                Operand::Register(Register::BL),
                Operand::Immediate(Data::U16(4))
            )
        )
    }

    #[test]
    fn immediate_to_register_sign_not_set() {
        let bytes: [u8; 4] = [0b10000001, 0b11000011, 0b00000100, 0b00000001];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Add
            ),
            Inst::with_operands(
                Operation::Add,
                Operand::Register(Register::BX),
                Operand::Immediate(Data::U16(260))
            )
        )
    }
}

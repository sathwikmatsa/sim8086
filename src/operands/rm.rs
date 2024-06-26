use crate::{
    disasm::{WithRMField, WithWideField},
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
    ByteStream,
};

#[derive(Default)]
pub struct RM;

impl WithRMField for RM {}

impl WithWideField for RM {
    // no wide bit; implicitly deals with 16-bit registers
    const WIDE_MASK_MATCH: u8 = 0;
}

impl InstructionDecoder for RM {
    fn decode(&self, first_byte: u8, byte_stream: &mut ByteStream, op: Operation) -> Inst {
        let second_byte = byte_stream
            .next()
            .expect("extract second instruction byte")
            .to_owned();

        let rm = Self::extract_rm(first_byte, second_byte, byte_stream).into();
        Inst::with_operand(op, rm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{Operand, Register};

    const DECODER: RM = RM;

    #[test]
    fn reg() {
        let bytes: [u8; 2] = [0b11111111, 0b11110011];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Push
            ),
            Inst::with_operand(Operation::Push, Operand::Register(Register::BX))
        )
    }
}

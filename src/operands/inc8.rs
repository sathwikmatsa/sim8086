use crate::{
    disasm::WithInc8,
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
    ByteStream,
};

#[derive(Default)]
pub struct Inc8;

impl WithInc8 for Inc8 {}

impl InstructionDecoder for Inc8 {
    fn decode(&self, first_byte: u8, byte_stream: &mut ByteStream, op: Operation) -> Inst {
        let inc8 = Self::extract_inc8(first_byte, byte_stream).into();
        Inst::with_operand(op, inc8)
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{Inc, Operand};

    use super::*;

    const DECODER: Inc8 = Inc8;

    #[test]
    fn je() {
        let bytes: [u8; 2] = [0b01110100, 0b11111110];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::JE
            ),
            Inst::with_operand(Operation::JE, Operand::Increment(Inc::I8(-2)))
        );
    }
}

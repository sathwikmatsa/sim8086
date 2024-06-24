use crate::{
    disasm::WithInc16,
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
    ByteStream,
};

#[derive(Default)]
pub struct Inc16;

impl WithInc16 for Inc16 {}

impl InstructionDecoder for Inc16 {
    fn decode(&self, _first_byte: u8, byte_stream: &mut ByteStream, op: Operation) -> Inst {
        let inc16 = Self::extract_inc16(byte_stream).into();
        Inst::with_operand(op, inc16)
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{Inc, Operand};

    use super::*;

    const DECODER: Inc16 = Inc16;

    #[test]
    fn call() {
        let bytes: [u8; 3] = [0b11101000, 0b00000000, 0b00000001];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Call
            ),
            Inst::with_operand(Operation::Call, Operand::Increment(Inc::I16(256)))
        );
    }
}

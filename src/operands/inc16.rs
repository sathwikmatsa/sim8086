use crate::{
    extractors::WithInc16,
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct Inc16;

impl WithInc16 for Inc16 {}

impl InstructionDecoder for Inc16 {
    fn decode(
        &self,
        _first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let inc16 = Self::extract_inc16(byte_stream).into();
        Inst {
            operation: op,
            first: Some(inc16),
            second: None,
        }
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
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Call),
            Inst {
                operation: Operation::Call,
                first: Some(Operand::Increment(Inc::I16(256))),
                second: None
            }
        );
    }
}

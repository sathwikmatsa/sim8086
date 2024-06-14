use crate::{
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct NoOps2;

impl InstructionDecoder for NoOps2 {
    fn decode(
        &self,
        _first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<u8>>,
        op: Operation,
    ) -> Inst {
        byte_stream.next();
        Inst {
            operation: op,
            first: None,
            second: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DECODER: NoOps2 = NoOps2;

    #[test]
    fn aam() {
        let bytes: [u8; 2] = [0b11010100, 0b00001010];
        let mut stream = bytes[1..].iter().peekable();
        assert_eq!(
            DECODER.decode(bytes[0], &mut stream, Operation::AAM),
            Inst {
                operation: Operation::AAM,
                second: None,
                first: None
            }
        );
        assert!(stream.next().is_none())
    }
}
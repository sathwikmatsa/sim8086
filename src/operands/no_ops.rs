use crate::{
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
    ByteStream,
};

#[derive(Default)]
pub struct NoOps;

impl InstructionDecoder for NoOps {
    fn decode(&self, _first_byte: u8, _byte_stream: &mut ByteStream, op: Operation) -> Inst {
        Inst::new(op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DECODER: NoOps = NoOps;

    #[test]
    fn xlat() {
        let bytes: [u8; 1] = [0b11010111];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::XLAT
            ),
            Inst::new(Operation::XLAT)
        );
    }
}

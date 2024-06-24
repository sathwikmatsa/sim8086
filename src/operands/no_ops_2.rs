use crate::{
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
    ByteStream,
};

#[derive(Default)]
pub struct NoOps2;

impl InstructionDecoder for NoOps2 {
    fn decode(&self, _first_byte: u8, byte_stream: &mut ByteStream, op: Operation) -> Inst {
        byte_stream.next();
        Inst::new(op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DECODER: NoOps2 = NoOps2;

    #[test]
    fn aam() {
        let bytes: [u8; 2] = [0b11010100, 0b00001010];
        let mut stream = ByteStream::new(bytes[1..].iter());
        assert_eq!(
            DECODER.decode(bytes[0], &mut stream, Operation::AAM),
            Inst::new(Operation::AAM)
        );
        assert!(stream.next().is_none())
    }
}

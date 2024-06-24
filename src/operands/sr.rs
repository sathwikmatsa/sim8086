use crate::{
    disasm::WithSR,
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
    ByteStream,
};

#[derive(Default)]
pub struct SR;

impl WithSR for SR {
    const SR_RIGHT_SHIFT_BY: u8 = 3;
}

impl InstructionDecoder for SR {
    fn decode(&self, first_byte: u8, _byte_stream: &mut ByteStream, op: Operation) -> Inst {
        let sr = Self::extract_sr(first_byte).into();
        Inst::with_operand(op, sr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{Operand, SegmentRegister};

    const DECODER: SR = SR;

    #[test]
    fn sr() {
        let bytes: [u8; 1] = [0b00011110];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Push
            ),
            Inst::with_operand(Operation::Push, Operand::SR(SegmentRegister::DS))
        )
    }
}

use crate::{
    extractors::WithSR,
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct SR;

impl WithSR for SR {
    const SR_RIGHT_SHIFT_BY: u8 = 3;
}

impl InstructionDecoder for SR {
    fn decode(
        &self,
        first_byte: u8,
        _byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let sr = Self::extract_sr(first_byte).into();
        Inst {
            operation: op,
            first: Some(sr),
            second: None,
        }
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
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Push),
            Inst {
                first: Some(Operand::SR(SegmentRegister::DS)),
                second: None,
                operation: Operation::Push
            }
        )
    }
}

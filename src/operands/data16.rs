use crate::{
    disasm::WithData16,
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct Data16;

impl WithData16 for Data16 {}

impl InstructionDecoder for Data16 {
    fn decode(
        &self,
        _first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let data16 = Self::extract_data16(byte_stream).into();
        Inst::with_operand(op, data16)
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{Data, Operand};

    use super::*;

    const DECODER: Data16 = Data16;

    #[test]
    fn ret() {
        let bytes: [u8; 3] = [0b11000010, 0b00000000, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Ret),
            Inst::with_operand(Operation::Ret, Operand::Immediate(Data::U16(256)))
        );
    }
}

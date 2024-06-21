use crate::{
    disasm::WithData8,
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct Data8;

impl WithData8 for Data8 {}

impl InstructionDecoder for Data8 {
    fn decode(
        &self,
        _first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let data8 = Self::extract_data8(byte_stream).into();
        Inst::with_operand(op, data8)
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{Data, Operand};

    use super::*;

    const DECODER: Data8 = Data8;

    #[test]
    fn int() {
        let bytes: [u8; 2] = [0b11001101, 0b0000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::INT),
            Inst::with_operand(Operation::INT, Operand::Immediate(Data::U8(1)))
        );
    }
}

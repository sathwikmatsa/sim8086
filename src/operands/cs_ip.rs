use crate::{
    extractors::WithCsIp,
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct CsIp;

impl WithCsIp for CsIp {}

impl InstructionDecoder for CsIp {
    fn decode(
        &self,
        _first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let cs_ip = Self::extract_cs_ip(byte_stream).into();
        Inst::with_operand(op, cs_ip)
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{CsIp as CsIpField, Operand};

    use super::*;

    const DECODER: CsIp = CsIp;

    #[test]
    fn call() {
        let bytes: [u8; 5] = [0b10011010, 0b11001000, 0b00000001, 0b01111011, 0b00000000];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Call),
            Inst::with_operand(
                Operation::Call,
                Operand::CsIp(CsIpField {
                    code_segment: 123,
                    instruction_pointer: 456
                })
            )
        );
    }
}

use crate::{
    disasm::{WithData, WithWideField},
    fields::{Data, EffectiveAddress, Operation, Register},
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct AccDA;

impl WithWideField for AccDA {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl WithData for AccDA {}

impl InstructionDecoder for AccDA {
    fn decode(
        &self,
        first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let wide = Self::is_wide(first_byte);
        let ws = Self::get_wide_size(first_byte);
        let data = Self::extract_data(first_byte, byte_stream);
        let acc = if wide { Register::AX } else { Register::AL }.into();
        let direct_address = EffectiveAddress::DirectAddress(
            match data {
                Data::U16(x) => x,
                Data::U8(x) => x.into(),
            },
            ws,
        )
        .into();
        Inst::with_operands(op, acc, direct_address)
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{Operand, Wide};

    use super::*;

    const DECODER: AccDA = AccDA;

    #[test]
    fn not_wide() {
        let bytes: [u8; 2] = [0b10100000, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Mov),
            Inst::with_operands(
                Operation::Mov,
                Operand::Register(Register::AL),
                Operand::EffectiveAddress(EffectiveAddress::DirectAddress(1, Wide::Byte))
            )
        );
    }

    #[test]
    fn wide() {
        let bytes: [u8; 3] = [0b10100001, 0b00000000, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Mov),
            Inst::with_operands(
                Operation::Mov,
                Operand::Register(Register::AX),
                Operand::EffectiveAddress(EffectiveAddress::DirectAddress(256, Wide::Word))
            )
        );
    }
}

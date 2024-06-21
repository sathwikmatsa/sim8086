use crate::{
    disasm::{WithData, WithWideField},
    fields::{Data, EffectiveAddress, Operation, Register},
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct DAAcc;

impl WithWideField for DAAcc {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl WithData for DAAcc {}

impl InstructionDecoder for DAAcc {
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
        Inst::with_operands(op, direct_address, acc)
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{Operand, Wide};

    use super::*;

    const DECODER: DAAcc = DAAcc;

    #[test]
    fn not_wide() {
        let bytes: [u8; 2] = [0b10100010, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Mov),
            Inst::with_operands(
                Operation::Mov,
                Operand::EffectiveAddress(EffectiveAddress::DirectAddress(1, Wide::Byte)),
                Operand::Register(Register::AL)
            )
        );
    }

    #[test]
    fn wide() {
        let bytes: [u8; 3] = [0b10100011, 0b00000000, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Mov),
            Inst::with_operands(
                Operation::Mov,
                Operand::EffectiveAddress(EffectiveAddress::DirectAddress(256, Wide::Word)),
                Operand::Register(Register::AX),
            )
        );
    }

    #[test]
    fn print() {
        let ins = Inst::with_operands(
            Operation::Mov,
            Operand::EffectiveAddress(EffectiveAddress::DirectAddress(256, Wide::Word)),
            Operand::Register(Register::AX),
        );
        assert_eq!(format!("{}", ins), "mov word [256], ax");
    }
}

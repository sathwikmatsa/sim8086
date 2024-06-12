use crate::{
    extractors::{WithData, WithWideField},
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
        let data = Self::extract_data(first_byte, byte_stream);
        let acc = if wide { Register::AX } else { Register::AL }.into();
        let direct_address = EffectiveAddress::DirectAddress(match data {
            Data::U16(x) => x,
            Data::U8(x) => x.into(),
        })
        .into();
        Inst {
            operation: op,
            first: Some(direct_address),
            second: Some(acc),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::Operand;

    use super::*;

    const DECODER: DAAcc = DAAcc;

    #[test]
    fn not_wide() {
        let bytes: [u8; 2] = [0b10100010, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Mov),
            Inst {
                operation: Operation::Mov,
                second: Some(Operand::Register(Register::AL)),
                first: Some(Operand::EffectiveAddress(EffectiveAddress::DirectAddress(
                    1
                )))
            }
        );
    }

    #[test]
    fn wide() {
        let bytes: [u8; 3] = [0b10100011, 0b00000000, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Mov),
            Inst {
                operation: Operation::Mov,
                second: Some(Operand::Register(Register::AX)),
                first: Some(Operand::EffectiveAddress(EffectiveAddress::DirectAddress(
                    256
                )))
            }
        );
    }

    #[test]
    fn print() {
        let ins = Inst {
            operation: Operation::Mov,
            second: Some(Operand::Register(Register::AX)),
            first: Some(Operand::EffectiveAddress(EffectiveAddress::DirectAddress(
                256,
            ))),
        };
        assert_eq!(format!("{}", ins), "mov [256], ax");
    }
}

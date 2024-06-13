use crate::{
    extractors::WithWideField,
    fields::{Data, Operation, Register},
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct FixedPort;

impl WithWideField for FixedPort {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl InstructionDecoder for FixedPort {
    fn decode(
        &self,
        first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<u8>>,
        op: Operation,
    ) -> Inst {
        let wide = Self::is_wide(first_byte);
        let acc = if wide { Register::AX } else { Register::AL }.into();
        let data8 = Data::U8(byte_stream.next().expect("extract second byte").to_owned()).into();
        Inst {
            operation: op,
            first: Some(acc),
            second: Some(data8),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::Operand;

    use super::*;

    const DECODER: FixedPort = FixedPort;

    #[test]
    fn wide() {
        let bytes: [u8; 2] = [0b11100101, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::IN),
            Inst {
                operation: Operation::IN,
                second: Some(Operand::Immediate(Data::U8(1))),
                first: Some(Operand::Register(Register::AX))
            }
        );
    }

    #[test]
    fn not_wide() {
        let bytes: [u8; 2] = [0b11100110, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::OUT),
            Inst {
                operation: Operation::OUT,
                second: Some(Operand::Immediate(Data::U8(1))),
                first: Some(Operand::Register(Register::AL))
            }
        );
    }
}

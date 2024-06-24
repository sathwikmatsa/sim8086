use crate::{
    disasm::WithWideField,
    fields::{Data, Operation, Register},
    instruction::{Inst, InstructionDecoder},
    ByteStream,
};

#[derive(Default)]
pub struct FixedPort;

impl WithWideField for FixedPort {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl InstructionDecoder for FixedPort {
    fn decode(&self, first_byte: u8, byte_stream: &mut ByteStream, op: Operation) -> Inst {
        let wide = Self::is_wide(first_byte);
        let acc = if wide { Register::AX } else { Register::AL }.into();
        let data8 = Data::U8(byte_stream.next().expect("extract second byte").to_owned()).into();
        Inst::with_operands(
            op,
            if op == Operation::IN { acc } else { data8 },
            if op == Operation::IN { data8 } else { acc },
        )
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
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::IN
            ),
            Inst::with_operands(
                Operation::IN,
                Operand::Register(Register::AX),
                Operand::Immediate(Data::U8(1)),
            )
        );
    }

    #[test]
    fn not_wide() {
        let bytes: [u8; 2] = [0b11100110, 0b00000001];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::OUT
            ),
            Inst::with_operands(
                Operation::OUT,
                Operand::Immediate(Data::U8(1)),
                Operand::Register(Register::AL),
            )
        );
    }
}

use crate::{
    disasm::WithWideField,
    fields::{Operand, Operation, Register},
    instruction::{Inst, InstructionDecoder},
    ByteStream,
};

#[derive(Default)]
pub struct VariablePort;

impl WithWideField for VariablePort {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl InstructionDecoder for VariablePort {
    fn decode(&self, first_byte: u8, _byte_stream: &mut ByteStream, op: Operation) -> Inst {
        let wide = Self::is_wide(first_byte);
        let acc = if wide { Register::AX } else { Register::AL }.into();
        let dx = Operand::Register(Register::DX);
        Inst::with_operands(
            op,
            if op == Operation::IN { acc } else { dx },
            if op == Operation::IN { dx } else { acc },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DECODER: VariablePort = VariablePort;

    #[test]
    fn wide() {
        let bytes: [u8; 1] = [0b11101101];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::IN
            ),
            Inst::with_operands(
                Operation::IN,
                Operand::Register(Register::AX),
                Operand::Register(Register::DX)
            )
        );
    }

    #[test]
    fn not_wide() {
        let bytes: [u8; 1] = [0b11101110];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::OUT
            ),
            Inst::with_operands(
                Operation::OUT,
                Operand::Register(Register::DX),
                Operand::Register(Register::AL)
            )
        );
    }
}

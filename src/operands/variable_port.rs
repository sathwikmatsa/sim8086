use crate::{
    extractors::WithWideField,
    fields::{Operand, Operation, Register},
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct VariablePort;

impl WithWideField for VariablePort {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl InstructionDecoder for VariablePort {
    fn decode(
        &self,
        first_byte: u8,
        _byte_stream: &mut std::iter::Peekable<std::slice::Iter<u8>>,
        op: Operation,
    ) -> Inst {
        let wide = Self::is_wide(first_byte);
        let acc = if wide { Register::AX } else { Register::AL }.into();
        let dx = Operand::Register(Register::DX);
        Inst {
            operation: op,
            first: if op == Operation::IN {
                Some(acc)
            } else {
                Some(dx)
            },
            second: if op == Operation::IN {
                Some(dx)
            } else {
                Some(acc)
            },
        }
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
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::IN),
            Inst {
                operation: Operation::IN,
                second: Some(Operand::Register(Register::DX)),
                first: Some(Operand::Register(Register::AX))
            }
        );
    }

    #[test]
    fn not_wide() {
        let bytes: [u8; 1] = [0b11101110];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::OUT),
            Inst {
                operation: Operation::OUT,
                second: Some(Operand::Register(Register::DX)),
                first: Some(Operand::Register(Register::AL))
            }
        );
    }
}

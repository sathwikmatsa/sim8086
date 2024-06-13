use crate::{
    extractors::{WithDestField, WithRMField, WithSR, WithWideField},
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct SRRM;

impl WithRMField for SRRM {}

impl WithWideField for SRRM {
    // wide bit is not present as this op inherently involves 16-bit registers
    const WIDE_MASK_MATCH: u8 = 0;
}

impl WithSR for SRRM {
    const SR_RIGHT_SHIFT_BY: u8 = 3;
}

impl WithDestField for SRRM {
    const DEST_MASK_MATCH: u8 = 0b00000010;
}

impl InstructionDecoder for SRRM {
    fn decode(
        &self,
        first_byte: u8,
        byte_stream: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        op: Operation,
    ) -> Inst {
        let second_byte = byte_stream
            .next()
            .expect("extract second instruction byte")
            .to_owned();

        let sr = Self::extract_sr(second_byte).into();
        let is_sr_dest = Self::is_dest_in_reg_field(first_byte);
        let rm = Self::extract_rm(first_byte, second_byte, byte_stream).into();
        Inst {
            operation: op,
            first: if is_sr_dest { Some(sr) } else { Some(rm) },
            second: if is_sr_dest { Some(rm) } else { Some(sr) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{Operand, Register, SegmentRegister};

    const DECODER: SRRM = SRRM;

    #[test]
    fn reg_to_sr() {
        let bytes: [u8; 2] = [0b10001110, 0b11000011];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Mov),
            Inst {
                second: Some(Operand::Register(Register::BX)),
                first: Some(Operand::SR(SegmentRegister::ES)),
                operation: Operation::Mov
            }
        )
    }

    #[test]
    fn sr_to_reg() {
        let bytes: [u8; 2] = [0b10001100, 0b11000011];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter().peekable(), Operation::Mov),
            Inst {
                first: Some(Operand::Register(Register::BX)),
                second: Some(Operand::SR(SegmentRegister::ES)),
                operation: Operation::Mov
            }
        )
    }
}

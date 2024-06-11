use crate::{
    extractors::{WithData, WithRMField, WithWideField},
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
};

#[derive(Default)]
pub struct RMImd;

impl WithRMField for RMImd {}

impl WithData for RMImd {}

impl WithWideField for RMImd {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl InstructionDecoder for RMImd {
    fn decode(
        &self,
        first_byte: u8,
        byte_stream: &mut std::slice::Iter<u8>,
        op: Operation,
    ) -> Inst {
        let second_byte = byte_stream
            .next()
            .expect("extract second instruction byte")
            .to_owned();
        let rm = Self::extract_rm(first_byte, second_byte, byte_stream).into();
        let data = Self::extract_data(first_byte, byte_stream).into();
        Inst {
            operation: op,
            first: Some(rm),
            second: Some(data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{Data, EffectiveAddress, Operand, Register};

    const DECODER: RMImd = RMImd;

    #[test]
    fn immediate_to_register_wide() {
        let bytes: [u8; 4] = [0b11000111, 0b11000011, 0b00000100, 0b00000001];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter(), Operation::Mov),
            Inst {
                operation: Operation::Mov,
                first: Some(Operand::Register(Register::BX)),
                second: Some(Operand::Immediate(Data::U16(260)))
            }
        )
    }

    #[test]
    fn immediate_to_register_not_wide() {
        let bytes: [u8; 3] = [0b11000110, 0b11000011, 0b00000100];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter(), Operation::Mov),
            Inst {
                operation: Operation::Mov,
                first: Some(Operand::Register(Register::BL)),
                second: Some(Operand::Immediate(Data::U8(4)))
            }
        )
    }

    #[test]
    fn immediate_to_mem_no_disp() {
        let bytes: [u8; 3] = [0b11000110, 0b00000011, 0b00000101];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter(), Operation::Mov),
            Inst {
                operation: Operation::Mov,
                first: Some(Operand::EffectiveAddress(EffectiveAddress::BP_DI(None))),
                second: Some(Operand::Immediate(Data::U8(5)))
            }
        )
    }

    #[test]
    fn immediate_to_mem_direct_address() {
        let bytes: [u8; 6] = [
            0b11000111, 0b00000110, 0b00000100, 0b00000000, 0b00000000, 0b00000001,
        ];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter(), Operation::Mov),
            Inst {
                operation: Operation::Mov,
                first: Some(Operand::EffectiveAddress(EffectiveAddress::DirectAddress(
                    4
                ))),
                second: Some(Operand::Immediate(Data::U16(256)))
            }
        )
    }

    #[test]
    fn immediate_to_mem_8bit_disp() {
        let bytes: [u8; 4] = [0b11000110, 0b01000110, 0b00000100, 0b00000000];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter(), Operation::Mov),
            Inst {
                operation: Operation::Mov,
                first: Some(Operand::EffectiveAddress(EffectiveAddress::BP(4))),
                second: Some(Operand::Immediate(Data::U8(0)))
            }
        )
    }

    #[test]
    fn immediate_to_mem_16bit_disp() {
        let bytes: [u8; 6] = [
            0b11000111, 0b10000100, 0b00000100, 0b00000000, 0b00000000, 0b00000001,
        ];
        assert_eq!(
            DECODER.decode(bytes[0], &mut bytes[1..].iter(), Operation::Mov),
            Inst {
                operation: Operation::Mov,
                first: Some(Operand::EffectiveAddress(EffectiveAddress::SI(Some(4)))),
                second: Some(Operand::Immediate(Data::U16(256)))
            }
        )
    }
}
use crate::{
    disasm::{WithDestField, WithRMField, WithRegField, WithWideField},
    fields::Operation,
    instruction::{Inst, InstructionDecoder},
    ByteStream,
};

#[derive(Default)]
pub struct RegRM;

impl WithRMField for RegRM {}

impl WithRegField for RegRM {
    const RIGHT_SHIFT_BY: u8 = 3;
}

impl WithDestField for RegRM {
    const DEST_MASK_MATCH: u8 = 0b00000010;
}

impl WithWideField for RegRM {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl InstructionDecoder for RegRM {
    fn decode(&self, first_byte: u8, byte_stream: &mut ByteStream, op: Operation) -> Inst {
        let second_byte = byte_stream
            .next()
            .expect("extract second instruction byte")
            .to_owned();

        let reg = Self::extract_reg(first_byte, second_byte).into();
        let is_reg_dest = Self::is_dest_in_reg_field(first_byte);
        let rm = Self::extract_rm(first_byte, second_byte, byte_stream).into();
        Inst::with_operands(
            op,
            if is_reg_dest { reg } else { rm },
            if is_reg_dest { rm } else { reg },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::{EffectiveAddress, Operand, Register, Wide};

    const DECODER: RegRM = RegRM;

    #[test]
    fn reg_to_reg_wide() {
        let bytes: [u8; 2] = [0b10001001, 0b11000011];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Mov
            ),
            Inst::with_operands(
                Operation::Mov,
                Operand::Register(Register::BX),
                Operand::Register(Register::AX)
            )
        )
    }

    #[test]
    fn reg_to_reg_not_wide() {
        let bytes: [u8; 2] = [0b10001000, 0b11000011];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Mov
            ),
            Inst::with_operands(
                Operation::Mov,
                Operand::Register(Register::BL),
                Operand::Register(Register::AL)
            )
        )
    }

    #[test]
    fn reg_to_mem_no_disp() {
        let bytes: [u8; 2] = [0b10001000, 0b00010011];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Mov
            ),
            Inst::with_operands(
                Operation::Mov,
                Operand::EffectiveAddress(EffectiveAddress::BP_DI(None, Wide::Byte)),
                Operand::Register(Register::DL)
            )
        )
    }

    #[test]
    fn reg_to_mem_direct_address() {
        let bytes: [u8; 4] = [0b10001001, 0b00010110, 0b00000001, 0b00000000];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Mov
            ),
            Inst::with_operands(
                Operation::Mov,
                Operand::EffectiveAddress(EffectiveAddress::DirectAddress(1, Wide::Word)),
                Operand::Register(Register::DX)
            )
        )
    }

    #[test]
    fn reg_to_mem_8bit_disp() {
        let bytes: [u8; 3] = [0b10001000, 0b01001110, 0b00000010];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Mov
            ),
            Inst::with_operands(
                Operation::Mov,
                Operand::EffectiveAddress(EffectiveAddress::BP(2, Wide::Byte)),
                Operand::Register(Register::CL)
            )
        )
    }

    #[test]
    fn reg_to_mem_16bit_disp() {
        let bytes: [u8; 4] = [0b10001001, 0b10011000, 0b00000000, 0b00000001];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Mov
            ),
            Inst::with_operands(
                Operation::Mov,
                Operand::EffectiveAddress(EffectiveAddress::BX_SI(Some(256), Wide::Word)),
                Operand::Register(Register::BX)
            )
        )
    }

    #[test]
    fn mem_16bit_disp_to_reg() {
        let bytes: [u8; 4] = [0b10001011, 0b10011000, 0b00000000, 0b00000001];
        assert_eq!(
            DECODER.decode(
                bytes[0],
                &mut ByteStream::new(bytes[1..].iter()),
                Operation::Mov
            ),
            Inst::with_operands(
                Operation::Mov,
                Operand::Register(Register::BX),
                Operand::EffectiveAddress(EffectiveAddress::BX_SI(Some(256), Wide::Word))
            )
        )
    }
}

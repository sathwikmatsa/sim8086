use crate::fields::{Data, EffectiveAddress, Register, RM};

pub trait WithWideField {
    const WIDE_MASK_MATCH: u8;

    fn is_wide(first_byte: u8) -> bool {
        (first_byte & Self::WIDE_MASK_MATCH) == Self::WIDE_MASK_MATCH
    }
}

pub trait WithDestField: WithRegField {
    const DEST_MASK_MATCH: u8;

    fn is_dest_in_reg_field(first_byte: u8) -> bool {
        (first_byte & Self::DEST_MASK_MATCH) == Self::DEST_MASK_MATCH
    }
}

pub trait WithRMField: WithWideField {
    const MOD_RIGHT_SHIFT_BY: u8 = 6;
    const RM_RIGHT_SHIFT_BY: u8 = 0;

    fn extract_mod(second_byte: u8) -> u8 {
        const MOD_MASK: u8 = 0b00000011;
        (second_byte >> Self::MOD_RIGHT_SHIFT_BY) & MOD_MASK
    }

    fn extract_rm_bin(second_byte: u8) -> u8 {
        const RM_MASK: u8 = 0b00000111;
        (second_byte >> Self::RM_RIGHT_SHIFT_BY) & RM_MASK
    }

    fn extract_disp<'a, I>(modf: u8, rm: u8, byte_stream: &mut I) -> Option<u16>
    where
        I: Iterator<Item = &'a u8>,
    {
        if modf == 0b01 {
            let data = byte_stream.next().expect("extract disp-low").to_owned();
            let sign_extended = ((data as i8) as i16) as u16;
            Some(sign_extended)
        } else if modf == 0b10 || (modf == 0b00 && rm == 0b110) {
            let data_low = byte_stream.next().expect("extract disp-low").to_owned();
            let data_high = byte_stream.next().expect("extract disp-high").to_owned();
            let data: u16 = ((data_high as u16) << 8) | (data_low as u16);
            Some(data)
        } else {
            None
        }
    }

    fn extract_rm<'a, I>(first_byte: u8, second_byte: u8, byte_stream: &mut I) -> RM
    where
        I: Iterator<Item = &'a u8>,
    {
        let wide = Self::is_wide(first_byte);
        let modf = Self::extract_mod(second_byte);
        let rm = Self::extract_rm_bin(second_byte);
        let disp = Self::extract_disp(modf, rm, byte_stream);
        if modf == 0b11 {
            RM::Reg(register_from_u8(rm, wide))
        } else if modf == 0b00 && rm == 0b110 {
            RM::Mem(EffectiveAddress::DirectAddress(
                disp.expect("direct address 16bit displacement"),
            ))
        } else {
            match rm {
                0b000 => RM::Mem(EffectiveAddress::BX_SI(disp)),
                0b001 => RM::Mem(EffectiveAddress::BX_DI(disp)),
                0b010 => RM::Mem(EffectiveAddress::BP_SI(disp)),
                0b011 => RM::Mem(EffectiveAddress::BP_DI(disp)),
                0b100 => RM::Mem(EffectiveAddress::SI(disp)),
                0b101 => RM::Mem(EffectiveAddress::DI(disp)),
                0b110 => RM::Mem(EffectiveAddress::BP(disp.expect("disp"))),
                0b111 => RM::Mem(EffectiveAddress::BX(disp)),
                _ => unreachable!(),
            }
        }
    }
}

#[rustfmt::skip]
fn register_from_u8(reg: u8, wide: bool) -> Register {
    match reg {
        0b000 => if wide { Register::AX } else { Register::AL },
        0b001 => if wide { Register::CX } else { Register::CL },
        0b010 => if wide { Register::DX } else { Register::DL },
        0b011 => if wide { Register::BX } else { Register::BL },
        0b100 => if wide { Register::SP } else { Register::AH },
        0b101 => if wide { Register::BP } else { Register::CH },
        0b110 => if wide { Register::SI } else { Register::DH },
        0b111 => if wide { Register::DI } else { Register::BH },
        _ => unreachable!()
    }
}

pub trait WithRegField: WithWideField {
    const RIGHT_SHIFT_BY: u8;

    fn extract_reg(first_byte: u8, reg_byte: u8) -> Register {
        let wide = Self::is_wide(first_byte);
        let reg_byte = reg_byte >> Self::RIGHT_SHIFT_BY;
        const REG_MASK: u8 = 0b00000111;
        let reg = reg_byte & REG_MASK;
        register_from_u8(reg, wide)
    }
}

pub trait WithData: WithWideField {
    fn extract_data<'a, I>(first_byte: u8, byte_stream: &mut I) -> Data
    where
        I: Iterator<Item = &'a u8>,
    {
        let wide = Self::is_wide(first_byte);
        if wide {
            let data_low = byte_stream.next().expect("extract data-low").to_owned();
            let data_high = byte_stream.next().expect("extract data-high").to_owned();
            let data: u16 = ((data_high as u16) << 8) | (data_low as u16);
            Data::U16(data)
        } else {
            let data = byte_stream.next().expect("extract data-8").to_owned();
            Data::U8(data)
        }
    }
}
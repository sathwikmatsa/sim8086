use crate::fields::{
    register_from_u8, sr_from_u8, Data, EffectiveAddress, Inc, Register, SegmentRegister, Wide, RM,
};

pub trait WithSignField {
    const SIGN_MASK_MATCH: u8 = 0b00000010;

    fn sign_extend(first_byte: u8) -> bool {
        (first_byte & Self::SIGN_MASK_MATCH) == Self::SIGN_MASK_MATCH
    }
}

pub trait WithWideField {
    const WIDE_MASK_MATCH: u8;

    fn is_wide(first_byte: u8) -> bool {
        (first_byte & Self::WIDE_MASK_MATCH) == Self::WIDE_MASK_MATCH
    }

    fn get_wide_size(first_byte: u8) -> Wide {
        if Self::WIDE_MASK_MATCH == 0 {
            Wide::None
        } else {
            Self::is_wide(first_byte).into()
        }
    }
}

pub trait WithDestField {
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
        let ws = Self::get_wide_size(first_byte);
        let modf = Self::extract_mod(second_byte);
        let rm = Self::extract_rm_bin(second_byte);
        let disp = Self::extract_disp(modf, rm, byte_stream);
        if modf == 0b11 {
            RM::Reg(register_from_u8(rm, wide))
        } else if modf == 0b00 && rm == 0b110 {
            RM::Mem(EffectiveAddress::DirectAddress(
                disp.expect("direct address 16bit displacement"),
                ws,
            ))
        } else {
            match rm {
                0b000 => RM::Mem(EffectiveAddress::BX_SI(disp, ws)),
                0b001 => RM::Mem(EffectiveAddress::BX_DI(disp, ws)),
                0b010 => RM::Mem(EffectiveAddress::BP_SI(disp, ws)),
                0b011 => RM::Mem(EffectiveAddress::BP_DI(disp, ws)),
                0b100 => RM::Mem(EffectiveAddress::SI(disp, ws)),
                0b101 => RM::Mem(EffectiveAddress::DI(disp, ws)),
                0b110 => RM::Mem(EffectiveAddress::BP(disp.expect("disp"), ws)),
                0b111 => RM::Mem(EffectiveAddress::BX(disp, ws)),
                _ => unreachable!(),
            }
        }
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

pub trait WithDataS: WithWideField + WithSignField {
    fn extract_data<'a, I>(first_byte: u8, byte_stream: &mut I) -> Data
    where
        I: Iterator<Item = &'a u8>,
    {
        let wide = Self::is_wide(first_byte);
        let sign = Self::sign_extend(first_byte);
        if !sign {
            if wide {
                let data_low = byte_stream.next().expect("extract data-low").to_owned();
                let data_high = byte_stream.next().expect("extract data-high").to_owned();
                let data: u16 = ((data_high as u16) << 8) | (data_low as u16);
                Data::U16(data)
            } else {
                let data = byte_stream.next().expect("extract data-8").to_owned();
                Data::U8(data)
            }
        } else {
            let data = byte_stream.next().expect("extract data").to_owned();
            let sign_extended = ((data as i8) as i16) as u16;
            Data::U16(sign_extended)
        }
    }
}

pub trait WithInc8 {
    fn extract_inc8<'a, I>(_first_byte: u8, byte_stream: &mut I) -> Inc
    where
        I: Iterator<Item = &'a u8>,
    {
        let data = byte_stream.next().expect("extract inc-8").to_owned();
        Inc::I8(data as i8)
    }
}

pub trait WithSR {
    const SR_RIGHT_SHIFT_BY: u8;

    fn extract_sr(sr_byte: u8) -> SegmentRegister {
        sr_from_u8((sr_byte >> Self::SR_RIGHT_SHIFT_BY) & 0b00000011)
    }
}

pub trait WithVField {
    const V_MASK_MATCH: u8;

    fn is_v_set(v_byte: u8) -> bool {
        (v_byte & Self::V_MASK_MATCH) == Self::V_MASK_MATCH
    }
}

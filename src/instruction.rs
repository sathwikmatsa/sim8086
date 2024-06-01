use enum_stringify::EnumStringify;
use std::fmt;

pub trait Instruction: fmt::Display {
    fn opcode_mask() -> u8 where Self: Sized;
    fn opcode_match() -> u8 where Self: Sized;

    fn match_opcode(first_byte: u8) -> bool
    where
        Self: Sized,
    {
        return (first_byte & Self::opcode_mask()) == Self::opcode_match();
    }

    fn new<'a, I>(first_byte: u8, byte_stream: &mut I) -> Self
    where
        Self: Sized,
        I: Iterator<Item = &'a u8>;
}

pub trait WithWideField {
    const WIDE_MASK_MATCH: u8;

    fn is_wide(first_byte: u8) -> bool {
        return (first_byte & Self::WIDE_MASK_MATCH) == Self::WIDE_MASK_MATCH;
    }
}

pub trait WithDestField: WithRegField {
    const DEST_MASK_MATCH: u8;

    fn is_dest_in_reg_field(first_byte: u8) -> bool {
        return (first_byte & Self::DEST_MASK_MATCH) == Self::DEST_MASK_MATCH;
    }
}

pub trait WithRMField: WithWideField {
    const MOD_RIGHT_SHIFT_BY: u8 = 6;
    const RM_RIGHT_SHIFT_BY: u8 = 0;

    fn extract_mod(second_byte: u8) -> u8 {
        const MOD_MASK: u8 = 0b00000011;
        return (second_byte >> Self::MOD_RIGHT_SHIFT_BY) & MOD_MASK;
    }

    fn extract_rm_bin(second_byte: u8) -> u8 {
        const RM_MASK: u8 = 0b00000111;
        return (second_byte >> Self::RM_RIGHT_SHIFT_BY) & RM_MASK;
    }

    fn extract_disp<'a, I>(modf: u8, rm: u8, mut byte_stream: I) -> Option<Data>
    where
        I: Iterator<Item = &'a u8>,
    {
        if modf == 0b01 {
            let data = byte_stream.next().expect("extract disp-low").to_owned();
            Some(Data::U8(data))
        } else if modf == 0b10 || (modf == 0b00 && rm == 0b110) {
            let data_low = byte_stream.next().expect("extract disp-low").to_owned();
            let data_high = byte_stream.next().expect("extract disp-high").to_owned();
            let data: u16 = ((data_high as u16) << 8) | (data_low as u16);
            Some(Data::U16(data))
        } else {
            None
        }
    }

    fn extract_rm<'a, I>(first_byte: u8, second_byte: u8, byte_stream: I) -> RM
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
                match disp.expect("direct address 16bit displacement") {
                    Data::U8(_) => unreachable!(),
                    Data::U16(x) => x,
                },
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
    fn extract_data<'a, I>(first_byte: u8, mut byte_stream: I) -> Data
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

pub trait WithDisp: WithRMField {}

#[derive(Debug, PartialEq)]
pub enum Data {
    U8(u8),
    U16(u16),
}

impl From<&Data> for u16 {
    fn from(data: &Data) -> Self {
        match data {
            Data::U8(value) => value.to_owned() as u16,
            Data::U16(value) => value.to_owned(),
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u16::from(self))
    }
}

#[derive(EnumStringify, Debug, PartialEq)]
#[rustfmt::skip]
#[enum_stringify(case ="lower")]
pub enum Register {
    AX, BX, CX, DX,
    AH, BH, CH, DH,
    AL, BL, CL, DL,
    SP, BP, SI, DI,
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum EffectiveAddress {
    DirectAddress(u16),
    BX_SI(Option<Data>),
    BX_DI(Option<Data>),
    BP_SI(Option<Data>),
    BP_DI(Option<Data>),
    SI(Option<Data>),
    DI(Option<Data>),
    BP(Data),
    BX(Option<Data>),
}

impl fmt::Display for EffectiveAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EffectiveAddress::DirectAddress(x) => write!(f, "[{}]", x),
            EffectiveAddress::BX_SI(x) => match x {
                Some(y) => write!(f, "[bx + si + {}]", u16::from(y)),
                None => write!(f, "[bx + si]"),
            },
            EffectiveAddress::BX_DI(x) => match x {
                Some(y) => write!(f, "[bx + di + {}]", u16::from(y)),
                None => write!(f, "[bx + di]"),
            },
            EffectiveAddress::BP_SI(x) => match x {
                Some(y) => write!(f, "[bp + si + {}]", u16::from(y)),
                None => write!(f, "[bp + si]"),
            },
            EffectiveAddress::BP_DI(x) => match x {
                Some(y) => write!(f, "[bp + di + {}]", u16::from(y)),
                None => write!(f, "[bp + di]"),
            },
            EffectiveAddress::SI(x) => match x {
                Some(y) => write!(f, "[si + {}]", u16::from(y)),
                None => write!(f, "[si]"),
            },
            EffectiveAddress::DI(x) => match x {
                Some(y) => write!(f, "[di + {}]", u16::from(y)),
                None => write!(f, "[di]"),
            },
            EffectiveAddress::BP(x) => write!(f, "[bp + {}]", u16::from(x)),
            EffectiveAddress::BX(x) => match x {
                Some(y) => write!(f, "[bx + {}]", u16::from(y)),
                None => write!(f, "[bx]"),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RM {
    Reg(Register),
    Mem(EffectiveAddress),
}

impl fmt::Display for RM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RM::Reg(x) => x.to_string(),
                RM::Mem(x) => x.to_string(),
            }
        )
    }
}

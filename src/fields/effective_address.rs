use std::fmt::{self, Display};

use super::Operand;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Wide {
    Byte,
    Word,
    None,
}

impl Display for Wide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Byte => write!(f, "byte "),
            Self::Word => write!(f, "word "),
            Self::None => write!(f, ""),
        }
    }
}

impl From<bool> for Wide {
    fn from(value: bool) -> Self {
        if value {
            Wide::Word
        } else {
            Wide::Byte
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum EffectiveAddress {
    DirectAddress(u16, Wide),
    BX_SI(Option<u16>, Wide),
    BX_DI(Option<u16>, Wide),
    BP_SI(Option<u16>, Wide),
    BP_DI(Option<u16>, Wide),
    SI(Option<u16>, Wide),
    DI(Option<u16>, Wide),
    BP(u16, Wide),
    BX(Option<u16>, Wide),
}

impl From<EffectiveAddress> for Operand {
    fn from(val: EffectiveAddress) -> Self {
        Operand::EffectiveAddress(val)
    }
}

fn disp_str_repr(d: &u16) -> String {
    let i = *d as i16;
    format!("{}{}", if i < 0 { "" } else { "+ " }, i)
}

impl fmt::Display for EffectiveAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EffectiveAddress::DirectAddress(x, w) => write!(f, "{}[{}]", w, x),
            EffectiveAddress::BX_SI(x, w) => match x {
                Some(y) => write!(f, "{}[bx + si {}]", w, disp_str_repr(y)),
                None => write!(f, "{}[bx + si]", w),
            },
            EffectiveAddress::BX_DI(x, w) => match x {
                Some(y) => write!(f, "{}[bx + di {}]", w, disp_str_repr(y)),
                None => write!(f, "{}[bx + di]", w),
            },
            EffectiveAddress::BP_SI(x, w) => match x {
                Some(y) => write!(f, "{}[bp + si {}]", w, disp_str_repr(y)),
                None => write!(f, "{}[bp + si]", w),
            },
            EffectiveAddress::BP_DI(x, w) => match x {
                Some(y) => write!(f, "{}[bp + di {}]", w, disp_str_repr(y)),
                None => write!(f, "{}[bp + di]", w),
            },
            EffectiveAddress::SI(x, w) => match x {
                Some(y) => write!(f, "{}[si {}]", w, disp_str_repr(y)),
                None => write!(f, "{}[si]", w),
            },
            EffectiveAddress::DI(x, w) => match x {
                Some(y) => write!(f, "{}[di {}]", w, disp_str_repr(y)),
                None => write!(f, "{}[di]", w),
            },
            EffectiveAddress::BP(x, w) => write!(f, "{}[bp {}]", w, disp_str_repr(x)),
            EffectiveAddress::BX(x, w) => match x {
                Some(y) => write!(f, "{}[bx {}]", w, disp_str_repr(y)),
                None => write!(f, "{}[bx]", w),
            },
        }
    }
}

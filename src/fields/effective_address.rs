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

impl EffectiveAddress {
    pub fn wide(&self) -> Wide {
        match self {
            Self::DirectAddress(_, x) => *x,
            Self::BX_SI(_, x) => *x,
            Self::BX_DI(_, x) => *x,
            Self::BP_SI(_, x) => *x,
            Self::BP_DI(_, x) => *x,
            Self::SI(_, x) => *x,
            Self::DI(_, x) => *x,
            Self::BP(_, x) => *x,
            Self::BX(_, x) => *x,
        }
    }

    pub fn clocks(&self) -> usize {
        match self {
            Self::DirectAddress(_, _) => 6,
            Self::SI(None, _) => 5,
            Self::DI(None, _) => 5,
            Self::BP(0, _) => 5,
            Self::BX(None, _) => 5,
            Self::SI(Some(_), _) => 9,
            Self::DI(Some(_), _) => 9,
            Self::BP(_, _) => 9,
            Self::BX(Some(_), _) => 9,
            Self::BP_DI(None, _) => 7,
            Self::BX_SI(None, _) => 7,
            Self::BP_SI(None, _) => 8,
            Self::BX_DI(None, _) => 8,
            Self::BP_DI(Some(_), _) => 11,
            Self::BX_SI(Some(_), _) => 11,
            Self::BP_SI(Some(_), _) => 12,
            Self::BX_DI(Some(_), _) => 12,
        }
    }
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
            EffectiveAddress::DirectAddress(x, _) => write!(f, "[{}]", x),
            EffectiveAddress::BX_SI(x, _) => match x {
                Some(y) => write!(f, "[bx + si {}]", disp_str_repr(y)),
                None => write!(f, "[bx + si]"),
            },
            EffectiveAddress::BX_DI(x, _) => match x {
                Some(y) => write!(f, "[bx + di {}]", disp_str_repr(y)),
                None => write!(f, "[bx + di]"),
            },
            EffectiveAddress::BP_SI(x, _) => match x {
                Some(y) => write!(f, "[bp + si {}]", disp_str_repr(y)),
                None => write!(f, "[bp + si]"),
            },
            EffectiveAddress::BP_DI(x, _) => match x {
                Some(y) => write!(f, "[bp + di {}]", disp_str_repr(y)),
                None => write!(f, "[bp + di]"),
            },
            EffectiveAddress::SI(x, _) => match x {
                Some(y) => write!(f, "[si {}]", disp_str_repr(y)),
                None => write!(f, "[si]"),
            },
            EffectiveAddress::DI(x, _) => match x {
                Some(y) => write!(f, "[di {}]", disp_str_repr(y)),
                None => write!(f, "[di]"),
            },
            EffectiveAddress::BP(x, _) => write!(f, "[bp {}]", disp_str_repr(x)),
            EffectiveAddress::BX(x, _) => match x {
                Some(y) => write!(f, "[bx {}]", disp_str_repr(y)),
                None => write!(f, "[bx]"),
            },
        }
    }
}

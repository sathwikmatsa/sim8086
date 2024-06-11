use std::fmt;

use super::Operand;

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum EffectiveAddress {
    DirectAddress(u16),
    BX_SI(Option<u16>),
    BX_DI(Option<u16>),
    BP_SI(Option<u16>),
    BP_DI(Option<u16>),
    SI(Option<u16>),
    DI(Option<u16>),
    BP(u16),
    BX(Option<u16>),
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
            EffectiveAddress::DirectAddress(x) => write!(f, "[{}]", x),
            EffectiveAddress::BX_SI(x) => match x {
                Some(y) => write!(f, "[bx + si {}]", disp_str_repr(y)),
                None => write!(f, "[bx + si]"),
            },
            EffectiveAddress::BX_DI(x) => match x {
                Some(y) => write!(f, "[bx + di {}]", disp_str_repr(y)),
                None => write!(f, "[bx + di]"),
            },
            EffectiveAddress::BP_SI(x) => match x {
                Some(y) => write!(f, "[bp + si {}]", disp_str_repr(y)),
                None => write!(f, "[bp + si]"),
            },
            EffectiveAddress::BP_DI(x) => match x {
                Some(y) => write!(f, "[bp + di {}]", disp_str_repr(y)),
                None => write!(f, "[bp + di]"),
            },
            EffectiveAddress::SI(x) => match x {
                Some(y) => write!(f, "[si {}]", disp_str_repr(y)),
                None => write!(f, "[si]"),
            },
            EffectiveAddress::DI(x) => match x {
                Some(y) => write!(f, "[di {}]", disp_str_repr(y)),
                None => write!(f, "[di]"),
            },
            EffectiveAddress::BP(x) => write!(f, "[bp {}]", disp_str_repr(x)),
            EffectiveAddress::BX(x) => match x {
                Some(y) => write!(f, "[bx {}]", disp_str_repr(y)),
                None => write!(f, "[bx]"),
            },
        }
    }
}
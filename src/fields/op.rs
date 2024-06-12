use std::fmt::{self, Display};

use enum_stringify::EnumStringify;

use super::{Data, EffectiveAddress, Register};

#[derive(EnumStringify, Debug, PartialEq, Clone, Copy)]
#[enum_stringify(case = "lower")]
pub enum Operation {
    Mov,
    Add,
    Sub,
    Cmp
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operand {
    Immediate(Data),
    Register(Register),
    EffectiveAddress(EffectiveAddress),
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EffectiveAddress(x) => write!(f, "{}", x),
            Self::Register(x) => write!(f, "{}", x),
            Self::Immediate(x) => write!(f, "{}", x),
        }
    }
}

use std::fmt;

use super::{EffectiveAddress, Operand, Register};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RM {
    Reg(Register),
    Mem(EffectiveAddress),
}

impl From<RM> for Operand {
    fn from(val: RM) -> Self {
        match val {
            RM::Reg(x) => Operand::Register(x),
            RM::Mem(x) => Operand::EffectiveAddress(x),
        }
    }
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
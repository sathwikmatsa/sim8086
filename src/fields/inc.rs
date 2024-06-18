use std::fmt;

use super::Operand;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Inc {
    I8(i8),
    I16(i16),
}

impl From<Inc> for Operand {
    fn from(val: Inc) -> Self {
        Operand::Increment(val)
    }
}

impl fmt::Display for Inc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I8(x) => write!(f, "$+2+{}", x),
            Self::I16(x) => write!(f, "$+3+{}", x),
        }
    }
}

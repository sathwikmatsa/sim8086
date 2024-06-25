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

impl TryFrom<Operand> for Inc {
    type Error = ();
    fn try_from(value: Operand) -> Result<Self, Self::Error> {
        match value {
            Operand::Increment(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl From<Inc> for i16 {
    fn from(value: Inc) -> Self {
        match value {
            Inc::I8(x) => x as i16,
            Inc::I16(x) => x,
        }
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

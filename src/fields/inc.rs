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

impl From<&Inc> for i16 {
    fn from(data: &Inc) -> Self {
        match data {
            Inc::I8(value) => value.to_owned() as i16,
            Inc::I16(value) => value.to_owned(),
        }
    }
}

impl fmt::Display for Inc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = i16::from(self);
        if val >= 0 {
            write!(f, "$+2+{}", val)
        } else {
            write!(f, "$+2{}", val)
        }
    }
}

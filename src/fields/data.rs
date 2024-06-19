use std::fmt;

use super::Operand;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Data {
    U8(u8),
    U16(u16),
}

impl From<Data> for Operand {
    fn from(val: Data) -> Self {
        Operand::Immediate(val)
    }
}

impl From<&Data> for u16 {
    fn from(data: &Data) -> Self {
        match data {
            Data::U8(value) => value.to_owned() as u16,
            Data::U16(value) => value.to_owned(),
        }
    }
}

impl TryFrom<&Data> for u8 {
    type Error = ();
    fn try_from(value: &Data) -> Result<Self, Self::Error> {
        match value {
            Data::U8(data) => Ok(data.to_owned()),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u16::from(self))
    }
}

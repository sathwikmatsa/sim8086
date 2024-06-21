use std::fmt;
use std::ops;

use super::Operand;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Data {
    U8(u8),
    U16(u16),
}

impl ops::Add<Data> for Data {
    type Output = Data;

    fn add(self, rhs: Data) -> Self::Output {
        match self {
            Self::U8(x) => match rhs {
                Self::U8(y) => Data::U8(x + y),
                Self::U16(y) => Data::U16(x as u16 + y),
            },
            Self::U16(x) => match rhs {
                Self::U8(y) => Data::U16(x + y as u16),
                Self::U16(y) => Data::U16(x + y),
            },
        }
    }
}

impl ops::Sub<Data> for Data {
    type Output = Data;

    fn sub(self, rhs: Data) -> Self::Output {
        match self {
            Self::U8(x) => match rhs {
                Self::U8(y) => Data::U8(x - y),
                Self::U16(y) => Data::U16(x as u16 - y),
            },
            Self::U16(x) => match rhs {
                Self::U8(y) => Data::U16(x - y as u16),
                Self::U16(y) => Data::U16(x - y),
            },
        }
    }
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

impl Data {
    pub fn is_zero(&self) -> bool {
        u16::from(self) == 0
    }
    pub fn is_signed(&self) -> bool {
        match self {
            Self::U8(x) => (x >> 7) == 1,
            Self::U16(x) => (x >> 15) == 1,
        }
    }
    pub fn is_even_parity(&self) -> bool {
        match self {
            Self::U8(x) => x.count_ones() % 2 == 0,
            Self::U16(x) => x.count_ones() % 2 == 0,
        }
    }
}

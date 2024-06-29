use std::fmt;
use std::ops;

use super::Operand;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Data {
    U8(u8),
    U16(u16),
}

impl Data {
    fn to_u16(self) -> u16 {
        match self {
            Data::U8(x) => x as u16,
            Data::U16(x) => x,
        }
    }
}

pub struct Carry(pub bool);
pub struct HalfCarry(pub bool);
pub struct DataWithCarry(pub Data, pub Carry, pub HalfCarry);

impl ops::Add<Data> for Data {
    type Output = DataWithCarry;

    fn add(self, rhs: Data) -> Self::Output {
        const NIBBLE_MAX: u8 = 15;
        match self {
            Self::U8(x) => match rhs {
                Self::U8(y) => {
                    let (val, carry) = x.overflowing_add(y);
                    let xnibble = (x << 4) >> 4;
                    let ynibble = (y << 4) >> 4;
                    let half_carry = xnibble + ynibble > NIBBLE_MAX;
                    DataWithCarry(Data::U8(val), Carry(carry), HalfCarry(half_carry))
                }
                Self::U16(_) => unreachable!(),
            },
            Self::U16(x) => match rhs {
                Self::U8(_) => unreachable!(),
                Self::U16(y) => {
                    let (val, carry) = x.overflowing_add(y);
                    let xnibble = ((x << 12) >> 12) as u8;
                    let ynibble = ((y << 12) >> 12) as u8;
                    let half_carry = xnibble + ynibble > NIBBLE_MAX;
                    DataWithCarry(Data::U16(val), Carry(carry), HalfCarry(half_carry))
                }
            },
        }
    }
}

impl ops::Sub<Data> for Data {
    type Output = DataWithCarry;

    fn sub(self, rhs: Data) -> Self::Output {
        match self {
            Self::U8(x) => match rhs {
                Self::U8(y) => {
                    let (val, carry) = x.overflowing_sub(y);
                    let xnibble = (x << 4) >> 4;
                    let ynibble = (y << 4) >> 4;
                    let half_carry = xnibble < ynibble;
                    DataWithCarry(Data::U8(val), Carry(carry), HalfCarry(half_carry))
                }
                Self::U16(_) => unreachable!(),
            },
            Self::U16(x) => match rhs {
                Self::U8(_) => unreachable!(),
                Self::U16(y) => {
                    let (val, carry) = x.overflowing_sub(y);
                    let xnibble = ((x << 12) >> 12) as u8;
                    let ynibble = ((y << 12) >> 12) as u8;
                    let half_carry = xnibble < ynibble;
                    DataWithCarry(Data::U16(val), Carry(carry), HalfCarry(half_carry))
                }
            },
        }
    }
}

impl ops::BitAnd<Data> for Data {
    type Output = Data;
    fn bitand(self, rhs: Data) -> Self::Output {
        match self {
            Self::U8(x) => match rhs {
                Self::U8(y) => Data::U8(x & y),
                Self::U16(_) => unreachable!(),
            },
            Self::U16(x) => match rhs {
                Self::U8(_) => unreachable!(),
                Self::U16(y) => Data::U16(x & y),
            },
        }
    }
}

impl ops::BitXor<Data> for Data {
    type Output = Data;
    fn bitxor(self, rhs: Data) -> Self::Output {
        match self {
            Self::U8(x) => match rhs {
                Self::U8(y) => Data::U8(x ^ y),
                Self::U16(_) => unreachable!(),
            },
            Self::U16(x) => match rhs {
                Self::U8(_) => unreachable!(),
                Self::U16(y) => Data::U16(x ^ y),
            },
        }
    }
}

impl ops::Shr<Data> for Data {
    type Output = Data;
    fn shr(self, rhs: Data) -> Self::Output {
        match self {
            Self::U8(x) => match rhs {
                Self::U8(y) => Data::U8(x >> y),
                Self::U16(_) => unreachable!(),
            },
            Self::U16(x) => match rhs {
                Self::U8(y) => Data::U16(x >> y),
                Self::U16(_) => unreachable!(),
            },
        }
    }
}

impl From<Data> for Operand {
    fn from(val: Data) -> Self {
        Operand::Immediate(val)
    }
}

impl From<Data> for u16 {
    fn from(data: Data) -> Self {
        data.to_u16()
    }
}

impl From<&Data> for u16 {
    fn from(data: &Data) -> Self {
        data.to_u16()
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
    pub fn is_lower_byte_even_parity(&self) -> bool {
        match self {
            Self::U8(x) => x.count_ones() % 2 == 0,
            Self::U16(x) => x.to_le_bytes()[0].count_ones() % 2 == 0,
        }
    }
}

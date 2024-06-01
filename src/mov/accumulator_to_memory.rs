use crate::instruction::{Data, EffectiveAddress, Instruction, Register, WithData, WithWideField};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct AccumulatorToMemory {
    pub acc: Register,
    pub direct_address: EffectiveAddress,
}

impl fmt::Display for AccumulatorToMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "mov [{}], {}",
            match self.direct_address {
                EffectiveAddress::DirectAddress(x) => x,
                _ => unreachable!(),
            },
            self.acc
        )
    }
}

impl WithWideField for AccumulatorToMemory {
    const WIDE_MASK_MATCH: u8 = 0b00000001;
}

impl WithData for AccumulatorToMemory {}

impl Instruction for AccumulatorToMemory {
    fn new<'a, I>(first_byte: u8, byte_stream: &mut I) -> Self
    where
        Self: Sized,
        I: Iterator<Item = &'a u8>,
    {
        let wide = Self::is_wide(first_byte);
        let data = Self::extract_data(first_byte, byte_stream);
        Self {
            acc: if wide { Register::AX } else { Register::AL },
            direct_address: EffectiveAddress::DirectAddress(match data {
                Data::U16(x) => x,
                Data::U8(x) => x.into(),
            }),
        }
    }
    
    fn opcode_mask() -> u8 where Self: Sized {
        const OPCODE_MASK: u8 = 0b11111110;
        return OPCODE_MASK;
    }
    
    fn opcode_match() -> u8 where Self: Sized {
        const OPCODE_MATCH: u8 = 0b10100010;
        return OPCODE_MATCH;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_match() {
        assert!(AccumulatorToMemory::match_opcode(0b10100011));
        assert!(!AccumulatorToMemory::match_opcode(0b10010101));
    }

    #[test]
    fn not_wide() {
        let bytes: [u8; 2] = [0b10100010, 0b00000001];
        assert_eq!(
            AccumulatorToMemory::new(bytes[0], &mut bytes[1..].into_iter()),
            AccumulatorToMemory {
                acc: Register::AL,
                direct_address: EffectiveAddress::DirectAddress(1)
            }
        );
    }

    #[test]
    fn wide() {
        let bytes: [u8; 3] = [0b10100011, 0b00000000, 0b00000001];
        assert_eq!(
            AccumulatorToMemory::new(bytes[0], &mut bytes[1..].into_iter()),
            AccumulatorToMemory {
                acc: Register::AX,
                direct_address: EffectiveAddress::DirectAddress(256)
            }
        );
    }

    #[test]
    fn print() {
        let ins = AccumulatorToMemory {
            acc: Register::AX,
            direct_address: EffectiveAddress::DirectAddress(256),
        };
        assert_eq!(format!("{}", ins), "mov [256], ax");
    }
}

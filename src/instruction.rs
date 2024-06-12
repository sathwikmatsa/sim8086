use std::fmt::{self, Display};
use std::iter::Peekable;
use std::slice::Iter;

use crate::fields::{Data, Operand, Operation};

#[derive(Debug, PartialEq)]
pub struct Inst {
    pub operation: Operation,
    pub first: Option<Operand>,
    pub second: Option<Operand>,
}

impl Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // special cases
        if let Some(Operand::EffectiveAddress(x)) = self.first {
            if let Some(Operand::Immediate(y)) = self.second {
                return write!(
                    f,
                    "{} {}, {}",
                    self.operation,
                    x,
                    match y {
                        Data::U8(b) => format!("byte {}", b),
                        Data::U16(w) => format!("word {}", w),
                    }
                );
            }
        }

        write!(f, "{}", self.operation)?;
        if self.first.is_some() {
            write!(f, " {}", self.first.unwrap())?;
            if self.second.is_some() {
                write!(f, ", {}", self.second.unwrap())?;
            }
        }
        Ok(())
    }
}

pub trait InstructionDecoder {
    fn decode(&self, first_byte: u8, byte_stream: &mut Peekable<Iter<u8>>, op: Operation) -> Inst;
}

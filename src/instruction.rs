use std::fmt::{self, Display};
use std::iter::Peekable;
use std::slice::Iter;

use enum_stringify::EnumStringify;

use crate::fields::{Operand, Operation};

#[derive(Debug, Default, PartialEq, EnumStringify, Copy, Clone)]
pub enum InstructionPrefix {
    #[default]
    Lock,
    Rep,
    SegmentOverride,
    LockSegmentOverride,
}

#[derive(Debug, PartialEq)]
pub struct Inst {
    pub operation: Operation,
    pub first: Option<Operand>,
    pub second: Option<Operand>,
    pub prefix: Option<InstructionPrefix>,
}

impl Inst {
    pub fn new(op: Operation) -> Self {
        Inst {
            operation: op,
            first: None,
            second: None,
            prefix: None,
        }
    }

    pub fn with_operand(op: Operation, first: Operand) -> Self {
        Inst {
            operation: op,
            first: Some(first),
            second: None,
            prefix: None,
        }
    }

    pub fn with_operands(op: Operation, first: Operand, second: Operand) -> Self {
        Inst {
            operation: op,
            first: Some(first),
            second: Some(second),
            prefix: None,
        }
    }

    pub fn add_instruction_prefix(&mut self, prefix: InstructionPrefix) {
        self.prefix = Some(prefix);
    }
}

impl Display for Inst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Handle instruction prefix
        if let Some(prefix) = self.prefix {
            write!(f, "{} ", prefix.to_string().to_ascii_lowercase())?;
        }

        // special cases / workarounds / hacks
        // Push & Pop implicitly assume 16-bit operations in 8086. But NASM is complaining, so hardcoding size
        if [Operation::Push, Operation::Pop].contains(&self.operation) && self.second.is_none() {
            if let Some(Operand::EffectiveAddress(x)) = self.first {
                return write!(f, "{} word {}", self.operation, x);
            }
        }
        // XCHG operand order shouldn't matter during runtime. This is just to make the binary testing
        // work
        if self.operation == Operation::XCHG && self.second.is_some() {
            return write!(
                f,
                "{} {}, {}",
                self.operation,
                self.second.unwrap(),
                self.first.unwrap()
            );
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

impl InstructionDecoder for InstructionPrefix {
    fn decode(
        &self,
        _first_byte: u8,
        _byte_stream: &mut Peekable<Iter<u8>>,
        _op: Operation,
    ) -> Inst {
        // this is a dirty hack to let the macro in decoder.rs compile
        unreachable!()
    }
}

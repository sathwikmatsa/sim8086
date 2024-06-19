use std::fmt::{self, Display};
use std::iter::Peekable;
use std::mem::swap;
use std::slice::Iter;
use std::str::FromStr;

use crate::fields::{Operand, Operation, SegmentRegister};

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum InstructionPrefix {
    #[default]
    Lock,
    Rep,
    SegmentOverride(SegmentRegister),
    LockSegmentOverride(SegmentRegister),
}

impl FromStr for InstructionPrefix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lock" => Ok(Self::Lock),
            "Rep" => Ok(Self::Rep),
            "SegmentOverrideCS" => Ok(Self::SegmentOverride(SegmentRegister::CS)),
            "SegmentOverrideES" => Ok(Self::SegmentOverride(SegmentRegister::ES)),
            "SegmentOverrideDS" => Ok(Self::SegmentOverride(SegmentRegister::DS)),
            "SegmentOverrideSS" => Ok(Self::SegmentOverride(SegmentRegister::SS)),
            "LockSegmentOverrideCS" => Ok(Self::LockSegmentOverride(SegmentRegister::CS)),
            "LockSegmentOverrideES" => Ok(Self::LockSegmentOverride(SegmentRegister::ES)),
            "LockSegmentOverrideDS" => Ok(Self::LockSegmentOverride(SegmentRegister::DS)),
            "LockSegmentOverrideSS" => Ok(Self::LockSegmentOverride(SegmentRegister::SS)),
            _ => Err(()),
        }
    }
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

    pub fn with_operand_v2(op: Operation, first: impl Into<Operand>) -> Self {
        Inst {
            operation: op,
            first: Some(first.into()),
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

    pub fn with_operands_v2(
        op: Operation,
        first: impl Into<Operand>,
        second: impl Into<Operand>,
    ) -> Self {
        Inst {
            operation: op,
            first: Some(first.into()),
            second: Some(second.into()),
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
            match prefix {
                InstructionPrefix::Rep => write!(f, "rep ")?,
                InstructionPrefix::Lock => write!(f, "lock ")?,
                InstructionPrefix::LockSegmentOverride(_) => write!(f, "lock ")?,
                _ => (),
            }
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
        let mut first = self.first;
        let mut second = self.second;
        if self.operation == Operation::XCHG && self.second.is_some() {
            swap(&mut first, &mut second);
        }

        let handle_ea = |x: Operand| -> String {
            if let Operand::EffectiveAddress(ea) = x {
                let wide = ea.wide();
                match self.prefix {
                    Some(InstructionPrefix::SegmentOverride(x))
                    | Some(InstructionPrefix::LockSegmentOverride(x)) => {
                        format!("{}{}:{}", wide, x, ea)
                    }
                    _ => format!("{}{}", wide, ea),
                }
            } else {
                x.to_string()
            }
        };

        write!(f, "{}", self.operation)?;
        if first.is_some() {
            write!(f, " {}", handle_ea(first.unwrap()))?;
            if second.is_some() {
                write!(f, ", {}", handle_ea(second.unwrap()))?;
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

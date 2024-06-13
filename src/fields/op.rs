use std::fmt::{self, Display};

use enum_stringify::EnumStringify;

use super::{Data, EffectiveAddress, Inc, Register, SegmentRegister};

#[derive(EnumStringify, Debug, PartialEq, Clone, Copy)]
#[enum_stringify(case = "lower")]
#[allow(clippy::upper_case_acronyms)]
pub enum Operation {
    Mov,
    Add,
    Sub,
    Cmp,
    JE,
    JL,
    JLE,
    JB,
    JBE,
    JP,
    JO,
    JS,
    JNE,
    JNL,
    JNLE,
    JNB,
    JNBE,
    JNP,
    JNO,
    JNS,
    LOOP,
    LOOPZ,
    LOOPNZ,
    JCXZ,
    Push,
    Pop,
    XCHG,
    IN,
    OUT,
    XLAT,
    LEA,
    LDS,
    LES,
    LAHF,
    SAHF,
    PUSHF,
    POPF,
    ADC,
    INC,
    AAA,
    DAA,
    SBB,
    DEC,
    NEG,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operand {
    Increment(Inc),
    Immediate(Data),
    Register(Register),
    EffectiveAddress(EffectiveAddress),
    SR(SegmentRegister),
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EffectiveAddress(x) => write!(f, "{}", x),
            Self::Register(x) => write!(f, "{}", x),
            Self::Immediate(x) => write!(f, "{}", x),
            Self::Increment(x) => write!(f, "{}", x),
            Self::SR(x) => write!(f, "{}", x),
        }
    }
}

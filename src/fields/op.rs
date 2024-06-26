use std::fmt::{self, Display};

use enum_stringify::EnumStringify;

use super::{CsIp, Data, EffectiveAddress, Inc, Register, SegmentRegister};

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
    AAS,
    DAS,
    MUL,
    IMUL,
    AAM,
    DIV,
    IDIV,
    AAD,
    CBW,
    CWD,
    NOT,
    SHL,
    SHR,
    SAR,
    ROL,
    ROR,
    RCL,
    RCR,
    AND,
    TEST,
    OR,
    XOR,
    MOVSB,
    MOVSW,
    LODSB,
    LODSW,
    STOSB,
    STOSW,
    CMPSB,
    CMPSW,
    SCASB,
    SCASW,
    Call,
    #[enum_stringify(rename = "call far")]
    CallFar,
    Jmp,
    #[enum_stringify(rename = "jmp far")]
    JmpFar,
    Ret,
    #[enum_stringify(rename = "retf")]
    RetFar,
    INT,
    INT3,
    INTO,
    IRET,
    CLC,
    CMC,
    STC,
    CLD,
    STD,
    CLI,
    STI,
    HLT,
    WAIT,

    // instruction prefixes
    Lock,
    Rep,
    SegmentOverrideES,
    SegmentOverrideCS,
    SegmentOverrideSS,
    SegmentOverrideDS,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operand {
    Increment(Inc),
    Immediate(Data),
    Register(Register),
    EffectiveAddress(EffectiveAddress),
    SR(SegmentRegister),
    CsIp(CsIp),
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EffectiveAddress(x) => write!(f, "{}", x),
            Self::Register(x) => write!(f, "{}", x),
            Self::Immediate(x) => write!(f, "{}", x),
            Self::Increment(x) => write!(f, "{}", x),
            Self::SR(x) => write!(f, "{}", x),
            Self::CsIp(x) => write!(f, "{}", x),
        }
    }
}

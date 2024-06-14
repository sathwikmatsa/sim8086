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
    #[enum_stringify(rename = "rep movsb")]
    RepMOVSB,
    #[enum_stringify(rename = "rep movsw")]
    RepMOVSW,
    #[enum_stringify(rename = "rep lodsb")]
    RepLODSB,
    #[enum_stringify(rename = "rep lodsw")]
    RepLODSW,
    #[enum_stringify(rename = "rep stosb")]
    RepSTOSB,
    #[enum_stringify(rename = "rep stosw")]
    RepSTOSW,
    #[enum_stringify(rename = "repe cmpsb")]
    RepeCMPSB,
    #[enum_stringify(rename = "repe cmpsw")]
    RepeCMPSW,
    #[enum_stringify(rename = "repe scasb")]
    RepeSCASB,
    #[enum_stringify(rename = "repe scasw")]
    RepeSCASW,
    #[enum_stringify(rename = "repne cmpsb")]
    RepneCMPSB,
    #[enum_stringify(rename = "repne cmpsw")]
    RepneCMPSW,
    #[enum_stringify(rename = "repne scasb")]
    RepneSCASB,
    #[enum_stringify(rename = "repne scasw")]
    RepneSCASW,
    Call,
    Jmp,
    Ret,
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

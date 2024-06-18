use std::fmt;

use super::Operand;

#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct CsIp {
    pub code_segment: u16,
    pub instruction_pointer: u16,
}

impl From<CsIp> for Operand {
    fn from(val: CsIp) -> Self {
        Operand::CsIp(val)
    }
}

impl fmt::Display for CsIp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.code_segment, self.instruction_pointer)
    }
}

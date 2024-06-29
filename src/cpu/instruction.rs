use crate::{disasm::Instruction, fields::Operation};

pub struct JmpTakenClocks(pub usize);
pub struct JmpNotTakenClocks(pub usize);

impl Instruction {
    pub fn is_conditional_advance(&self) -> bool {
        matches!(
            self.operation,
            Operation::JE
                | Operation::JL
                | Operation::JLE
                | Operation::JB
                | Operation::JBE
                | Operation::JP
                | Operation::JO
                | Operation::JS
                | Operation::JNE
                | Operation::JNL
                | Operation::JNLE
                | Operation::JNB
                | Operation::JNBE
                | Operation::JNP
                | Operation::JNO
                | Operation::JNS
        )
    }

    pub fn clocks_for_coditional_advance(&self) -> (JmpTakenClocks, JmpNotTakenClocks) {
        assert!(self.is_conditional_advance());
        match self.operation {
            Operation::JE => (JmpTakenClocks(16), JmpNotTakenClocks(4)),
            Operation::JNE => (JmpTakenClocks(16), JmpNotTakenClocks(4)),
            Operation::JB => (JmpTakenClocks(16), JmpNotTakenClocks(4)),
            _ => unimplemented!("{:?}", self),
        }
    }
}

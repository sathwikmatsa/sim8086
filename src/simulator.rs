use std::fmt::Display;

use crate::{
    cpu::{Flags, Registers},
    fields::Operation,
    handlers::*,
    instruction::Inst,
};

#[derive(Default)]
pub struct Simulator {
    pub registers: Registers,
    pub flags: Flags,
}

impl Simulator {
    pub fn exec(&mut self, inst: &Inst) {
        match inst.operation {
            Operation::Mov => handle_mov(inst, &mut self.registers),
            Operation::Add => handle_arithmetic(
                ArithmeticOp::Add,
                inst,
                &mut self.registers,
                &mut self.flags,
            ),
            Operation::Sub => handle_arithmetic(
                ArithmeticOp::Sub,
                inst,
                &mut self.registers,
                &mut self.flags,
            ),
            Operation::Cmp => handle_arithmetic(
                ArithmeticOp::Cmp,
                inst,
                &mut self.registers,
                &mut self.flags,
            ),
            _ => unimplemented!(),
        }
    }
}

impl Display for Simulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.registers)?;
        write!(f, "{}", self.flags)
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{Data, Register};

    use super::*;

    #[test]
    fn simulator_move_immediate_to_register() {
        let mov = Inst::with_operands_v2(Operation::Mov, Register::BX, Data::U16(256));
        let mut simulator = Simulator::default();
        simulator.exec(&mov);
        assert_eq!(simulator.registers.get(Register::BX), Data::U16(256));
    }
}

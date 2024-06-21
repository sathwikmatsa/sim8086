use std::fmt::Display;

use crate::{
    cpu::Registers,
    fields::{Operand, Operation},
    instruction::Inst,
};

#[derive(Default)]
pub struct Simulator {
    registers: Registers,
}

impl Simulator {
    pub fn exec(&mut self, inst: &Inst) {
        match inst.operation {
            Operation::Mov => {
                let first = inst.first.expect("mov has first operand");
                let second = inst.second.expect("mov has second operand");

                match (first, second) {
                    (Operand::Register(reg), Operand::Immediate(data)) => {
                        self.registers.set_imd(reg, data)
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        self.registers.set_reg(reg1, reg2)
                    }
                    (Operand::Register(reg), Operand::SR(sr)) => {
                        self.registers.set_reg_from_sr(reg, sr)
                    }
                    (Operand::SR(sr), Operand::Register(reg)) => {
                        self.registers.set_sr_from_reg(sr, reg)
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl Display for Simulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.registers)
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

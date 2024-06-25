use std::fmt::Display;

use crate::{
    cpu::{Flags, Registers},
    disasm::Program,
    fields::{Inc, Operation},
    handlers::*,
};

#[derive(Default)]
pub struct Simulator {
    pub registers: Registers,
    pub flags: Flags,
    pub ip: u16,
    log_ip: bool,
}

impl Simulator {
    pub fn enable_ip_log(&mut self) {
        self.log_ip = true;
    }

    pub fn exec(&mut self, program: &mut Program) {
        while let Some(inst) = program.next_instruction() {
            self.ip += inst.size as u16;

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
                Operation::JNE => {
                    if !self.flags.zero {
                        let first = inst.first.expect("JNE has first operand");
                        let inc: Inc = first.try_into().expect("JNE has Inc operand");
                        let nbytes: i16 = inc.into();
                        program.advance_by(nbytes);
                        self.ip = self.ip.checked_add_signed(nbytes).unwrap();
                    }
                }
                _ => unimplemented!(),
            }
        }
    }
}

impl Display for Simulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.registers)?;
        if self.log_ip {
            writeln!(f, "      ip: {:#06x} ({})", self.ip, self.ip)?;
        }
        write!(f, "{}", self.flags)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        fields::{Data, Register},
        instruction::Inst,
    };

    use super::*;

    #[test]
    fn simulator_move_immediate_to_register() {
        let mut mov = Inst::with_operands_v2(Operation::Mov, Register::BX, Data::U16(256));
        mov.set_size(3);
        let mut simulator = Simulator::default();
        let mut program = vec![mov].try_into().unwrap();
        simulator.exec(&mut program);
        assert_eq!(simulator.registers.get(Register::BX), Data::U16(256));
    }
}

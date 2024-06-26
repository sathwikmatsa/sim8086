use std::fmt::Display;

use crate::{
    conditional_advance,
    cpu::{Flags, Memory, Registers},
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
    pub memory: Memory,
}

impl Simulator {
    pub fn enable_ip_log(&mut self) {
        self.log_ip = true;
    }

    pub fn exec(&mut self, program: &mut Program) {
        while let Some(inst) = program.next_instruction() {
            self.ip += inst.size as u16;

            match inst.operation {
                Operation::Mov => handle_mov(inst, &mut self.registers, &mut self.memory),
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
                    conditional_advance!(!self.flags.zero, "JNE", self, inst, program)
                }
                Operation::JE => {
                    conditional_advance!(self.flags.zero, "JE", self, inst, program)
                }
                Operation::JB => {
                    conditional_advance!(self.flags.carry, "JB", self, inst, program)
                }
                Operation::JP => {
                    conditional_advance!(self.flags.parity, "JP", self, inst, program)
                }
                Operation::LOOPNZ => {
                    self.registers.dec_cx();
                    let cond = self.registers.cx() != 0 && !self.flags.zero;
                    conditional_advance!(cond, "LOOPNZ", self, inst, program)
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

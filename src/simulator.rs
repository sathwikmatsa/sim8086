use std::fmt::Display;

use crate::{
    conditional_advance,
    cpu::{Clocks8086, Clocks8088, Flags, JmpNotTakenClocks, JmpTakenClocks, Memory, Registers},
    disasm::Program,
    fields::{EffectiveAddress, Inc, Operation},
    handlers::*,
};

#[derive(Default)]
pub struct Simulator {
    pub registers: Registers,
    pub flags: Flags,
    pub ip: u16,
    log_ip: bool,
    estimate_cycles: bool,
    cycles_8086: Clocks8086,
    cycles_8088: Clocks8088,
    pub memory: Memory,
}

impl Simulator {
    pub fn enable_ip_log(&mut self) {
        self.log_ip = true;
    }

    pub fn enable_cycle_estimation(&mut self) {
        self.estimate_cycles = true;
    }

    pub fn clocks_8086(&self) -> usize {
        self.cycles_8086.0
    }

    pub fn clocks_8088(&self) -> usize {
        self.cycles_8088.0
    }

    pub fn exec(&mut self, program: &mut Program) {
        while let Some(inst) = program.next_instruction() {
            // STOP on RET
            if inst.operation == Operation::Ret {
                break;
            }
            self.ip += inst.size as u16;

            if self.estimate_cycles && !inst.is_conditional_advance() {
                let (clocks86, clocks88) = inst.clocks(|ea: EffectiveAddress| -> bool {
                    self.registers.calculate_eff_addr(ea) % 2 != 0
                });
                self.cycles_8086 += clocks86;
                self.cycles_8088 += clocks88;
            }

            match inst.operation {
                Operation::Mov => handle_mov(inst, &mut self.registers, &mut self.memory),
                Operation::Add => handle_arithmetic(
                    ArithmeticOp::Add,
                    inst,
                    &mut self.registers,
                    &mut self.flags,
                    &mut self.memory,
                ),
                Operation::Sub => handle_arithmetic(
                    ArithmeticOp::Sub,
                    inst,
                    &mut self.registers,
                    &mut self.flags,
                    &mut self.memory,
                ),
                Operation::Cmp => handle_arithmetic(
                    ArithmeticOp::Cmp,
                    inst,
                    &mut self.registers,
                    &mut self.flags,
                    &mut self.memory,
                ),
                Operation::JNE => {
                    conditional_advance!(!self.flags.zero, "JNE", self, inst, program);
                }
                Operation::JE => {
                    conditional_advance!(self.flags.zero, "JE", self, inst, program);
                }
                Operation::JB => {
                    conditional_advance!(self.flags.carry, "JB", self, inst, program);
                }
                Operation::JP => {
                    conditional_advance!(self.flags.parity, "JP", self, inst, program);
                }
                Operation::LOOPNZ => {
                    self.registers.dec_cx();
                    let cond = self.registers.cx() != 0 && !self.flags.zero;
                    conditional_advance!(cond, "LOOPNZ", self, inst, program);
                }
                Operation::LOOP => {
                    self.registers.dec_cx();
                    let cond = self.registers.cx() != 0;
                    conditional_advance!(cond, "LOOP", self, inst, program);
                }
                Operation::TEST => handle_logical(
                    LogicalOp::Test,
                    inst,
                    &mut self.registers,
                    &mut self.flags,
                    &mut self.memory,
                ),
                Operation::XOR => handle_logical(
                    LogicalOp::Xor,
                    inst,
                    &mut self.registers,
                    &mut self.flags,
                    &mut self.memory,
                ),
                Operation::INC => handle_arithmetic(
                    ArithmeticOp::Inc,
                    inst,
                    &mut self.registers,
                    &mut self.flags,
                    &mut self.memory,
                ),
                Operation::DEC => handle_arithmetic(
                    ArithmeticOp::Dec,
                    inst,
                    &mut self.registers,
                    &mut self.flags,
                    &mut self.memory,
                ),
                Operation::SHR => handle_logical(
                    LogicalOp::Shr,
                    inst,
                    &mut self.registers,
                    &mut self.flags,
                    &mut self.memory,
                ),
                _ => unimplemented!("{:?}", inst),
            }
        }
    }

    pub fn dump_memory(&self, mut f: impl std::io::Write) -> Result<(), std::io::Error> {
        f.write_all(self.memory.raw())
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

use crate::{cpu::Registers, fields::Operand, instruction::Inst};

pub fn handle_mov(inst: &Inst, registers: &mut Registers) {
    let first = inst.first.expect("mov has first operand");
    let second = inst.second.expect("mov has second operand");

    match (first, second) {
        (Operand::Register(reg), Operand::Immediate(data)) => registers.set_imd(reg, data),
        (Operand::Register(reg1), Operand::Register(reg2)) => registers.set_reg(reg1, reg2),
        (Operand::Register(reg), Operand::SR(sr)) => registers.set_reg_from_sr(reg, sr),
        (Operand::SR(sr), Operand::Register(reg)) => registers.set_sr_from_reg(sr, reg),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::{Data, Operation, Register};

    use super::*;

    #[test]
    fn move_immediate_to_register() {
        let inst = Inst::with_operands_v2(Operation::Mov, Register::BX, Data::U16(256));
        let mut registers = Registers::default();
        handle_mov(&inst, &mut registers);
        assert_eq!(registers.get(Register::BX), Data::U16(256));
    }
}

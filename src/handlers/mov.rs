use crate::{cpu::Registers, disasm::Instruction, fields::Operand};

pub fn handle_mov(inst: &Instruction, registers: &mut Registers) {
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
        let inst = Instruction {
            operation: Operation::Mov,
            first: Some(Register::BX.into()),
            second: Some(Data::U16(256).into()),
            prefix: None,
            size: 4,
        };
        let mut registers = Registers::default();
        handle_mov(&inst, &mut registers);
        assert_eq!(registers.get(Register::BX), Data::U16(256));
    }
}

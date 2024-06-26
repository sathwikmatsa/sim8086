use crate::{
    cpu::{Memory, Registers},
    disasm::Instruction,
    fields::{Data, Operand, Wide},
};

pub fn handle_mov(inst: &Instruction, registers: &mut Registers, memory: &mut Memory) {
    let first = inst.first.expect("mov has first operand");
    let second = inst.second.expect("mov has second operand");

    match (first, second) {
        (Operand::Register(reg), Operand::Immediate(data)) => registers.set_imd(reg, data),
        (Operand::Register(reg1), Operand::Register(reg2)) => registers.set_reg(reg1, reg2),
        (Operand::Register(reg), Operand::SR(sr)) => registers.set_reg_from_sr(reg, sr),
        (Operand::SR(sr), Operand::Register(reg)) => registers.set_sr_from_reg(sr, reg),
        (Operand::EffectiveAddress(addr), Operand::Immediate(Data::U16(imd))) => {
            memory.store_16(registers.calculate_eff_addr(addr), imd);
        }
        (Operand::EffectiveAddress(addr), Operand::Immediate(Data::U8(imd))) => {
            memory.store_8(registers.calculate_eff_addr(addr), imd);
        }
        (Operand::Register(reg), Operand::EffectiveAddress(ea)) => {
            if ea.wide() == Wide::Byte {
                unimplemented!()
            }
            let addr = registers.calculate_eff_addr(ea);
            let imd = memory.load_16(addr);
            registers.set_imd(reg, Data::U16(imd));
        }
        (Operand::EffectiveAddress(ea), Operand::Register(reg)) => {
            let data = registers.get(reg);
            let addr = registers.calculate_eff_addr(ea);
            match ea.wide() {
                Wide::Byte => memory.store_8(addr, (&data).try_into().expect("8bit data")),
                Wide::Word => memory.store_16(addr, data.into()),
                _ => unreachable!(),
            }
        }
        _ => unimplemented!("{:?}", inst),
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
        let mut memory = Memory::default();
        handle_mov(&inst, &mut registers, &mut memory);
        assert_eq!(registers.get(Register::BX), Data::U16(256));
    }
}

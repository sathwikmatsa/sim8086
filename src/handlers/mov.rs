use crate::{
    cpu::{Memory, Registers},
    disasm::Instruction,
    fields::{Data, EffectiveAddress, Operand, Register, Wide},
};

pub fn handle_mov(inst: &Instruction, registers: &mut Registers, memory: &mut Memory) {
    let first = inst.first.expect("mov has first operand");
    let second = inst.second.expect("mov has second operand");

    match (first, second) {
        (Operand::Register(reg), Operand::Immediate(data)) => registers.set_imd(reg, data),
        (Operand::Register(reg1), Operand::Register(reg2)) => registers.set_reg(reg1, reg2),
        (Operand::Register(reg), Operand::SR(sr)) => registers.set_reg_from_sr(reg, sr),
        (Operand::SR(sr), Operand::Register(reg)) => registers.set_sr_from_reg(sr, reg),
        (Operand::EffectiveAddress(addr), Operand::Immediate(imd)) => match addr {
            EffectiveAddress::DirectAddress(da, Wide::Word) => memory.store_16(da, imd.into()),
            EffectiveAddress::BX(Some(disp), Wide::Word) => {
                let bx: u16 = registers.get(Register::BX).into();
                let addr = bx + disp;
                memory.store_16(addr, imd.into());
            }
            _ => unimplemented!(),
        },
        (Operand::Register(reg), Operand::EffectiveAddress(addr)) => match addr {
            EffectiveAddress::DirectAddress(da, Wide::Word) => {
                let mem = memory.load_16(da);
                registers.set_imd(reg, Data::U16(mem));
            }
            _ => unimplemented!(),
        },
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
        let mut memory = Memory::default();
        handle_mov(&inst, &mut registers, &mut memory);
        assert_eq!(registers.get(Register::BX), Data::U16(256));
    }
}

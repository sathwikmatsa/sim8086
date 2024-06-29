use enum_stringify::EnumStringify;

use crate::{
    cpu::{Flags, Memory, Registers},
    disasm::Instruction,
    fields::{Data, DataWithCarry, Operand, Operation, Wide},
};

#[derive(EnumStringify, PartialEq)]
#[enum_stringify(case = "lower")]
pub enum ArithmeticOp {
    Add,
    Sub,
    Cmp,
    Inc,
    Dec,
}

impl ArithmeticOp {
    fn compute(&self, lhs: Data, rhs: Data) -> DataWithCarry {
        match self {
            Self::Add => lhs + rhs,
            Self::Inc => {
                lhs + match lhs {
                    Data::U16(_) => Data::U16(1),
                    Data::U8(_) => Data::U8(1),
                }
            }
            Self::Dec => {
                lhs - match lhs {
                    Data::U16(_) => Data::U16(1),
                    Data::U8(_) => Data::U8(1),
                }
            }
            Self::Sub => lhs - rhs,
            Self::Cmp => lhs - rhs,
        }
    }
}

pub fn handle_arithmetic(
    op: ArithmeticOp,
    inst: &Instruction,
    registers: &mut Registers,
    flags: &mut Flags,
    memory: &mut Memory,
) {
    let first = inst
        .first
        .unwrap_or_else(|| panic!("{} has first operand", op));
    let second = inst
        .second
        .or({
            if matches!(inst.operation, Operation::INC | Operation::DEC) {
                // This is a dummy value. RHS is overriden based on first operand width in compute fn
                Some(Operand::Immediate(Data::U16(1)))
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("{} has second operand", op));

    match (first, second) {
        (Operand::Register(reg), Operand::Immediate(rhs)) => {
            let lhs = registers.get(reg);
            let newval = op.compute(lhs, rhs);
            if op != ArithmeticOp::Cmp {
                registers.set_imd(reg, newval.0);
            }
            flags.set(lhs, rhs, op, newval);
        }
        (Operand::Register(reg1), Operand::Register(reg2)) => {
            let lhs = registers.get(reg1);
            let rhs = registers.get(reg2);
            let newval = op.compute(lhs, rhs);
            if op != ArithmeticOp::Cmp {
                registers.set_imd(reg1, newval.0);
            }
            flags.set(lhs, rhs, op, newval);
        }
        (Operand::Register(reg), Operand::EffectiveAddress(ea)) => {
            let addr = registers.calculate_eff_addr(ea);
            let rhs = match ea.wide() {
                Wide::Word => Data::U16(memory.load_16(addr)),
                Wide::Byte => Data::U8(memory.load_8(addr)),
                _ => unreachable!(),
            };
            let lhs = registers.get(reg);
            let newval = op.compute(lhs, rhs);
            if op != ArithmeticOp::Cmp {
                registers.set_imd(reg, newval.0);
            }
            flags.set(lhs, rhs, op, newval);
        }
        (Operand::EffectiveAddress(ea), Operand::Register(reg)) => {
            if ea.wide() == Wide::Byte {
                unimplemented!()
            }
            let addr = registers.calculate_eff_addr(ea);
            let lhs = Data::U16(memory.load_16(addr));
            let rhs = registers.get(reg);
            let newval = op.compute(lhs, rhs);
            if op != ArithmeticOp::Cmp {
                memory.store_16(addr, newval.0.into());
            }
            flags.set(lhs, rhs, op, newval);
        }
        (Operand::EffectiveAddress(ea), Operand::Immediate(imd)) => {
            if ea.wide() == Wide::Byte {
                unimplemented!()
            }
            let addr = registers.calculate_eff_addr(ea);
            let lhs = Data::U16(memory.load_16(addr));
            let rhs = imd;
            let newval = op.compute(lhs, rhs);
            if op != ArithmeticOp::Cmp {
                memory.store_16(addr, newval.0.into());
            }
            flags.set(lhs, rhs, op, newval);
        }
        _ => unimplemented!("{:?}", inst),
    }
}

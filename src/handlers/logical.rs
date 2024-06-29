use enum_stringify::EnumStringify;

use crate::{
    cpu::{Flags, Memory, Registers},
    disasm::Instruction,
    fields::{Data, Operand},
};

#[derive(EnumStringify, PartialEq)]
#[enum_stringify(case = "lower")]
pub enum LogicalOp {
    Test,
    Xor,
    Shr,
}

impl LogicalOp {
    fn compute(&self, lhs: Data, rhs: Data) -> Data {
        match self {
            Self::Test => lhs & rhs,
            Self::Xor => lhs ^ rhs,
            Self::Shr => lhs >> rhs,
        }
    }
}

pub fn handle_logical(
    op: LogicalOp,
    inst: &Instruction,
    registers: &mut Registers,
    flags: &mut Flags,
    _memory: &mut Memory,
) {
    let first = inst
        .first
        .unwrap_or_else(|| panic!("{} has first operand", op));
    let second = inst
        .second
        .unwrap_or_else(|| panic!("{} has second operand", op));

    match (first, second) {
        (Operand::Register(reg1), Operand::Register(reg2)) => {
            let lhs = registers.get(reg1);
            let rhs = registers.get(reg2);
            let newval = op.compute(lhs, rhs);
            if op != LogicalOp::Test {
                registers.set_imd(reg1, newval);
            }
            flags.set_logical(newval);
        }
        (Operand::Register(reg), Operand::Immediate(rhs)) => {
            let lhs = registers.get(reg);
            let newval = op.compute(lhs, rhs);
            if op != LogicalOp::Test {
                registers.set_imd(reg, newval);
            }
            flags.set_logical(newval);
        }
        _ => unimplemented!("{:?}", inst),
    }
}

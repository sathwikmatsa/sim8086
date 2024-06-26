use std::ops::AddAssign;

use crate::{
    disasm::Instruction,
    fields::{Data, EffectiveAddress, Operand, Operation, Register, Wide},
};

#[derive(Default)]
pub struct Clocks8086(pub usize);

impl AddAssign for Clocks8086 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

#[derive(Default)]
pub struct Clocks8088(pub usize);

impl AddAssign for Clocks8088 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Instruction {
    fn get_clocks_for_wide(
        &self,
        base: usize,
        transfers: usize,
        is_odd_ea: bool,
    ) -> (Clocks8086, Clocks8088) {
        let mut c86 = base;
        let mut c88 = base;

        if is_odd_ea {
            c86 += transfers * 4;
        }

        c88 += transfers * 4;

        (Clocks8086(c86), Clocks8088(c88))
    }

    pub fn clocks<F>(&self, is_ea_odd: F) -> (Clocks8086, Clocks8088)
    where
        F: Fn(EffectiveAddress) -> bool,
    {
        match self.operation {
            Operation::Add => {
                let first = self.first.expect("first operand exist for Add op");
                let second = self.second.expect("second operand exist for Add op");
                match (first, second) {
                    (Operand::Register(_), Operand::Register(_)) => (Clocks8086(3), Clocks8088(3)),
                    (Operand::Register(_), Operand::EffectiveAddress(ea)) => {
                        let base = 9 + ea.clocks();
                        match ea.wide() {
                            Wide::Word => self.get_clocks_for_wide(base, 1, is_ea_odd(ea)),
                            Wide::Byte => (Clocks8086(base), Clocks8088(base)),
                            _ => unreachable!(),
                        }
                    }
                    (Operand::EffectiveAddress(ea), Operand::Register(_)) => {
                        if ea.wide() == Wide::Byte {
                            unimplemented!()
                        }
                        self.get_clocks_for_wide(16 + ea.clocks(), 2, is_ea_odd(ea))
                    }
                    (Operand::Register(_), Operand::Immediate(_)) => (Clocks8086(4), Clocks8088(4)),
                    (Operand::EffectiveAddress(ea), Operand::Immediate(_)) => {
                        if ea.wide() == Wide::Byte {
                            unimplemented!()
                        }
                        self.get_clocks_for_wide(17 + ea.clocks(), 2, is_ea_odd(ea))
                    }
                    _ => unreachable!("{:?}", self),
                }
            }
            Operation::Mov => {
                let first = self.first.expect("first operand exist for Mov op");
                let second = self.second.expect("second operand exist for Mov op");
                match (first, second) {
                    (Operand::EffectiveAddress(ea), Operand::Register(Register::AX)) => {
                        self.get_clocks_for_wide(10, 1, is_ea_odd(ea))
                    }
                    (Operand::Register(Register::AX), Operand::EffectiveAddress(ea)) => {
                        self.get_clocks_for_wide(10, 1, is_ea_odd(ea))
                    }
                    (Operand::Register(_), Operand::Register(_)) => (Clocks8086(2), Clocks8088(2)),
                    (Operand::Register(_), Operand::EffectiveAddress(ea)) => {
                        self.get_clocks_for_wide(8 + ea.clocks(), 1, is_ea_odd(ea))
                    }
                    (Operand::EffectiveAddress(ea), Operand::Register(_)) => {
                        self.get_clocks_for_wide(9 + ea.clocks(), 1, is_ea_odd(ea))
                    }
                    (Operand::Register(_), Operand::Immediate(Data::U16(_))) => {
                        (Clocks8086(4), Clocks8088(4))
                    }
                    (Operand::EffectiveAddress(ea), Operand::Immediate(Data::U16(_))) => {
                        self.get_clocks_for_wide(10 + ea.clocks(), 1, is_ea_odd(ea))
                    }
                    (Operand::EffectiveAddress(ea), Operand::Immediate(Data::U8(_))) => {
                        let clocks = 10 + ea.clocks();
                        (Clocks8086(clocks), Clocks8088(clocks))
                    }
                    (Operand::SR(_), Operand::Register(_)) => (Clocks8086(2), Clocks8088(2)),
                    (Operand::SR(_), Operand::EffectiveAddress(ea)) => {
                        self.get_clocks_for_wide(8 + ea.clocks(), 1, is_ea_odd(ea))
                    }
                    (Operand::Register(_), Operand::SR(_)) => (Clocks8086(2), Clocks8088(2)),
                    (Operand::EffectiveAddress(ea), Operand::SR(_)) => {
                        self.get_clocks_for_wide(9 + ea.clocks(), 1, is_ea_odd(ea))
                    }
                    _ => unimplemented!("{:?}", self),
                }
            }
            Operation::TEST => {
                let first = self.first.expect("first operand exist for Test op");
                let second = self.second.expect("second operand exist for Test op");
                match (first, second) {
                    (Operand::Register(_), Operand::Register(_)) => (Clocks8086(3), Clocks8088(3)),
                    _ => unimplemented!("{:?}", self),
                }
            }
            Operation::XOR => {
                let first = self.first.expect("first operand exist for Xor op");
                let second = self.second.expect("second operand exist for Xor op");
                match (first, second) {
                    (Operand::Register(_), Operand::Register(_)) => (Clocks8086(3), Clocks8088(3)),
                    _ => unimplemented!("{:?}", self),
                }
            }
            Operation::INC => {
                let first = self.first.expect("first operand exist for Inc op");
                match first {
                    Operand::Register(reg) => {
                        let clocks = if reg.is_wide() { 2 } else { 3 };
                        (Clocks8086(clocks), Clocks8088(clocks))
                    }
                    _ => unimplemented!("{:?}", self),
                }
            }
            Operation::DEC => {
                let first = self.first.expect("first operand exist for Dec op");
                match first {
                    Operand::Register(reg) => {
                        let clocks = if reg.is_wide() { 2 } else { 3 };
                        (Clocks8086(clocks), Clocks8088(clocks))
                    }
                    _ => unimplemented!("{:?}", self),
                }
            }
            Operation::Cmp => {
                let first = self.first.expect("first operand exist for Cmp op");
                let second = self.second.expect("second operand exist for Cmp op");
                match (first, second) {
                    (Operand::Register(_), Operand::Register(_)) => (Clocks8086(3), Clocks8088(3)),
                    (Operand::Register(_), Operand::Immediate(_)) => (Clocks8086(4), Clocks8088(4)),
                    _ => unimplemented!("{:?}", self),
                }
            }
            Operation::Ret => (Clocks8086(8), Clocks8088(8)),
            Operation::SHR => {
                let first = self.first.expect("first operand exist for Shr op");
                let second = self.second.expect("second operand exist for Shr op");
                match (first, second) {
                    (Operand::Register(_), Operand::Immediate(_)) => (Clocks8086(2), Clocks8088(2)),
                    _ => unimplemented!("{:?}", self),
                }
            }
            _ => unimplemented!("{:?}", self),
        }
    }
}

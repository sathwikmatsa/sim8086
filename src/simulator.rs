use std::fmt::Display;

use crate::{
    fields::{Data, Operand, Operation, Register},
    instruction::Inst,
};

#[derive(Default)]
struct Registers {
    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,
    sp: u16,
    bp: u16,
    si: u16,
    di: u16,
}

trait SetBytes {
    fn set_low(&mut self, value: u8);
    fn set_high(&mut self, value: u8);
}

impl SetBytes for u16 {
    fn set_high(&mut self, value: u8) {
        *self &= 0x00FF;
        *self |= (value as u16) << 8;
    }

    fn set_low(&mut self, value: u8) {
        *self &= 0xFF00;
        *self |= value as u16;
    }
}

impl Registers {
    fn set(&mut self, reg: Register, imd: Data) {
        match reg {
            Register::AX => self.ax = u16::from(&imd),
            Register::BX => self.bx = u16::from(&imd),
            Register::CX => self.cx = u16::from(&imd),
            Register::DX => self.dx = u16::from(&imd),
            Register::SP => self.sp = u16::from(&imd),
            Register::BP => self.bp = u16::from(&imd),
            Register::SI => self.si = u16::from(&imd),
            Register::DI => self.di = u16::from(&imd),
            Register::AL => self
                .ax
                .set_low(u8::try_from(&imd).expect("u8 for 8bit registers")),
            Register::BL => self
                .bx
                .set_low(u8::try_from(&imd).expect("u8 for 8bit registers")),
            Register::CL => self
                .cx
                .set_low(u8::try_from(&imd).expect("u8 for 8bit registers")),
            Register::DL => self
                .dx
                .set_low(u8::try_from(&imd).expect("u8 for 8bit registers")),
            Register::AH => self
                .ax
                .set_high(u8::try_from(&imd).expect("u8 for 8bit registers")),
            Register::BH => self
                .bx
                .set_high(u8::try_from(&imd).expect("u8 for 8bit registers")),
            Register::CH => self
                .cx
                .set_high(u8::try_from(&imd).expect("u8 for 8bit registers")),
            Register::DH => self
                .dx
                .set_high(u8::try_from(&imd).expect("u8 for 8bit registers")),
        }
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Final registers:")?;
        writeln!(f, "      ax: {:#06X} ({})", self.ax, self.ax)?;
        writeln!(f, "      bx: {:#06X} ({})", self.bx, self.bx)?;
        writeln!(f, "      cx: {:#06X} ({})", self.cx, self.cx)?;
        writeln!(f, "      dx: {:#06X} ({})", self.dx, self.dx)?;
        writeln!(f, "      sp: {:#06X} ({})", self.sp, self.sp)?;
        writeln!(f, "      bp: {:#06X} ({})", self.bp, self.bp)?;
        writeln!(f, "      si: {:#06X} ({})", self.si, self.si)?;
        writeln!(f, "      di: {:#06X} ({})", self.di, self.di)?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Simulator {
    registers: Registers,
}

impl Simulator {
    pub fn exec(&mut self, inst: &Inst) {
        match inst.operation {
            Operation::Mov => {
                let first = inst.first.expect("mov has first operand");
                let second = inst.second.expect("mov has second operand");

                match (first, second) {
                    (Operand::Register(reg), Operand::Immediate(data)) => {
                        self.registers.set(reg, data)
                    }
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl Display for Simulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.registers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_set_16() {
        let mut regs = Registers::default();
        regs.set(Register::BX, Data::U16(8));
        assert_eq!(regs.bx, 8)
    }

    #[test]
    fn register_set_low() {
        let mut regs = Registers::default();
        regs.set(Register::BX, Data::U16(65535));
        regs.set(Register::BL, Data::U8(0));
        assert_eq!(regs.bx, 65280)
    }

    #[test]
    fn register_set_high() {
        let mut regs = Registers::default();
        regs.set(Register::BX, Data::U16(65535));
        regs.set(Register::BH, Data::U8(0));
        assert_eq!(regs.bx, 255)
    }

    #[test]
    fn simulator_move_immediate_to_register() {
        let mov = Inst::with_operands_v2(Operation::Mov, Register::BX, Data::U16(256));
        let mut simulator = Simulator::default();
        simulator.exec(&mov);
        assert_eq!(simulator.registers.bx, 256);
    }
}

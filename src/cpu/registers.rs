use std::fmt::Display;

use crate::fields::{Data, EffectiveAddress, Register, SegmentRegister};

#[derive(Default)]
pub struct Registers {
    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,
    sp: u16,
    bp: u16,
    si: u16,
    di: u16,
    es: u16,
    ss: u16,
    ds: u16,
    cs: u16,
}

trait GetSetBytes {
    fn set_low(&mut self, value: u8);
    fn set_high(&mut self, value: u8);
    fn get_low(&self) -> u8;
    fn get_high(&self) -> u8;
}

impl GetSetBytes for u16 {
    fn set_high(&mut self, value: u8) {
        *self &= 0x00FF;
        *self |= (value as u16) << 8;
    }

    fn set_low(&mut self, value: u8) {
        *self &= 0xFF00;
        *self |= value as u16;
    }

    fn get_high(&self) -> u8 {
        (*self >> 8) as u8
    }

    fn get_low(&self) -> u8 {
        ((*self << 8) >> 8) as u8
    }
}

impl Registers {
    pub fn dec_cx(&mut self) {
        self.cx = self.cx.checked_sub(1).unwrap();
    }

    pub fn cx(&self) -> u16 {
        self.cx
    }

    pub fn set_reg(&mut self, to: Register, from: Register) {
        let imd = self.get(from);
        self.set_imd(to, imd);
    }

    pub fn set_reg_from_sr(&mut self, reg: Register, sr: SegmentRegister) {
        let imd = self.get_sr(sr);
        self.set_imd(reg, imd);
    }

    pub fn set_sr_from_reg(&mut self, sr: SegmentRegister, reg: Register) {
        let imd = self.get(reg);
        self.set_sr_imd(sr, imd);
    }

    pub fn get_sr(&self, sr: SegmentRegister) -> Data {
        match sr {
            SegmentRegister::CS => Data::U16(self.cs),
            SegmentRegister::DS => Data::U16(self.ds),
            SegmentRegister::ES => Data::U16(self.es),
            SegmentRegister::SS => Data::U16(self.ss),
        }
    }

    pub fn get(&self, reg: Register) -> Data {
        match reg {
            Register::AX => Data::U16(self.ax),
            Register::BX => Data::U16(self.bx),
            Register::CX => Data::U16(self.cx),
            Register::DX => Data::U16(self.dx),
            Register::SP => Data::U16(self.sp),
            Register::BP => Data::U16(self.bp),
            Register::SI => Data::U16(self.si),
            Register::DI => Data::U16(self.di),
            Register::AL => Data::U8(self.ax.get_low()),
            Register::BL => Data::U8(self.bx.get_low()),
            Register::CL => Data::U8(self.cx.get_low()),
            Register::DL => Data::U8(self.dx.get_low()),
            Register::AH => Data::U8(self.ax.get_high()),
            Register::BH => Data::U8(self.bx.get_high()),
            Register::CH => Data::U8(self.cx.get_high()),
            Register::DH => Data::U8(self.dx.get_high()),
        }
    }

    pub fn set_sr_imd(&mut self, sr: SegmentRegister, imd: Data) {
        match sr {
            SegmentRegister::CS => self.cs = u16::from(&imd),
            SegmentRegister::DS => self.ds = u16::from(&imd),
            SegmentRegister::ES => self.es = u16::from(&imd),
            SegmentRegister::SS => self.ss = u16::from(&imd),
        }
    }

    pub fn set_imd(&mut self, reg: Register, imd: Data) {
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

    pub fn calculate_eff_addr(&self, ea: EffectiveAddress) -> u16 {
        match ea {
            EffectiveAddress::DirectAddress(addr, _) => addr,
            EffectiveAddress::BX(disp, _) => self.bx + disp.unwrap_or(0),
            EffectiveAddress::BP_SI(disp, _) => self.bp + self.si + disp.unwrap_or(0),
            EffectiveAddress::BP(disp, _) => self.bp + disp,
            EffectiveAddress::SI(disp, _) => self.si + disp.unwrap_or(0),
            EffectiveAddress::DI(disp, _) => self.di + disp.unwrap_or(0),
            _ => unimplemented!("{:?}", ea),
        }
    }
}

macro_rules! write_if_non_zero {
    ($f:expr, $self:ident, $field:ident) => {
        if $self.$field != 0 {
            writeln!(
                $f,
                "      {}: {:#06x} ({})",
                stringify!($field),
                $self.$field,
                $self.$field
            )?;
        }
    };
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Final registers:")?;
        write_if_non_zero!(f, self, ax);
        write_if_non_zero!(f, self, bx);
        write_if_non_zero!(f, self, cx);
        write_if_non_zero!(f, self, dx);
        write_if_non_zero!(f, self, sp);
        write_if_non_zero!(f, self, bp);
        write_if_non_zero!(f, self, si);
        write_if_non_zero!(f, self, di);
        write_if_non_zero!(f, self, cs);
        write_if_non_zero!(f, self, ds);
        write_if_non_zero!(f, self, ss);
        write_if_non_zero!(f, self, es);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_set_16() {
        let mut regs = Registers::default();
        regs.set_imd(Register::BX, Data::U16(8));
        assert_eq!(regs.bx, 8)
    }

    #[test]
    fn register_set_low() {
        let mut regs = Registers::default();
        regs.set_imd(Register::BX, Data::U16(65535));
        regs.set_imd(Register::BL, Data::U8(0));
        assert_eq!(regs.bx, 65280)
    }

    #[test]
    fn register_set_high() {
        let mut regs = Registers::default();
        regs.set_imd(Register::BX, Data::U16(65535));
        regs.set_imd(Register::BH, Data::U8(0));
        assert_eq!(regs.bx, 255)
    }
}

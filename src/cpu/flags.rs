use std::fmt::Display;

use crate::{
    fields::{Carry, Data, DataWithCarry, HalfCarry},
    handlers::ArithmeticOp,
};

#[derive(Default, PartialEq)]
pub struct Flags {
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
    pub carry: bool,
    pub overflow: bool,
    pub auxiliary: bool,
}

macro_rules! generate_flag_checks {
    ($($field:ident => $letter:expr),*) => {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut print_header = true;
            $(
                if self.$field {
                    // https://github.com/rust-lang/rust/issues/24580
                    #[allow(unused_assignments)]
                    if print_header {
                        write!(f, "   flags: ")?;
                        print_header = false;
                    }
                    write!(f, "{}", $letter)?;
                }
            )*
            writeln!(f)
        }
    };
}

impl Display for Flags {
    generate_flag_checks!(
        auxiliary => "A",
        carry => "C",
        overflow => "O",
        parity => "P",
        sign => "S",
        zero => "Z"
    );
}

impl Flags {
    pub fn set(&mut self, lhs: Data, rhs: Data, op: ArithmeticOp, computation: DataWithCarry) {
        let DataWithCarry(value, Carry(carry), HalfCarry(half_carry)) = computation;
        self.zero = value.is_zero();
        self.parity = value.is_lower_byte_even_parity();
        self.sign = value.is_signed();
        self.carry = carry;

        self.overflow = matches!(
            (lhs.is_signed(), rhs.is_signed(), op, value.is_signed()),
            (false, false, ArithmeticOp::Add, true)
                | (true, true, ArithmeticOp::Add, false)
                | (true, false, ArithmeticOp::Sub | ArithmeticOp::Cmp, false)
                | (false, true, ArithmeticOp::Sub | ArithmeticOp::Cmp, true)
        );

        self.auxiliary = half_carry;
    }

    pub fn set_logical(&mut self, value: Data) {
        self.zero = value.is_zero();
        self.parity = value.is_lower_byte_even_parity();
        self.sign = value.is_signed();
        self.carry = false;
        self.overflow = false;
        self.auxiliary = false;
    }
}

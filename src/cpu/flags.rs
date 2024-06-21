use std::fmt::Display;

use crate::fields::Data;

#[derive(Default, PartialEq)]
pub struct Flags {
    pub zero: bool,
    pub sign: bool,
    pub parity: bool,
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
        parity => "P",
        sign => "S",
        zero => "Z"
    );
}

impl Flags {
    pub fn set(&mut self, computation: Data) {
        self.zero = computation.is_zero();
        self.parity = computation.is_even_parity();
        self.sign = computation.is_signed();
    }
}

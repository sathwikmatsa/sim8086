#[macro_export]
macro_rules! conditional_advance {
    ($condition:expr, $err_str:expr, $self:ident, $inst:ident, $program:ident) => {
        if $condition {
            let first = $inst.first.expect(concat!($err_str, " has first operand"));
            let inc: Inc = first
                .try_into()
                .expect(concat!($err_str, " has Inc operand"));
            let nbytes: i16 = inc.into();
            $program.advance_by(nbytes);
            $self.ip = $self.ip.checked_add_signed(nbytes).unwrap();
        }
    };
}

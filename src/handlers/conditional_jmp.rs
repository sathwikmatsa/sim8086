#[macro_export]
macro_rules! conditional_advance {
    ($condition:expr, $err_str:expr, $self:ident, $inst:ident, $program:ident) => {{
        let (taken, not_taken) = if $self.estimate_cycles {
            let (JmpTakenClocks(a), JmpNotTakenClocks(b)) = $inst.clocks_for_coditional_advance();
            (a, b)
        } else {
            (0, 0)
        };
        if $condition {
            let first = $inst.first.expect(concat!($err_str, " has first operand"));
            let inc: Inc = first
                .try_into()
                .expect(concat!($err_str, " has Inc operand"));
            let nbytes: i16 = inc.into();
            $program.advance_by(nbytes);
            $self.ip = $self.ip.checked_add_signed(nbytes).unwrap();
            $self.cycles_8086 += Clocks8086(taken);
            $self.cycles_8088 += Clocks8088(taken);
        } else {
            $self.cycles_8086 += Clocks8086(not_taken);
            $self.cycles_8088 += Clocks8088(not_taken);
        }
    }};
}

pub fn main() {
    pub fn add_catch_carry(mut a: u32, b: u32) -> u32 {
        let carry: bool;
        unsafe {
            core::arch::asm!(
                "addl {rhs:e}, {lhs:e}",
                lhs = inlateout(reg) a,
                rhs = in(reg) b,
                flag_out("c") carry,
                //~^ ERROR 9:17: 9:36: flag_out operands for inline assembly are unstable [E0658]
                options(att_syntax, nostack, nomem, pure)
            );
        }
        if carry {
            panic!("carry");
        }
        return a;
    }
    add_catch_carry(1, 2);
}

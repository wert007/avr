
use mem::Address;
use regs::{self,Register,RegisterFile};
use mcu::Mcu;
use inst;

/// The AVR CPU.
pub struct Cpu
{
    register_file: RegisterFile,
}

impl Cpu
{
    pub fn new<M>() -> Self
        where M: Mcu
    {
        Cpu {
            register_file: M::register_file(),
        }
    }

    pub fn execute(&mut self, inst: inst::Instruction) {
        use inst::Instruction;
        use inst::{OpRd,OpRdK,OpRdRr};

        match inst {
            Instruction::Rd(op, rd) => match op {
                OpRd::Inc => self.inc(rd),
                OpRd::Dec => self.dec(rd),
                OpRd::Com => self.com(rd),
                OpRd::Neg => self.neg(rd),
                OpRd::Push => self.push(rd),
                OpRd::Pop => self.pop(rd),
                OpRd::Swap => self.swap(rd),
            },
            Instruction::RdK(op, rd, k) => match op {
                OpRdK::Subi => self.subi(rd, k),
                OpRdK::Sbci => self.sbci(rd, k),
                OpRdK::Andi => self.andi(rd, k),
                OpRdK::Ori => self.ori(rd, k),
                OpRdK::Cpi => self.cpi(rd, k),
                OpRdK::Ldi => self.ldi(rd, k),
            },
            Instruction::RdRr(op, rd, rr) => match op {
                OpRdRr::Add => self.add(rd, rr),
                OpRdRr::Adc => self.adc(rd, rr),
                OpRdRr::Sub => self.sub(rd, rr),
                OpRdRr::Sbc => self.sbc(rd, rr),
                OpRdRr::Mul => self.mul(rd, rr),
                OpRdRr::And => self.and(rd, rr),
                OpRdRr::Or => self.or(rd, rr),
                OpRdRr::Eor => self.eor(rd, rr),
                OpRdRr::Cpse => self.cpse(rd, rr),
                OpRdRr::Cp => self.cp(rd, rr),
                OpRdRr::Cpc => self.cpc(rd, rr),
                OpRdRr::Mov => self.mov(rd, rr),
            },
        }
    }

    /// lhs = lhs + rhs
    pub fn add(&mut self, lhs: u8, rhs: u8) {
        let sum = self.do_rdrr(lhs, rhs, |a,b| a+b);
        self.update_status_register(sum);
    }

    pub fn adc(&mut self, lhs: u8, rhs: u8) {
        unimplemented!()
    }

    /// lhs = lhs - rhs
    pub fn sub(&mut self, lhs: u8, rhs: u8) {
        let diff = self.do_rdrr(lhs, rhs, |a,b| a-b);
        self.update_status_register(diff);
    }

    pub fn sbc(&mut self, lhs: u8, rhs: u8) {
        unimplemented!()
    }

    pub fn subi(&mut self, rd: u8, imm: u8) {
        let diff = self.do_rdi(rd, |d| d-imm as u16);
        self.update_status_register(diff);
    }

    pub fn sbci(&mut self, rd: u8, imm: u8) {
        // TODO: set carry flag
        let diff = self.do_rdi(rd, |d| d-imm as u16);
        self.update_status_register(diff);
    }

    pub fn mul(&mut self, rd: u8, rr: u8) {
        unimplemented!()
    }

    pub fn and(&mut self, lhs: u8, rhs: u8) {
        self.do_rdrr(lhs, rhs, |a,b| a&b);
    }

    pub fn andi(&mut self, rd: u8, imm: u8) {
        self.do_rdi(rd, |d| d&imm as u16);
    }

    pub fn or(&mut self, lhs: u8, rhs: u8) {
        self.do_rdrr(lhs, rhs, |a,b| a|b);
    }

    pub fn ori(&mut self, rd: u8, imm: u8) {
        self.do_rdi(rd, |d| d&imm as u16);
    }

    pub fn eor(&mut self, lhs: u8, rhs: u8) {
        self.do_rdrr(lhs, rhs, |a,b| a^b);
    }

    pub fn com(&mut self, rd: u8) {
        self.do_rd(rd, |a| 0xff-a)
    }

    pub fn neg(&mut self, rd: u8) {
        self.do_rd(rd, |a| -(a as i8) as u8)
    }

    pub fn mov(&mut self, lhs: u8, rhs: u8) {
        self.do_rdrr(lhs, rhs, |_,b| b);
    }

    pub fn movw(&mut self, lhs: u8, rhs: u8) {
        self.do_rdrr16(lhs, rhs, |_,b| b)
    }

    pub fn lsl(&mut self, rd: u8) {
        self.do_rd(rd, |d| d<<1)
    }

    pub fn lsr(&mut self, rd: u8) {
        self.do_rd(rd, |d| d>>1)
    }

    pub fn inc(&mut self, rd: u8) {
        self.do_rd(rd, |d| d+1)
    }

    pub fn dec(&mut self, rd: u8) {
        self.do_rd(rd, |d| d-1)
    }

    pub fn push(&mut self, rd: u8) {
        unimplemented!()
    }

    pub fn pop(&mut self, rd: u8) {
        unimplemented!()
    }

    pub fn swap(&mut self, rd: u8) {
        self.do_rd(rd, |d| {
            let lo = d & 0x0f;
            let hi = d & 0xf0;

            (lo << 4) | (hi >> 4)
        })
    }

    pub fn cp(&mut self, rd: u8, rr: u8) {
        unimplemented!()
    }

    pub fn cpc(&mut self, rd: u8, rr: u8) {
        unimplemented!()
    }

    pub fn cpse(&mut self, rd: u8, rr: u8) {
        unimplemented!()
    }

    pub fn cpi(&mut self, rd: u8, imm: u8) {
        unimplemented!()
    }

    pub fn ldi(&mut self, rd: u8, imm: u8) {
        unimplemented!()
    }

    pub fn nop(&mut self) { }

    fn do_rd<F>(&mut self, rd: u8, mut f: F)
        where F: FnMut(u8) -> u8 {

        let rd_reg = self.register_file.gpr_mut(rd).unwrap();
        let rd_val = *rd_reg;

        *rd_reg = f(rd_val)
    }

    /// Returns the value of `rd` after execution.
    fn do_rdrr<F>(&mut self, rd: u8, rr: u8, mut f: F) -> u16
        where F: FnMut(u16,u16) -> u16 {

        let rr_val = *self.register_file.gpr(rr).unwrap() as u16;
        let rd_reg = self.register_file.gpr_mut(rd).unwrap();
        let rd_val = (*rd_reg) as u16;

        let val = f(rd_val, rr_val);
        *rd_reg = val as u8;
        val
    }

    fn do_rdi<F>(&mut self, rd: u8, mut f: F) -> u16
        where F: FnMut(u16) -> u16 {
        
        let rd_reg = self.register_file.gpr_mut(rd).unwrap();
        let rd_val = *rd_reg as u16;

        let val = f(rd_val);
        *rd_reg = val as u8;
        val
    }

    fn do_rdrr16<F>(&mut self, rd: u8, rr: u8,  mut f: F)
        where F: FnMut(u16,u16) -> u16 {
        assert!(rd % 2 == 0 && rr % 2 == 0,
                "GPR pairs must be even numbers");

        let rr_val_lo = *self.register_file.gpr(rr).unwrap() as u16;
        let rr_val_hi = *self.register_file.gpr(rr+1).unwrap() as u16;
        let rr_val = (rr_val_hi << 8) | rr_val_lo;

        let rd_val_lo = *self.register_file.gpr(rd).unwrap() as u16;
        let rd_val_hi = *self.register_file.gpr(rd+1).unwrap() as u16;
        let rd_val = (rd_val_hi << 8) | rd_val_lo;

        let val = f(rd_val, rr_val);
        let val_lo = (val & 0x0f) >> 0;
        let val_hi = (val & 0xf0) >> 8;

        *self.register_file.gpr_mut(rd).unwrap() = val_lo as u8;
        *self.register_file.gpr_mut(rd+1).unwrap() = val_hi as u8;
    }

    /// Updates the `V`, `C`, `H`, `N`, `Z`, and `S` status flags.
    fn update_status_register(&mut self, val: u16) {
        self.update_overflow_flag(val);
        self.update_carry_flag(val);
        self.update_half_carry_flag(val);
        self.update_negative_flag(val);
        self.update_zero_flag(val);
    }

    /// Sets the overflow flag if `val` overflows a `u8`.
    fn update_overflow_flag(&mut self, val: u16) {
        let overflowed = val>0xff;
        self.update_status_flag_bit(regs::OVERFLOW_MASK, overflowed)
    }

    /// Sets the carry flag if necessary.
    fn update_carry_flag(&mut self, val: u16) {
        let is_carry = (val&0b100000000)>0;
        self.update_status_flag_bit(regs::CARRY_MASK, is_carry)
    }

    /// Sets the half carry flag if necessary.
    fn update_half_carry_flag(&mut self, val: u16) {
        let is_hcarry = (val & 0b1000)>0;
        self.update_status_flag_bit(regs::HALF_CARRY_MASK, is_hcarry)
    }

    /// Sets the negative flag based on `val`.
    fn update_negative_flag(&mut self, val: u16) {
        let is_negative = (val & 0b10000000)>0;
        self.update_status_flag_bit(regs::NEGATIVE_MASK, is_negative)
    }

    fn update_zero_flag(&mut self, val: u16) {
        let is_zero = val==0;
        self.update_status_flag_bit(regs::ZERO_MASK, is_zero)
    }

    fn update_status_flag_bit(&mut self, mask: u8, val: bool) {
        let sreg = self.register_file.sreg_mut();

        if val == true {
            *sreg |= mask;
        } else {
            *sreg &= !mask;
        }

        // TODO: update S flag. should be `N xor V`.
    }
}

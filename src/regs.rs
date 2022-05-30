use crate::{Error, SReg};

// TODO: s/addr/num

/// `SP` low register number.
pub const SP_LO_NUM: u8 = 32;
/// `SP` high register number.
pub const SP_HI_NUM: u8 = 33;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Register {
    pub name: String,
    pub value: u8,
}

/// The register file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegisterFile {
    registers: Vec<Register>,
    pub sreg: SReg,
}

impl RegisterFile {
    pub fn new(registers: Vec<Register>) -> Self {
        RegisterFile {
            registers,
            sreg: SReg::new(),
        }
    }

    pub fn registers(&self) -> ::std::slice::Iter<Register> {
        self.registers.iter()
    }

    /// Gets a register, or `None` if it doesn't exist.
    pub fn gpr(&self, addr: u8) -> Result<u8, Error> {
        self.registers
            .get(addr as usize)
            .map(|r| r.value)
            .ok_or(Error::RegisterDoesNotExist(addr))
    }

    /// Gets a mutable register, or `None` if it doesn't exist.
    pub fn gpr_mut(&mut self, addr: u8) -> Result<&mut u8, Error> {
        self.registers
            .get_mut(addr as usize)
            .map(|r| &mut r.value)
            .ok_or(Error::RegisterDoesNotExist(addr))
    }

    pub fn gpr_pair(&self, addr: u8) -> Result<(u8, u8), Error> {
        if addr % 2 != 0 {
            return Err(Error::RegisterPairOdd(addr));
        }

        let lo = self.gpr(addr)?;
        let hi = self.gpr(addr + 1)?;

        Ok((lo, hi))
    }

    pub fn gpr_pair_val(&self, addr: u8) -> Result<u16, Error> {
        let (lo, hi) = self.gpr_pair(addr)?;
        let val = ((hi as u16) << 8) | lo as u16;
        Ok(val)
    }

    pub fn set_gpr_pair(&mut self, low: u8, val: u16) {
        let val_lo = (val & 0x00ff) as u8;
        let val_hi = ((val & 0xff00) >> 8) as u8;

        *self.gpr_mut(low).unwrap() = val_lo;
        *self.gpr_mut(low + 1).unwrap() = val_hi;
    }

    /// Checks if a flag is set in SREG.
    pub fn sreg_flag(&self, mask: u8) -> bool {
        (self.sreg.0.value & mask) == mask
    }

    pub fn sreg_flag_set(&mut self, mask: u8) {
        self.sreg.0.value |= mask;
    }

    pub fn sreg_flag_clear(&mut self, mask: u8) {
        self.sreg.0.value &= !mask;
    }
}

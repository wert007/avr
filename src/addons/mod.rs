pub use self::uart::Uart;
use crate::{Core, Error, Instruction};
pub mod instruction_listener;
pub mod uart;

pub trait Addon {
    fn tick(&mut self, core: &mut Core, inst: Instruction, pc: u32) -> Result<(), Error>;
}

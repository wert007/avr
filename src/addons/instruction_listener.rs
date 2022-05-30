use crate::{Error, Instruction};

pub struct InstructionListener {}

impl crate::Addon for InstructionListener {
    fn tick(&mut self, _core: &mut crate::Core, inst: Instruction, pc: u32) -> Result<(), Error> {
        println!("{:5X}: Executing {:?}", pc, inst);
        Ok(())
    }
}

/// An error on the AVR.
#[derive(Debug)]
pub enum Error {
    UnknownInstruction(u32),
    StackOverflow,
    SegmentationFault { address: usize },
    RegisterDoesNotExist(u8),
    RegisterPairOdd(u8),
}

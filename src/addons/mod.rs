pub use self::uart::Uart;
use crate::Core;
pub mod uart;

pub trait Addon {
    fn tick(&mut self, core: &mut Core);
}

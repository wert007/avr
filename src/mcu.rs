use crate::addons;
use crate::{Core, Error};

pub struct Mcu {
    pub core: Core,
    addons: Vec<Box<dyn addons::Addon>>,
}

impl Mcu {
    pub fn new(core: Core) -> Self {
        Mcu {
            core,
            addons: Vec::new(),
        }
    }

    pub fn attach(&mut self, addon: Box<dyn addons::Addon>) {
        self.addons.push(addon);
    }

    pub fn tick(&mut self) -> Result<(), Error> {
        let (inst, pc) = self.core.tick()?;

        for addon in self.addons.iter_mut() {
            let _ = addon.tick(&mut self.core, inst, pc);
        }

        Ok(())
    }
}

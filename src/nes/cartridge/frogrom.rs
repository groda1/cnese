use std::boxed::Box;
use super::cartridge::CartridgeTrait;

const SIZE: usize = 49120;
const OFFSET: u16 = 0x4020;

pub struct FrogRom {
    rom: Box<[u8; SIZE]>,
}

impl FrogRom {
    pub fn new(filerom: &[u8]) -> FrogRom {
        let mut rom = Box::new([0xff as u8; SIZE]);

        rom.copy_from_slice(filerom);
        FrogRom {
            rom
        }
    }
}

impl CartridgeTrait for FrogRom {
    fn read(&self, address: u16) -> u8 {
        self.rom[(address - OFFSET) as usize]
    }

    fn read_slice(&self, address: u16, len: usize) -> &[u8] {
        let addr = (address - OFFSET) as usize;
        &self.rom[addr..addr + len]
    }

    fn write(&mut self, address: u16, data: u8) {
        unimplemented!()
    }
}

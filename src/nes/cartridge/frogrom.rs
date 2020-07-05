use std::boxed::Box;
use super::cartridge::CartridgeTrait;
use super::cartridge::{CARTRIDGE_OFFSET, CARTRIDGE_MAX_SIZE};

pub struct FrogRom {
    rom: Box<[u8; CARTRIDGE_MAX_SIZE]>,
}

impl FrogRom {
    pub fn new(filerom: &[u8]) -> FrogRom {
        let mut rom = Box::new([0xff as u8; CARTRIDGE_MAX_SIZE]);

        rom.copy_from_slice(filerom);
        FrogRom {
            rom
        }
    }
}

impl CartridgeTrait for FrogRom {
    fn read_prg(&self, address: u16) -> u8 {
        self.rom[(address - CARTRIDGE_OFFSET) as usize]
    }

    fn write_prg(&mut self, _address: u16, _data: u8) {
        unimplemented!()
    }

    fn read_chr(&self, _address: u16) -> u8 {
        unimplemented!()
    }

    fn read_chr_slice(&self, _address: u16, _len: usize) -> &[u8] {
        unimplemented!()
    }

    fn get_instruction_offset(&self) -> u16 { CARTRIDGE_OFFSET }
}

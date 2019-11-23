use super::nrom::NRom;
use super::frogrom::FrogRom;

use super::super::ines::PRG_ROM_CHUNK_SIZE;

pub trait CartridgeTrait {
    fn read(&self, address: u16) -> u8;
    fn read_slice(&self, address: u16, len: usize) -> &[u8];
    fn write(&mut self, address: u16, data: u8);
}

pub struct Cartridge {
    cartridge: Box<dyn CartridgeTrait>,
}

impl Cartridge {
    pub fn read(&self, address: u16) -> u8 {
        self.cartridge.read(address)
    }

    pub fn read_slice(&self, address: u16, len: usize) -> &[u8] {
        self.cartridge.read_slice(address, len)
    }

    pub fn write(&mut self, address:u16, data: u8 ) {
        self.cartridge.write(address, data);
    }
}

pub fn create_cartridge_from_ines(mapper: u8, prg_rom: Vec<&[u8]>) -> Result<Cartridge, &'static str> {
    match mapper {
        0 => Ok(Cartridge { cartridge: Box::new(NRom::new(prg_rom)) }),
        _ => Err("Unsupported mapper")
    }
}

pub fn create_cartridge_from_raw(data: &[u8]) -> Result<Cartridge, &'static str> {
    Ok(Cartridge { cartridge: Box::new(FrogRom::new(data)) })
}


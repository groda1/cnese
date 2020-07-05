use super::nrom::NRom;
use super::frogrom::FrogRom;

pub const CARTRIDGE_OFFSET: u16 = 0x4020;
pub const CARTRIDGE_MAX_SIZE: usize = 0x10000 - CARTRIDGE_OFFSET as usize;

pub trait CartridgeTrait {
    fn read_prg(&self, address: u16) -> u8;
    fn read_prg_slice(&self, address: u16, len: usize) -> &[u8];
    fn write_prg(&mut self, address: u16, data: u8);

    fn read_chr(&self, address: u16) -> u8;
    fn read_chr_slice(&self, address: u16, len: usize) -> &[u8];
}

pub struct Cartridge {
    cartridge: Box<dyn CartridgeTrait>,
}

impl Cartridge {
    pub fn read_prg(&self, address: u16) -> u8 {
        self.cartridge.read_prg(address)
    }
    pub fn read_prg_slice(&self, address: u16, len: usize) -> &[u8] {
        self.cartridge.read_prg_slice(address, len)
    }
    pub fn write_prg(&mut self, address:u16, data: u8 ) {
        self.cartridge.write_prg(address, data);
    }

    pub fn read_chr(&self, address: u16) -> u8 {
        self.cartridge.read_chr(address)
    }
    pub fn read_chr_slice(&self, address: u16, len: usize) -> &[u8] {
        self.cartridge.read_chr_slice(address, len)
    }
}

pub fn create_cartridge_from_ines(mapper: u8, prg_rom: Vec<&[u8]>) -> Result<Cartridge, String> {
    match mapper {
        0 => Ok(Cartridge { cartridge: Box::new(NRom::new(prg_rom)) }),
        _ => Err(format!("Unsupported mapper: {}", mapper))
    }
}

pub fn create_cartridge_from_raw(data: &[u8]) -> Result<Cartridge, String> {
    Ok(Cartridge { cartridge: Box::new(FrogRom::new(data)) })
}


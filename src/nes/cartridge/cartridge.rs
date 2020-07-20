use super::nrom::NRom;
use super::frogrom::FrogRom;
use crate::nes::cartridge::cartridge::Mirroring::{Horizontal, Vertical};
use crate::ppu::nametable::Mirroring;

pub const CARTRIDGE_OFFSET: u16 = 0x4020;
pub const CARTRIDGE_MAX_SIZE: usize = 0x10000 - CARTRIDGE_OFFSET as usize;


pub trait CartridgeTrait {
    fn read_prg(&self, address: u16) -> u8;
    fn write_prg(&mut self, address: u16, data: u8);

    fn read_chr(&self, address: u16) -> u8;
    fn read_chr_slice(&self, address: u16, len: usize) -> &[u8];
    fn write_chr(&mut self, address: u16, data: u8);

    fn get_instruction_offset(&self) -> u16;
}

pub struct Cartridge {
    implementation: Box<dyn CartridgeTrait>,
    instruction_offset: u16,
    mirroring: Mirroring,
}

impl Cartridge {
    fn new(cartridge: Box<dyn CartridgeTrait>, mirroring: Mirroring) -> Cartridge {
        let instruction_offset = cartridge.get_instruction_offset();
        Cartridge {
            implementation: cartridge,
            instruction_offset,
            mirroring,
        }
    }

    pub fn read_prg(&self, address: u16) -> u8 {
        self.implementation.read_prg(address)
    }
    pub fn write_prg(&mut self, address: u16, data: u8) {
        self.implementation.write_prg(address, data);
    }

    pub fn read_chr(&self, address: u16) -> u8 {
        self.implementation.read_chr(address)
    }
    pub fn read_chr_slice(&self, address: u16, len: usize) -> &[u8] {
        self.implementation.read_chr_slice(address, len)
    }

    pub fn write_chr(&mut self, address: u16, data: u8) {
        self.implementation.write_chr(address, data);
    }

    pub fn get_instruction_offset(&self) -> u16 { self.instruction_offset }
    pub fn get_mirroring(&self) -> Mirroring { self.mirroring }
}

pub fn create_cartridge_from_ines(mapper: u8, prg_rom: Vec<&[u8]>,
                                  chr_rom: Vec<&[u8]>,
                                  mirroring: u8) -> Result<Cartridge, String> {
    match mapper {
        0 => Ok(Cartridge::new(Box::new(NRom::new(prg_rom, chr_rom[0])),
                               if mirroring == 0 { Mirroring::Horizontal } else { Mirroring::Vertical })),
        _ => Err(format!("Unsupported mapper: {}", mapper))
    }
}

pub fn create_cartridge_from_raw(data: &[u8]) -> Result<Cartridge, String> {
    Ok(Cartridge::new(Box::new(FrogRom::new(data)),
                      Mirroring::Horizontal))
}




use std::boxed::Box;
use super::cartridge::CartridgeTrait;

use crate::nes::ines;

const PRG_RAM_SIZE: usize = 0x2000;
const PRG_ROM_SIZE: usize = 0x8000;

const PRG_RAM_START: u16 = 0x6000;
const PRG_RAM_END: u16 = 0x7FFF;
const PRG_ROM_START: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xFFFF;

const CHR_ROM_SIZE: usize = 0x2000;

pub struct NRom {
    /*
    CPU $6000-$7FFF: Family Basic only: PRG RAM, mirrored as necessary to fill entire 8 KiB window, write protectable with an external switch
    CPU $8000-$BFFF: First 16 KB of ROM.
    CPU $C000-$FFFF: Last 16 KB of ROM (NROM-256) or mirror of $8000-$BFFF (NROM-128).
    */

    prg_ram: Box<[u8; PRG_RAM_SIZE]>,
    prg_rom: Box<[u8; PRG_ROM_SIZE]>,
    chr_rom: Box<[u8; CHR_ROM_SIZE]>

}

impl NRom {
    pub fn new(ines_prg_vec: Vec<&[u8]>, ines_chr_rom: &[u8]) -> NRom {
        let prg_ram = Box::new([0 as u8; PRG_RAM_SIZE]);
        let mut prg_rom = Box::new([0 as u8; PRG_ROM_SIZE]);
        let mut chr_rom = Box::new([0 as u8; CHR_ROM_SIZE]);

        match ines_prg_vec.len() {
            1 => {
                prg_rom[0..ines::PRG_ROM_CHUNK_SIZE].copy_from_slice(&ines_prg_vec[0]);
                prg_rom[ines::PRG_ROM_CHUNK_SIZE..PRG_ROM_SIZE].copy_from_slice(&ines_prg_vec[0]);
            }
            2 => {
                prg_rom[0..ines::PRG_ROM_CHUNK_SIZE].copy_from_slice(&ines_prg_vec[0]);
                prg_rom[ines::PRG_ROM_CHUNK_SIZE..PRG_ROM_SIZE].copy_from_slice(&ines_prg_vec[1]);
            }
            _ => unreachable!()
        }

        chr_rom.copy_from_slice(ines_chr_rom);

        NRom {
            prg_ram,
            prg_rom,
            chr_rom
        }
    }
}

impl CartridgeTrait for NRom {
    fn read_prg(&self, address: u16) -> u8 {
        match address {
            PRG_RAM_START..=PRG_RAM_END => {
                self.prg_ram[(address - PRG_RAM_START) as usize]
            }
            PRG_ROM_START..=PRG_ROM_END => {
                self.prg_rom[(address - PRG_ROM_START) as usize]
            }
            _ => unreachable!()
        }
    }

    fn write_prg(&mut self, address: u16, data: u8) {
        match address {
            PRG_RAM_START..=PRG_RAM_END => {
                self.prg_ram[(address - PRG_RAM_START) as usize] = data;
            }
            PRG_ROM_START..=PRG_ROM_END => {
                self.prg_rom[(address - PRG_ROM_START) as usize] = data;
            }
            _ => unreachable!()
        }
    }

    fn read_chr(&self, address: u16) -> u8 {
        self.chr_rom[address as usize]
    }

    fn read_chr_slice(&self, address: u16, len: usize) -> &[u8] {
        let start = address as usize;
        &self.chr_rom[start..start + len]
    }

    fn get_instruction_offset(&self) -> u16 { PRG_ROM_START }
}

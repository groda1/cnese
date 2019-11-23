use std::boxed::Box;
use super::cartridge::CartridgeTrait;


const PRG_RAM_SIZE: usize = 0x2000;
const PRG_ROM_SIZE: usize = 0x8000;


pub struct NRom {
    /*
    CPU $6000-$7FFF: Family Basic only: PRG RAM, mirrored as necessary to fill entire 8 KiB window, write protectable with an external switch
    CPU $8000-$BFFF: First 16 KB of ROM.
    CPU $C000-$FFFF: Last 16 KB of ROM (NROM-256) or mirror of $8000-$BFFF (NROM-128).
    */

    prg_ram: Box<[u8; PRG_RAM_SIZE]>,
    prg_rom: Box<[u8; PRG_ROM_SIZE]>,

}

impl NRom {
    pub fn new(ines_prg_vec: Vec<&[u8]>) -> NRom {
        let prg_ram = Box::new([0 as u8; PRG_RAM_SIZE]);
        let mut prg_rom = Box::new([0 as u8; PRG_ROM_SIZE]);

        match ines_prg_vec.len() {
            1 => {
                prg_rom[0..0x4000].copy_from_slice(&ines_prg_vec[0]);
                prg_rom[0x4000..PRG_ROM_SIZE].copy_from_slice(&ines_prg_vec[0]);
            }
            2 => {
                prg_rom[0..0x4000].copy_from_slice(&ines_prg_vec[0]);
                prg_rom[0x4000..PRG_ROM_SIZE].copy_from_slice(&ines_prg_vec[1]);
            }
            _ => unreachable!()
        }

//        prg_rom.iter().for_each(|b| print!("{:02X}", b));
//        println!();

        NRom {
            prg_ram,
            prg_rom,
        }
    }
}

impl CartridgeTrait for NRom {
    fn read(&self, address: u16) -> u8 {
        unimplemented!()
    }

    fn read_slice(&self, address: u16, len: usize) -> &[u8] {
        unimplemented!()
    }

    fn write(&mut self, address: u16, data: u8) {
        unimplemented!()
    }
}

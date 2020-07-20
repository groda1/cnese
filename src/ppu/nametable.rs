use crate::ppu::nametable::Mirroring::{Horizontal, Vertical};

pub const START_ADDRESS: u16 = 0x2000;
pub const END_ADDRESS: u16 = 0x2FFF;
pub const MIRROR_START_ADDRESS:u16 = 0x3000;
pub const MIRROR_END_ADDRESS:u16 = 0x3EFF;

const NAMETABLE_SIZE: usize = 0x400;

pub struct NametableMemory {
    memory: [u8; NAMETABLE_SIZE * 2],
    mirroring: Mirroring,
}

impl NametableMemory {
    pub fn new(mirroring: Mirroring) -> NametableMemory {
        NametableMemory {
            memory: [0; NAMETABLE_SIZE * 2],
            mirroring,
        }
    }

    fn _calc_address(&self, address: u16) -> usize {
        match &self.mirroring {
            Mirroring::Horizontal => {
                let offset = (address - 0x2000) / 0x800;
                ((address % 0x400) + (offset * NAMETABLE_SIZE as u16)) as usize
            }
            Mirroring::Vertical => {
                (address % 0x800) as usize
            }
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            START_ADDRESS..=END_ADDRESS => {
                let raw_address = self._calc_address(address);
                self.memory[raw_address]
            }
            _ => unreachable!()

        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            START_ADDRESS..=END_ADDRESS => {
                let raw_address = self._calc_address(address);
                self.memory[raw_address] = data;
            }
            _ =>  unreachable!()
        }
    }
}

#[derive(Clone, Copy)]
pub enum Mirroring {
    Horizontal = 0,
    Vertical = 1,
}


#[cfg(test)]
#[test]
fn test_horizontal() {
    let mem = NametableMemory::new(Horizontal);

    assert_eq!(mem._calc_address(0x2000), mem._calc_address(0x2400));
    assert_eq!(mem._calc_address(0x2001), mem._calc_address(0x2401));
    assert_eq!(mem._calc_address(0x23ff), mem._calc_address(0x27ff));

    assert_eq!(mem._calc_address(0x2800), mem._calc_address(0x2c00));
    assert_eq!(mem._calc_address(0x2801), mem._calc_address(0x2c01));
    assert_eq!(mem._calc_address(0x2bff), mem._calc_address(0x2fff));

    assert_ne!(mem._calc_address(0x2000), mem._calc_address(0x2800));
    assert_ne!(mem._calc_address(0x2001), mem._calc_address(0x2801));
    assert_ne!(mem._calc_address(0x23ff), mem._calc_address(0x2bff));

    assert_ne!(mem._calc_address(0x2400), mem._calc_address(0x2c00));
    assert_ne!(mem._calc_address(0x2401), mem._calc_address(0x2c01));
    assert_ne!(mem._calc_address(0x27ff), mem._calc_address(0x2bff));
}

#[test]
fn test_vertical() {
    let mem = NametableMemory::new(Vertical);

    assert_eq!(mem._calc_address(0x2000), mem._calc_address(0x2800));
    assert_eq!(mem._calc_address(0x2001), mem._calc_address(0x2801));
    assert_eq!(mem._calc_address(0x23ff), mem._calc_address(0x2bff));

    assert_eq!(mem._calc_address(0x2400), mem._calc_address(0x2c00));
    assert_eq!(mem._calc_address(0x2401), mem._calc_address(0x2c01));
    assert_eq!(mem._calc_address(0x27ff), mem._calc_address(0x2fff));

    assert_ne!(mem._calc_address(0x2000), mem._calc_address(0x2400));
    assert_ne!(mem._calc_address(0x2001), mem._calc_address(0x2401));
    assert_ne!(mem._calc_address(0x23ff), mem._calc_address(0x27ff));

    assert_ne!(mem._calc_address(0x2800), mem._calc_address(0x2C00));
    assert_ne!(mem._calc_address(0x2801), mem._calc_address(0x2c01));
    assert_ne!(mem._calc_address(0x2bff), mem._calc_address(0x2fff));
}


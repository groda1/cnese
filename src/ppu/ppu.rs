use std::rc::Rc;
use std::cell::RefCell;

use crate::nes::cartridge::cartridge::Cartridge;
use super::register;

const PATTERN_TABLE_SIZE: usize = 0x1000;


pub struct Ppu {
    cartridge: Rc<RefCell<Cartridge>>,
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,

    oamaddr: u8,
    // TODO oamdata

    ppuscroll: u16,
    ppuaddr:u16,
    latch: Option<u8>,


}

impl Ppu {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Ppu {
        Ppu {
            cartridge,
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            latch: None
        }
    }

    pub fn write_register(&mut self, address: u16, data: u8) {
        match address % register::REGISTER_SIZE {

            register::PPUCTRL_OFFSET => {
                self.ppuctrl = data;
            }
            register::PPUMASK_OFFSET => {
                self.ppumask = data;
            }

            _=> unreachable!()
        }
    }
    pub fn read_register(&mut self, address: u16) -> u8 {
        match address % register::REGISTER_SIZE {
            register::PPUSTATUS_OFFSET => {
                let status = self.ppustatus;
                // TODO CLEAR VBLANK
                self.latch = None;

                status
            }
            _=> unreachable!()
        }
    }

    pub fn patterntable_to_texture_data(&self, pattern_table_index: u8) -> [u8; 16384] {
        const PATTERN_TABLE_TILE_COUNT: usize = 256;
        const PATTERN_TABLE_TILE_WIDTH: usize = 8;
        const PATTERN_TABLE_TILE_HEIGHT: usize = 8;
        const PATTERN_TABLE_PLANE_SIZE: usize = 8;
        const PATTERN_TABLE_BYTES_PER_TILE: usize = PATTERN_TABLE_PLANE_SIZE * 2;

        const TEXTURE_DATA_WIDTH: usize = 128; // 128 x 128 values
        const TEXTURE_TILE_COUNT_WIDTH: usize = TEXTURE_DATA_WIDTH / PATTERN_TABLE_TILE_WIDTH;

        let mut data = [0 as u8; PATTERN_TABLE_SIZE];
        data.copy_from_slice(self.cartridge.borrow()
            .read_chr_slice(pattern_table_index as u16 * PATTERN_TABLE_SIZE as u16, PATTERN_TABLE_SIZE));

        let mut target = [0; PATTERN_TABLE_TILE_COUNT * PATTERN_TABLE_TILE_WIDTH * PATTERN_TABLE_TILE_HEIGHT];

        // For every tile
        for i in 0..PATTERN_TABLE_TILE_COUNT {
            // For every line in pattern
            for j in 0..PATTERN_TABLE_TILE_HEIGHT {
                let lo_plane = data[i * PATTERN_TABLE_BYTES_PER_TILE + j];
                let hi_plane = data[i * PATTERN_TABLE_BYTES_PER_TILE + j + PATTERN_TABLE_PLANE_SIZE];

                // For every pixel in line
                for k in 0..PATTERN_TABLE_TILE_WIDTH {
                    let val = ((lo_plane >> k as u8) & 1) + (((hi_plane >> k as u8) & 1) << 1);

                    let x = (i % TEXTURE_TILE_COUNT_WIDTH * PATTERN_TABLE_TILE_WIDTH) + PATTERN_TABLE_TILE_WIDTH - 1 - k;
                    let y = (PATTERN_TABLE_TILE_HEIGHT * (i / TEXTURE_TILE_COUNT_WIDTH)) + j;

                    let target_index = (y * TEXTURE_DATA_WIDTH + x) as usize;

                    // println!("Target index {}, i={}, j={}, k={}, x={}, y={}", target_index, i,j,k,x,y);
                    target[target_index] = val;
                }
            }
            // println!("-----------");
        }

        // unreachable!();
        target
    }

    fn _read_memory(&self, address:u16) -> u8 {
        match address {
            0..=0x1FFF => {
                self.cartridge.borrow().read_chr(address)
            }
            0x2000..=0x2FFF => {
                unimplemented!()
            }
            0x3000..=0x3EFF => {
                // TODO ?
                unimplemented!()
            }
            0x3F00..=0x3FFF => {
                // TODO Palette RAM indexes
                // TODO Mirror
                unimplemented!()
            }
            _ => {
                unreachable!()
            }
        }
    }

    pub fn get_ppuctrl(&self) -> u8 {
        self.ppuctrl
    }

    pub fn get_ppumask(&self) -> u8 {
        self.ppumask
    }

    pub fn get_ppustatus(&self) -> u8 {
        self.ppustatus
    }

    pub fn get_oamaddr(&self) -> u8 {
        self.oamaddr
    }

    pub fn get_ppuscroll(&self) -> u16 {
        self.ppuscroll
    }

    pub fn get_ppuaddr(&self )-> u16 {
        self.ppuaddr
    }
}
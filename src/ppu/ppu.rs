use crate::nes::cartridge::cartridge::Cartridge;
use std::rc::Rc;
use std::cell::RefCell;

const PATTERN_TABLE_SIZE: usize = 0x1000;

pub struct Ppu {
    cartridge: Rc<RefCell<Cartridge>>
}


impl Ppu {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Ppu {
        Ppu { cartridge }
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
}
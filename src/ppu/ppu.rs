use crate::nes::cartridge::cartridge::Cartridge;
use std::rc::Rc;
use std::cell::RefCell;
use std::hint::unreachable_unchecked;

const PATTERN_TABLE_SIZE: usize = 0x1000;

pub struct Ppu {
    cartridge: Rc<RefCell<Cartridge>>
}


impl Ppu {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Ppu {
        Ppu { cartridge }
    }


    pub fn patterntable_to_texture_data(&self, pattern_table_index: u8) -> [u8; 16384] {
        // TODO Clean up this horrible method

        const TEXTURE_DATA_WIDTH: usize = 128; // 128 x 128 values

        // 256 tiles, 8x8 pixels

        let mut data = [0 as u8; PATTERN_TABLE_SIZE];

        data.copy_from_slice(self.cartridge.borrow()
            .read_chr_slice(pattern_table_index as u16 * PATTERN_TABLE_SIZE as u16, PATTERN_TABLE_SIZE));

        let mut target = [0; 256 * 8 * 8];

        // TODO loop over target instead


        // For every tiles
        for i in 0..256 {

            // println!("char {:x}", i);
            // For every line in pattern
            for j in 0..8 as usize {
                let lo_plane = data[i * 16 + j];
                let hi_plane = data[i  * 16 + j + 8];

                // println!("{:08b}", lo_plane);

                // For every pixel in line
                for k in 0..8 as usize {
                    let val =((lo_plane >> k as u8) & 1) + (((hi_plane >> k as u8) & 1) << 1);

                    let x = (i * 8 % 128) + 7 - k;
                    let y = (8 * (i / 16)) + j;

                    let target_index = (y * 128 + x) as usize;

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
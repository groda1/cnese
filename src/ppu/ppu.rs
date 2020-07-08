use crate::nes::cartridge::cartridge::Cartridge;
use super::register;

const PATTERN_TABLE_SIZE: usize = 0x1000;

const SCANLINE_PRE_RENDER: u16 = 261;
const SCANLINE_VISIBLE_START: u16 = 0;
const SCANLINE_VISIBLE_END: u16 = 239;
const SCANLINE_POST_RENDER: u16 = 240;
const SCANLINE_VBLANK_START: u16 = 241;
const SCANLINE_VBLANK_END: u16 = 260;

const SCANLINE_CYCLE_COUNT: u16 = 341;

const PIXEL_OUTPUT_CYCLE_OFFSET: u16 = 4;

pub const FRAMEBUFFER_WIDTH: usize = 256;
pub const FRAMEBUFFER_HEIGHT: usize = 240;
const FRAMEBUFFER_SIZE: usize = FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT;

pub struct Ppu {
    cartridge_ptr: *mut Cartridge,
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,

    oamaddr: u8,
    // TODO oamdata

    ppuscroll: u16,
    ppuaddr: u16,
    latch: Option<u8>,

    // rendering variables
    scanline: u16,
    scanline_cycle: u16,

    framecount: u64,

    framebuffer: [u8; FRAMEBUFFER_SIZE],
}

impl Ppu {
    pub fn new(cartridge_ptr: *mut Cartridge) -> Ppu {
        Ppu {
            cartridge_ptr,
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            latch: None,

            scanline: SCANLINE_PRE_RENDER,
            scanline_cycle: 0,
            framecount: 0,
            framebuffer: [0; FRAMEBUFFER_SIZE],
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
            register::OAMADDR_OFFSET => {
                self.oamaddr = data;
            }
            register::PPUADDR_OFFSET => {
                self._write_ppuaddr(data);
            }
            _ => {
                println!("CRASH ppu::write_register ${:04X} = ${:02X}", address, data);
                unreachable!()
            }
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
            _ => unreachable!()
        }
    }

    pub fn _write_ppuaddr(&mut self, data :u8) {
        match self.latch {
            Some(hi) => {
                self.ppuaddr = (hi as u16) << 8 + data as u16;
            }
            None => {
                self.latch = Some(data);
            }
        }
    }

    pub fn _output_framebuffer_pixel(&mut self) {
        match self.scanline {
            SCANLINE_VISIBLE_START..=SCANLINE_VISIBLE_END => {
                if self.scanline_cycle >= PIXEL_OUTPUT_CYCLE_OFFSET &&
                    self.scanline_cycle < (FRAMEBUFFER_WIDTH as u16 - PIXEL_OUTPUT_CYCLE_OFFSET) {
                    let pixel = self.scanline as usize * FRAMEBUFFER_WIDTH + self.scanline_cycle as usize - 4;
                    self.framebuffer[pixel] = ((self.scanline_cycle as u64 + self.framecount) % 64) as u8;
                }
            }
            _ => {}
        }
    }

    fn _process_render_scanline(&mut self) {
        let cycle_mod = self.scanline_cycle % 8;

        match self.scanline_cycle {
            0 => {} // Idle
            1..=256 => {
                if cycle_mod == 1 && self.scanline_cycle >= 9 {
                    //Reload shift registers
                }

                match cycle_mod {
                    1 => {
                        // Nametable byte
                    }
                    3 => {
                        // Attribute
                    }
                    5 => {
                        // Pattern lo
                    }
                    7 => {
                        // Pattern hi
                    }
                    _ => {}
                }
            }
            257..=320 => {
                // Reload shift registers
                match cycle_mod {
                    1 => {
                        // Garbage Nametable byte
                    }
                    3 => {
                        // Garbage Nametable byte
                    }
                    5 => {
                        // Pattern lo
                    }
                    7 => {
                        // Pattern hi
                    }
                    _ => {}
                }
            }
            321..=336 => {
                // Read first two tiles next scanline
            }
            337..=340 => {
                // Read unknown
            }
            _ => { unreachable!() }
        }
    }

    pub fn tick(&mut self) -> bool {
        match self.scanline {
            // Pre-render scanline
            SCANLINE_PRE_RENDER => {
                self._process_render_scanline();
            }
            SCANLINE_VISIBLE_START..=SCANLINE_VISIBLE_END => {
                self._process_render_scanline();
            }
            SCANLINE_POST_RENDER => {}
            SCANLINE_VBLANK_START..=SCANLINE_VBLANK_END => {}
            _ => { unreachable!() }
        }

        self._output_framebuffer_pixel();
        self._increment_scanline_cycle();


        if self.scanline == SCANLINE_POST_RENDER && self.scanline_cycle == 0 {
            true
        } else {
            false
        }

    }

    fn _increment_scanline_cycle(&mut self) {
        self.scanline_cycle += 1;

        if self.scanline_cycle == SCANLINE_CYCLE_COUNT {
            if self.scanline == SCANLINE_PRE_RENDER && self.framecount % 2 == 1 {
                self.scanline_cycle = 1;
            } else {
                self.scanline_cycle = 0;
            }
            self.scanline += 1;
        }

        if self.scanline >= SCANLINE_PRE_RENDER {
            self.scanline = 0;
            self.framecount += 1;
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

        let cartridge = unsafe { &mut *self.cartridge_ptr };
        let data = cartridge.read_chr_slice(pattern_table_index as u16 * PATTERN_TABLE_SIZE as u16, PATTERN_TABLE_SIZE);

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

                    target[target_index] = val;
                }
            }
        }

        target
    }

    fn _read_memory(&self, address: u16) -> u8 {
        match address {
            0..=0x1FFF => {
                let cartridge = unsafe { &mut *self.cartridge_ptr };
                cartridge.read_chr(address)
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

    pub fn get_ppuaddr(&self) -> u16 {
        self.ppuaddr
    }

    pub fn get_framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }
}


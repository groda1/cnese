use crate::nes::cartridge::cartridge::Cartridge;
use super::register;
use crate::ppu::register::{PpuStatus, PpuCtrl, PpuStatusTrait, PpuCtrlTrait};
use crate::ppu::nametable;
use crate::ppu::nametable::NametableMemory;

const PATTERN_TABLE_SIZE: usize = 0x1000;

const SCANLINE_PRE_RENDER: u16 = 261;
const SCANLINE_VISIBLE_START: u16 = 0;
const SCANLINE_VISIBLE_END: u16 = 239;
const SCANLINE_POST_RENDER: u16 = 240;
const SCANLINE_VBLANK_START: u16 = 241;
const SCANLINE_VBLANK_END: u16 = 260;

const SCANLINE_CYCLE_COUNT: u16 = 341;

const PIXEL_OUTPUT_CYCLE_OFFSET: u16 = 1;

pub const FRAMEBUFFER_WIDTH: usize = 256;
pub const FRAMEBUFFER_HEIGHT: usize = 240;
const FRAMEBUFFER_SIZE: usize = FRAMEBUFFER_WIDTH * FRAMEBUFFER_HEIGHT;

const PALETTE_RAM_SIZE : usize = 0x20;
const PALETTE_START_ADDRESS: u16 = 0x3F00;
const PALETTE_END_ADDRESS: u16 = 0x3FFF;

const PATTERN_TABLE_START : u16 = 0;
const PATTERN_TABLE_END :u16 = 0x1FFF;

const VRAM_SIZE:u16 = 0x4000;

pub struct Ppu {
    cartridge_ptr: *mut Cartridge,
    ppuctrl: PpuCtrl,
    ppumask: u8,
    ppustatus: PpuStatus,

    oamaddr: u8,
    // TODO oamdata

    ppuscroll: u16,
    ppuaddr: u16,
    latch: Option<u8>,

    nametable_memory: NametableMemory,
    palette_ram: [u8; PALETTE_RAM_SIZE],

    vram_read_buffer :u8,

    // rendering variables
    scanline: u16,
    scanline_cycle: u16,

    framecount: u64,

    framebuffer: [u8; FRAMEBUFFER_SIZE],

    v_horizontal: u8,
    v_vertical:u8,

    _tmp_nt_byte :u8,
    _tmp_at_byte:u8,
    _tmp_pt_lo:u8,
    _tmp_pt_hi: u8,

    bg_pattern_lo_shift: u16,
    bg_pattern_hi_shift: u16,


}

impl Ppu {
    pub fn new(cartridge_ptr: *mut Cartridge) -> Ppu {
        let cartridge = unsafe { &*cartridge_ptr };

        Ppu {
            cartridge_ptr,
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            latch: None,
            nametable_memory: NametableMemory::new(cartridge.get_mirroring()),
            palette_ram: [0; PALETTE_RAM_SIZE],
            vram_read_buffer: 0,
            scanline: SCANLINE_PRE_RENDER,
            scanline_cycle: 0,
            framecount: 0,
            framebuffer: [0; FRAMEBUFFER_SIZE],

            v_horizontal : 2,
            v_vertical : 30,

            _tmp_nt_byte : 0,
            _tmp_at_byte : 0,
            _tmp_pt_lo : 0,
            _tmp_pt_hi : 0,

            bg_pattern_lo_shift: 0,
            bg_pattern_hi_shift: 0,
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
            register::OAMDATA_OFFSET => {
                self._write_oamdata(data);
            }
            register::PPUSCROLL_OFFSET => {
                self._write_ppuscroll(data);
            }
            register::PPUADDR_OFFSET => {
                self._write_ppuaddr(data);
            }
            register::PPUDATA_OFFSET => {
                self._write_ppudata(data);
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
                self.ppustatus.clear_vblank();
                self.latch = None;

                status
            }
            register::PPUDATA_OFFSET => {
                self._read_ppudata()
            }
            _ => {
                println!("CRASH ppu::read_register ${:04X}", address);
                unreachable!()
            }
        }
    }

    fn _write_oamdata(&mut self, data: u8)  {
        // TODO write oamdata;
        // println!("writing oamdata");
    }


    fn _write_ppuaddr(&mut self, data: u8) {
        match self.latch {
            Some(hi) => {
                self.ppuaddr = (((hi as u16) << 8) + data as u16) % VRAM_SIZE;
                self.latch = None;
            }
            None => {
                self.latch = Some(data);
            }
        }
    }

    fn _write_ppuscroll(&mut self, data: u8) {
        match self.latch {
            Some(hi) => {
                self.ppuscroll = ((hi as u16) << 8) + data as u16;
            }
            None => {
                self.latch = Some(data);
            }
        }
    }

    fn _read_ppudata(&mut self) -> u8 {
        let mut return_value = self.vram_read_buffer;

        match self.ppuaddr {
            PATTERN_TABLE_START..=PATTERN_TABLE_END =>  unsafe {
                self.vram_read_buffer =  (&mut *self.cartridge_ptr).read_chr(self.ppuaddr);
            }
            nametable::START_ADDRESS..=nametable::END_ADDRESS => {
                self.vram_read_buffer = self.nametable_memory.read(self.ppuaddr);
            }
            nametable::MIRROR_START_ADDRESS..=nametable::MIRROR_END_ADDRESS => {
                let mirrored_address = self.ppuaddr - 0x1000;
                self.vram_read_buffer = self.nametable_memory.read(mirrored_address);
            }
            PALETTE_START_ADDRESS..=PALETTE_END_ADDRESS => {
                let mirrored_address = self.ppuaddr - 0x1000;

                return_value = self.palette_ram[(mirrored_address as usize % PALETTE_RAM_SIZE)];
                self.vram_read_buffer = self.nametable_memory.read(mirrored_address);
            }
            _ => {
                println!("_read_ppudata {:04x}", self.ppuaddr);
                unreachable!()
            }
        }

        self.ppuaddr += self.ppuctrl.vram_address_increment();
        if self.ppuaddr > VRAM_SIZE {
            self.ppuaddr = self.ppuaddr % VRAM_SIZE;
        }

        return_value
    }

    fn _write_ppudata(&mut self, data: u8) {
        match self.ppuaddr {
            PATTERN_TABLE_START..=PATTERN_TABLE_END => unsafe {
                (&mut *self.cartridge_ptr).write_chr(self.ppuaddr, data);
            }
            nametable::START_ADDRESS..=nametable::END_ADDRESS => {
               self.nametable_memory.write(self.ppuaddr, data);
            }
            nametable::MIRROR_START_ADDRESS..=nametable::MIRROR_END_ADDRESS => {
                let mirrored_address = self.ppuaddr - 0x1000;
                self.nametable_memory.write(mirrored_address, data);
            }
            PALETTE_START_ADDRESS..=PALETTE_END_ADDRESS => {
                self.palette_ram[self.ppuaddr as usize % PALETTE_RAM_SIZE];
            }
            _ => {
                println!("_write_ppudata {:04x}", self.ppuaddr);
                unreachable!()
            }
        }

        self.ppuaddr += self.ppuctrl.vram_address_increment();
        if self.ppuaddr > VRAM_SIZE {
            self.ppuaddr = self.ppuaddr % VRAM_SIZE;
        }
    }

    pub fn _output_framebuffer_pixel(&mut self) {
        match self.scanline {
            SCANLINE_VISIBLE_START..=SCANLINE_VISIBLE_END => {
                if self.scanline_cycle >= PIXEL_OUTPUT_CYCLE_OFFSET &&
                    self.scanline_cycle < (FRAMEBUFFER_WIDTH as u16 - PIXEL_OUTPUT_CYCLE_OFFSET) {
                    let pixel = self.scanline as usize * FRAMEBUFFER_WIDTH + self.scanline_cycle as usize - PIXEL_OUTPUT_CYCLE_OFFSET as usize;
                    self.framebuffer[pixel] = self._next_pixel_value();
                }
            }
            _ => {}
        }
    }

    pub fn _next_pixel_value(&mut self) -> u8 {
        let lo = (self.bg_pattern_lo_shift >> 15) as u8;
        let hi = (self.bg_pattern_hi_shift >> 15) as u8;

        self.bg_pattern_lo_shift <<= 1;
        self.bg_pattern_hi_shift <<= 1;


        (hi << 1)+ lo
    }

    fn _prerender_scanline(&mut self) {
        self._process_scanline();

        match self.scanline_cycle {
            1 => {
                self.ppustatus.clear_vblank();
                // TODO clear sprite overflow
            }
            280..=304 => {
                self.v_vertical = 0;
            }

            _ => {}
        }

    }

    fn _inc_v_horizontal(&mut self) {
        self.v_horizontal += 1;
        if self.v_horizontal > 32 {
            self.v_horizontal = 0;
        }
    }

    fn _inc_v_vertical(&mut self) {
        self.v_vertical += 1;
        if self.v_vertical >= 240 {
            self.v_vertical = 0;
        }
    }

    fn _reset_v_horizontal(&mut self) {
        self.v_horizontal = 0;
    }

    fn _fetch_nt_byte(&mut self) {
        let fetch_addr = (self.v_horizontal as u16 + ((self.v_vertical / 8)as u16 * 32)) | 0x2000;

        // println!("v.x={} v.y={} scanline={} cycle={}, fetch_addr={:04x}", self.v_horizontal , self.v_vertical, self.scanline, self.scanline_cycle, fetch_addr);
        self._tmp_nt_byte = self.nametable_memory.read(fetch_addr);

        if self._tmp_nt_byte > 0 {
            // println!("nt byte {}   scanline={} cycle={}", self._tmp_nt_byte, self.scanline, self.scanline_cycle);
        }
    }

     fn _fetch_bg_lo_byte(&mut self) {
        let addr = (self._tmp_nt_byte as u16 * 16) + (self.v_vertical % 8) as u16;
         self._tmp_pt_lo = unsafe { (*self.cartridge_ptr).read_chr(addr + 0x1000) };
         // TODO patterntable offset
    }


    fn _fetch_bg_hi_byte(&mut self) {
        let addr = (self._tmp_nt_byte as u16 * 16 + (self.v_vertical % 8) as u16) + 8;

        self._tmp_pt_hi = unsafe { (*self.cartridge_ptr).read_chr(addr + 0x1000) };
        // TODO patterntable offset
    }

    fn _reload_shift_registers(&mut self) {
        self.bg_pattern_lo_shift = (self.bg_pattern_lo_shift & 0xFF00) | self._tmp_pt_lo as u16;
        self.bg_pattern_hi_shift = (self.bg_pattern_hi_shift & 0xFF00) | self._tmp_pt_hi as u16;
    }

    fn _shift_byte(&mut self) {
        self.bg_pattern_lo_shift <<= 8;
        self.bg_pattern_hi_shift <<= 8;
    }

    fn _process_scanline(&mut self) {
        let cycle_mod = self.scanline_cycle % 8;

        match self.scanline_cycle {
            0 => {} // Idle
            1..=256 => {
                if cycle_mod == 1 && self.scanline_cycle >= 9 {
                    //Reload shift registers
                    self._reload_shift_registers();
                }

                match cycle_mod {
                    0 => { self._inc_v_horizontal(); }
                    1 => { self._fetch_nt_byte();


                    }
                    3 => {
                        // Attribute
                    }
                    5 => {
                        // Pattern lo
                        self._fetch_bg_lo_byte();
                    }
                    7 => {
                        // Pattern hi
                        self._fetch_bg_hi_byte();
                    }
                    _ => {}
                }
                if self.scanline_cycle == 256 {
                    self._inc_v_vertical();
                }

            }
            257 => {
                self._reset_v_horizontal();
            }

            321 => { self._fetch_nt_byte(); }
            325 => { self._fetch_bg_lo_byte(); }
            327 => { self._fetch_bg_hi_byte(); }
            328 => { self._inc_v_horizontal(); }
            329 => {
                self._fetch_nt_byte();
                self._reload_shift_registers();
                self._shift_byte();
            }
            333 => { self._fetch_bg_lo_byte(); }
            335 => { self._fetch_bg_hi_byte(); }
            336 => { self._inc_v_horizontal(); }

            337 => { self._reload_shift_registers();}
            338..=340 => {
                // Read unknown
            }
            _ => { }
        }
    }

    fn _vblank_scanline(&mut self) {
        if self.scanline_cycle == 1 {
            self.ppustatus.set_vblank();
        }
    }

    pub fn tick(&mut self) -> bool {
        match self.scanline {
            // Pre-render scanline
            SCANLINE_PRE_RENDER => {
                self._prerender_scanline();
            }
            SCANLINE_VISIBLE_START..=SCANLINE_VISIBLE_END => {
                self._process_scanline();
            }
            SCANLINE_POST_RENDER => {}
            SCANLINE_VBLANK_START..=SCANLINE_VBLANK_END => {
                self._vblank_scanline();
            }
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

        if self.scanline > SCANLINE_PRE_RENDER {
            self.scanline = 0;
            self.framecount += 1;

            self.v_vertical = 0; // TODO
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

    fn _write_memory(&self, address: u16) -> u8 {
        match address {
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

    pub fn get_nmi_signal(&self) -> bool {
        self.ppuctrl.generate_nmi() && self.ppustatus.is_vblank()
    }

    pub fn get_ppuaddr(&self) -> u16 {
        self.ppuaddr
    }

    pub fn get_framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }

}


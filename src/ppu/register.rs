
pub const REGISTER_SIZE: u16 = 8;

// PPUCTRL (write)
pub const PPUCTRL_OFFSET: u16 = 0x0;

// 7  bit  0
// ---- ----
// VPHB SINN
// |||| ||||
// |||| ||++- Base nametable address
// |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
// |||| |+--- VRAM address increment per CPU read/write of PPUDATA
// |||| |     (0: add 1, going across; 1: add 32, going down)
// |||| +---- Sprite pattern table address for 8x8 sprites
// ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
// |||+------ Background pattern table address (0: $0000; 1: $1000)
// ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
// |+-------- PPU master/slave select
// |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
// +--------- Generate an NMI at the start of the
// vertical blanking interval (0: off; 1: on)
const PPUCTRL_BASE_NAMETABLE_MASK: u8 = 0b11;
const PPUCTRL_VRAM_ADDRESS_INCREMENT_MASK: u8 = 1 << 2;
const PPUCTRL_SPRITE_PATTERN_TABLE_ADDR_MASK: u8 = 1 << 3;
const PPUCTRL_BG_PATTERN_TABBLE_ADDR_MASK: u8 = 1 << 4;
const PPUCTRL_SPRITE_SIZE_MASK: u8 = 1 << 5;
const PPUCTRL_PPU_MASTER_SLAVE_SELECT_MASK: u8 = 1 << 6;
const PPUCTRL_GENERATE_NMI_MASK: u8 = 1 << 7;

trait PpuCtrl {
    fn base_nametable_addr(&self) -> u16;
    fn vram_address_increment(&self) -> u8;
    fn sprite_pattern_table_addr(&self) -> u8;
    fn bg_pattern_table_addr(&self) -> u8;
    fn sprite_size(&self) -> u8;
    fn ppu_master_slave_select(&self) -> u8;
    fn generate_nmi(&self) -> u8;
}

impl PpuCtrl for u8 {
    fn base_nametable_addr(&self) -> u16 {
        unimplemented!()
    }

    fn vram_address_increment(&self) -> u8 {
        unimplemented!()
    }

    fn sprite_pattern_table_addr(&self) -> u8 {
        unimplemented!()
    }

    fn bg_pattern_table_addr(&self) -> u8 {
        unimplemented!()
    }

    fn sprite_size(&self) -> u8 {
        unimplemented!()
    }

    fn ppu_master_slave_select(&self) -> u8 {
        unimplemented!()
    }

    fn generate_nmi(&self) -> u8 {
        unimplemented!()
    }
}

// PPUMASK (write)
pub const PPUMASK_OFFSET:u16 = 0x1;
// 7  bit  0
// ---- ----
// BGRs bMmG
// |||| ||||
// |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
// |||| +---- 1: Show background
// |||+------ 1: Show sprites
// ||+------- Emphasize red
// |+-------- Emphasize green
// +--------- Emphasize blue
const PPUMASK_GREYSCALE_MASK: usize = 1 << 0;
const PPUMASK_SHOW_BG_LEFTMOST_MASK: usize = 1 << 1;
const PPUMASK_SHOW_SPRITES_LEFTMOST_MASK: usize = 1 << 2;
const PPUMASK_SHOW_BG_MASK: usize = 1 << 3;
const PPUMASK_SHOW_SPRITES_MASK: usize = 1 << 4;
const PPUMASK_EMPHASIZE_RED_MASK: usize = 1 << 5;
const PPUMASK_EMPHASIZE_GREEN_MASK: usize = 1 << 6;
const PPUMASK_EMPHASIZE_BLUE_MASK: usize = 1 << 7;


// PPUSTATUS (read)
pub const PPUSTATUS_OFFSET: u16 = 0x2;
// 7  bit  0
// ---- ----
// VSO. ....
// |||| ||||
// |||+-++++- Least significant bits previously written into a PPU register
// |||        (due to register not being updated for this address)
// ||+------- Sprite overflow. The intent was for this flag to be set
// ||         whenever more than eight sprites appear on a scanline, but a
// ||         hardware bug causes the actual behavior to be more complicated
// ||         and generate false positives as well as false negatives; see
// ||         PPU sprite evaluation. This flag is set during sprite
// ||         evaluation and cleared at dot 1 (the second dot) of the
// ||         pre-render line.
// |+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
// |          a nonzero background pixel; cleared at dot 1 of the pre-render
// |          line.  Used for raster timing.
// +--------- Vertical blank has started (0: not in vblank; 1: in vblank).
// Set at dot 1 of line 241 (the line *after* the post-render
// line); cleared after reading $2002 and at dot 1 of the
// pre-render line.

const PPUSTATUS_SPRITE_OVERFLOW_MASK: u8 = 1 << 5;
const PPUSTATUS_SPRITE_0_MASK: u8 = 1 << 6;
const PPUSTATUS_VBLANK_MASK: u8 = 1 << 7;


// OAMADDR (write)
pub const OAMADDR_OFFSET: u16 = 3;
// OAMDATA (read/write)
pub const OAMDATA_OFFSET :u16 = 4;

// PPUSCROLL ( write x2)
pub const PPUSCROLL_OFFSET : u16 = 5;

// PPUADDR (write x2)
pub const PPUADDR_OFFSET : u16 = 6;

// PPUDATA  (read/write)
pub const PPUDATA_OFFSET : u16 = 7;
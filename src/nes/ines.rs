use crate::util::file;

use super::cartridge::cartridge;
use crate::nes::cartridge::cartridge::Cartridge;
/*
An iNES file consists of the following sections, in order:

Header (16 bytes)
Trainer, if present (0 or 512 bytes)
PRG ROM data (16384 * x bytes)
CHR ROM data, if present (8192 * y bytes)
PlayChoice INST-ROM, if present (0 or 8192 bytes)
PlayChoice PROM, if present (16 bytes Data, 16 bytes CounterOut) (this is often missing, see PC10 ROM-Images for details)
Some ROM-Images additionally contain a 128-byte (or sometimes 127-byte) title at the end of the file.

The format of the header is as follows:

0-3: Constant $4E $45 $53 $1A ("NES" followed by MS-DOS end-of-file)
4: Size of PRG ROM in 16 KB units
5: Size of CHR ROM in 8 KB units (Value 0 means the board uses CHR RAM)
6: Flags 6 - Mapper, mirroring, battery, trainer
7: Flags 7 - Mapper, VS/Playchoice, NES 2.0
8: Flags 8 - PRG-RAM size (rarely used extension)
9: Flags 9 - TV system (rarely used extension)
10: Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension)
11-15: Unused padding (should be filled with zero, but some rippers put their name across bytes 7-15)
*/

const INES_PREFIX: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];

const PRG_ROM_CHUNK_COUNT_OFFSET: usize = 4;
pub const PRG_ROM_CHUNK_SIZE: usize = 0x4000;

const CHR_ROM_SIZE_OFFSET: usize = 5;
const CHR_ROM_CHUNK_SIZE: usize = 0x2000;

const HEADER_SIZE: usize = 0x10;
const TRAINER_SIZE: usize = 0x200;

const FLAGS_6_OFFSET: usize = 6;
const FLAGS_6_MIRRORING_MASK: u8 = 1;
const FLAGS_6_BATTERY_MASK: u8 = 2;
const FLAGS_6_TRAINER_MASK: u8 = 4;
const FLAGS_6_IGNORE_MIRRORING__MASK: u8 = 8;

const FLAGS_7_OFFSET: usize = 7;
const FLAGS_8_OFFSET: usize = 8;
const FLAGS_9_OFFSET: usize = 9;

pub fn open_ines(path: &String) -> Result<(Cartridge), &str> {
    let file_data = file::read_file(path);


    let header = &file_data[0..HEADER_SIZE];

    let ines_prefix = &header[0..4];
    if !ines_prefix.eq(&INES_PREFIX) {
        return Err("Not a valid iNES file");
    }

    let prg_size = header[PRG_ROM_CHUNK_COUNT_OFFSET];
    let chr_size = header[CHR_ROM_SIZE_OFFSET];
    let mapper = (header[FLAGS_7_OFFSET] & 0xf0) + (header[FLAGS_6_OFFSET] >> 4);

    let battery_ram = (header[FLAGS_6_OFFSET] & FLAGS_6_BATTERY_MASK) > 0;
    let trainer_present = (header[FLAGS_6_OFFSET] & FLAGS_6_TRAINER_MASK) > 0;

    let mut offset = if trainer_present { TRAINER_SIZE + HEADER_SIZE } else { HEADER_SIZE };
    let mut prg_rom_vec = Vec::new();

    for i in 0..prg_size {
        prg_rom_vec.push(&file_data[offset..offset + PRG_ROM_CHUNK_SIZE]);
        offset += PRG_ROM_CHUNK_SIZE;
    }

    println!("ines prg={} chr={} mapper={} battery={} trainer={}", prg_size, chr_size, mapper, battery_ram, trainer_present);

    let cartridge = cartridge::create_cartridge_from_ines(mapper, prg_rom_vec)?;

    Ok(cartridge)
}

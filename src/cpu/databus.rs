const DATABUS_SIZE: usize = std::u16::MAX as usize + 1;

pub const CARTRIDGE_SPACE_OFFSET: usize = 0x4020;

/*
Address range       Size        Device
$0000-$07FF         $0800       2KB internal RAM
$0800-$0FFF         $0800       Mirrors of $0000-$07FF
$1000-$17FF         $0800       Mirrors of $0000-$07FF
$1800-$1FFF         $0800       Mirrors of $0000-$07FF
$2000-$2007         $0008       NES PPU registers
$2008-$3FFF         $1FF8       Mirrors of $2000-2007 (repeats every 8 bytes)
$4000-$4017         $0018       NES APU and I/O registers
$4018-$401F         $0008       APU and I/O functionality that is normally disabled. See CPU Test Mode.
$4020-$FFFF         $BFE0       Cartridge space: PRG ROM, PRG RAM, and mapper registers (See Note)
*/


/*
$FFFA, $FFFB ... NMI (Non-Maskable Interrupt) vector
$FFFC, $FFFD ... RES (Reset) vector
$FFFE, $FFFF ... IRQ (Interrupt Request) vector
*/



pub struct Databus {
    data: Vec<u8>
}

impl Databus {
    pub fn new() -> Databus {
        Databus {
            data: vec![0; DATABUS_SIZE]
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let lo = self.read(address);
        let hi = self.read(address + 1);

        ((hi as u16) << 8) + lo as u16
    }

    pub fn read_slice(&self, address: u16, len: usize) -> &[u8] {
        let index = address as usize;

        &(self.data)[index..index + len]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.data[address as usize] = data;
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.data[CARTRIDGE_SPACE_OFFSET..CARTRIDGE_SPACE_OFFSET + rom_data.len()]
            .copy_from_slice(rom_data);
    }

    pub fn get_cartridge(&self) -> &[u8] {
        &(self.data)[CARTRIDGE_SPACE_OFFSET..0xFFFF]
    }
}

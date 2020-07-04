use super::super::nes::cartridge::cartridge::Cartridge;


const DATABUS_SIZE: usize = std::u16::MAX as usize + 1;


pub const RAM_SIZE: usize = 0x0800;

pub const CARTRIDGE_SPACE_OFFSET: u16 = 0x4020;
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


//TODO fix the stupid magic number ranges
pub struct Databus {
    ram: Box<[u8; RAM_SIZE]>,
    cartridge: Option<Cartridge>,
}

impl Databus {
    pub fn new() -> Databus {
        let ram = [0 as u8; RAM_SIZE];

        Databus {
            ram: Box::new(ram),
            cartridge: None,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0..=0x1fff => {
                self.ram[address as usize % RAM_SIZE]
            }
            0x4020..=0xFFFF => {
                self.cartridge.as_ref().unwrap().read(address)
            }
            _ => unreachable!()
        }
    }

    pub fn read_slice(&self, address: u16, len: usize) -> &[u8] {
        match address {
            0..=0x1fff => {
                let index = address as usize;
                &self.ram[index..index + len]
            }
            0x4020..=0xFFFF => {
                self.cartridge.as_ref().unwrap().read_slice(address, len)
            }
            _ => unreachable!()
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let lo = self.read(address);
        let hi = self.read(address + 1);

        ((hi as u16) << 8) + lo as u16
    }


    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0..=0x1fff => {
                self.ram[address as usize % RAM_SIZE] = data;
            }
            0x4020..=0xFFFF => {
                self.cartridge.as_mut().unwrap().write(address, data)
            }

            _ => unreachable!()
        }
    }


    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge);
    }

    // TODO this needs to be removed
    pub fn get_cartridge(&self) -> &[u8] {
        self.read_slice(CARTRIDGE_SPACE_OFFSET, 0xFFFF - CARTRIDGE_SPACE_OFFSET as usize)
    }
}

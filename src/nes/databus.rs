use std::cell::RefCell;
use std::rc::Rc;

use super::super::nes::cartridge::cartridge::Cartridge;
use crate::ppu::ppu::Ppu;

pub const CARTRIDGE_SPACE_START: u16 = 0x4020;

const INTERNAL_RAM_START: u16 = 0x0000;
const INTERNAL_RAM_END: u16 = 0x1FFF;

const NES_PPU_REGISTER_START:u16 = 0x2000;
const NES_PPU_REGISTER_END:u16 = 0x2007;

const NES_APU_IO_REGISTERS_START:u16 = 0x4000;
const NES_APU_IO_REGISTERS_END:u16 = 0x4017;

const RAM_SIZE: usize = 0x0800;

pub const END: u16 = 0xFFFF;

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


pub struct NesDatabus {
    ram: Box<[u8; RAM_SIZE]>,
    cartridge: *mut Cartridge,
    ppu: *mut Ppu

}

impl NesDatabus {
    pub fn new(cartridge: *mut Cartridge, ppu: *mut Ppu) -> NesDatabus {
        let ram = [0 as u8; RAM_SIZE];

        NesDatabus {
            ram: Box::new(ram),
            cartridge,
            ppu,
        }
    }

    // TODO move to apu/io controller
    fn _write_apu_io(&mut self, address: u16, data: u8) {

        if address == 0x4014 {
            println!("OAMDMA")
        } else if address == 0x4016 {
            println!("CONTROLLER POLL {}", data)
        }
    }

    // TODO move to apu/io controller
    fn _read_apu_io(&self, address: u16) -> u8 {

        if address == 0x4016 {
            0xff
        }
         else {
             unreachable!()
         }
    }

}

impl crate::cpu::databus::Databus for NesDatabus {
    fn read(&self, address: u16) -> u8 {
        match address {
            INTERNAL_RAM_START..=INTERNAL_RAM_END => {
                self.ram[address as usize % RAM_SIZE]
            }
            NES_PPU_REGISTER_START..=NES_PPU_REGISTER_END => unsafe {
                (*self.ppu).read_register(address)
            }
            NES_APU_IO_REGISTERS_START..=NES_APU_IO_REGISTERS_END => {
                self._read_apu_io(address)
            }
            CARTRIDGE_SPACE_START..=END => unsafe {
                (*self.cartridge).read_prg(address)
            }
            _ => {
                println!("CRASH Databus::read {:02x}", address);
                unreachable!()
            }
        }
    }

    fn read_u16(&self, address: u16) -> u16 {
        let lo = self.read(address);
        let hi = self.read(address + 1);

        ((hi as u16) << 8) + lo as u16
    }


    fn write(&mut self, address: u16, data: u8) {
        match address {
            INTERNAL_RAM_START..=INTERNAL_RAM_END => {
                self.ram[address as usize % RAM_SIZE] = data;
            }
            CARTRIDGE_SPACE_START..=END => unsafe {
                (*self.cartridge).write_prg(address, data);
            }
            NES_PPU_REGISTER_START..=NES_PPU_REGISTER_END => unsafe {
                (*self.ppu).write_register(address, data);
            }
            NES_APU_IO_REGISTERS_START..=NES_APU_IO_REGISTERS_END => {
                self._write_apu_io(address, data);
            }

            _ => {
                println!("CRASH: write ${:04X} = ${:02X}", address, data);
                unreachable!()
            }
        }
    }


}

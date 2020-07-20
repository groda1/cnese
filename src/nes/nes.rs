use std::cell::{RefCell, Ref};
use std::rc::Rc;

use crate::nes::databus::{NesDatabus, END};
use crate::cpu::cpu::Cpu;
use crate::nes::cartridge::cartridge::Cartridge;
use crate::ppu::ppu::Ppu;
use crate::cpu::databus::Databus;
use crate::cpu::instruction;
use crate::cpu::instruction::Instruction;

pub struct NES {
    cpu: Cpu,
    ppu: Box<Ppu>,
    databus: NesDatabus,
    cartridge: Box<Cartridge>,

    _actual_framerate: u32
}

impl NES {
    pub fn new(cartridge : Cartridge) -> NES {
        let mut cartridge = Box::new(cartridge);
        let cartridge_ptr: *mut Cartridge = &mut *cartridge;

        let mut ppu = Box::new(Ppu::new(cartridge_ptr));
        let ppu_ptr: *mut Ppu = &mut *ppu;

        NES {
            cpu: Cpu::new(),
            ppu,
            databus: NesDatabus::new(cartridge_ptr, ppu_ptr),
            cartridge,
            _actual_framerate: 0
        }
    }

    pub fn tick(&mut self) -> bool {
        self.cpu.tick(&mut self.databus);

        let mut frame_done = false;
        frame_done |= self.ppu.tick();
        frame_done |= self.ppu.tick();
        frame_done |= self.ppu.tick();

        if self.ppu.get_nmi_signal() {
            self.cpu.set_nmi_lo();
        } else {
            self.cpu.set_nmi_hi();
        }

        frame_done
    }

    pub fn tick_cpu_instruction(&mut self) {
        // TODO
        // let cycles = self.cpu.tick_instruction(&mut self.databus);
        // for _i in 0..cycles * 3 {
        //     self.ppu.tick();
        // }
    }

    pub fn get_databus(&self) -> &dyn Databus { &self.databus }

    pub fn get_ppu(&self) -> &Ppu {
        &self.ppu
    }
    pub fn get_cpu(&self) -> &Cpu { &self.cpu }
    pub fn reset(&mut self) { self.cpu.reset(&self.databus); }
    pub fn set_irq_lo(&mut self) { self.cpu.set_irq_lo(); }
    pub fn set_irq_hi(&mut self) { self.cpu.set_irq_hi(); }
    pub fn set_nmi_hi(&mut self) { self.cpu.set_nmi_hi(); }
    pub fn set_nmi_lo(&mut self) { self.cpu.set_nmi_lo(); }
    pub fn get_actual_framerate(&self) -> u32 { self._actual_framerate }

    pub fn set_actual_framerate(&mut self, frames_dropped: u32) {
        self._actual_framerate = frames_dropped
    }

    pub fn deassemble_prg(&self) -> (Vec<Instruction>, u16) {
        let start_address = self.cartridge.get_instruction_offset();

        let mut instructions: Vec<Instruction> = Vec::new();
        let mut i = start_address;

        while i < (END - 3) {
            let instruction = instruction::decode_instruction(&self.databus, i);
            i += instruction.get_size() as u16;
            instructions.push(instruction);
        }

        (instructions, start_address)
    }
}


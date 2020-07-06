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
    ppu: Rc<RefCell<Ppu>>,
    databus: NesDatabus,
    cartridge: Rc<RefCell<Cartridge>>,

    _actual_framerate: u32
}

impl NES {
    pub fn new(cartridge : Cartridge) -> NES {

        let cartridge_rc = Rc::new(RefCell::new(cartridge));
        let ppu_pc = Rc::new(RefCell::new(Ppu::new(cartridge_rc.clone())));

        NES {
            cpu: Cpu::new(),
            ppu: ppu_pc.clone(),
            databus: NesDatabus::new(cartridge_rc.clone(), ppu_pc.clone()),
            cartridge: cartridge_rc.clone(),
            _actual_framerate: 0
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.databus);
    }

    pub fn tick_cpu_instruction(&mut self) {
        self.cpu.tick_instruction(&mut self.databus);
    }

    pub fn get_databus(&self) -> &dyn Databus { &self.databus }

    pub fn borrow_ppu(&self) -> Ref<Ppu> {
        RefCell::borrow(&self.ppu)
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

        let start_address = self.cartridge.borrow().get_instruction_offset();

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


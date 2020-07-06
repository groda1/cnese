use crate::cpu::cpu::Cpu;
use crate::cpu::databus::Databus;
use crate::nes::cartridge::cartridge::Cartridge;
use crate::ppu::ppu::Ppu;
use std::cell::{RefCell, Ref};
use std::rc::Rc;
use std::borrow::{Borrow, BorrowMut};
use sdl2::render::UpdateTextureYUVError::RectNotInsideTexture;

pub struct NES {
    cpu: Cpu,
    ppu: Ppu,
    databus: Databus,
    cartridge: Rc<RefCell<Cartridge>>,

    _actual_framerate: u32
}

impl NES {
    pub fn new(cartridge : Cartridge) -> NES {

        let cart_ridge_rc = Rc::new(RefCell::new(cartridge));

        NES {
            cpu: Cpu::new(),
            ppu: Ppu::new(cart_ridge_rc.clone()),
            databus: Databus::new(cart_ridge_rc.clone()),
            cartridge: cart_ridge_rc.clone(),
            _actual_framerate: 0
        }

    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.databus);
    }

    pub fn tick_cpu_instruction(&mut self) {
        self.cpu.tick_instruction(&mut self.databus);
    }

    pub fn get_databus(&self) -> &Databus {
        &self.databus
    }

    pub fn get_ppu(&self) -> &Ppu {
        &self.ppu
    }

    pub fn borrow_cartridge(&self) -> Ref<Cartridge> {
       RefCell::borrow(&self.cartridge)
    }

    pub fn get_cpu(&self) -> &Cpu {
        &self.cpu
    }
    pub fn reset(&mut self) {
        self.cpu.reset(&self.databus);
    }

    // pub fn load_cartridge(&mut self, cartridge: Cartridge) {
    //
    //     self.databus.load_cartridge(cartridge);
    //     self.reset();
    // }

    pub fn kek(&mut self) {

        let fo = self.cartridge.borrow_mut();

    }
    pub fn set_irq_lo(&mut self) { self.cpu.set_irq_lo(); }
    pub fn set_irq_hi(&mut self) { self.cpu.set_irq_hi(); }
    pub fn set_nmi_hi(&mut self) { self.cpu.set_nmi_hi(); }
    pub fn set_nmi_lo(&mut self) { self.cpu.set_nmi_lo(); }

    pub fn get_actual_framerate(&self) -> u32 { self._actual_framerate }

    pub fn set_actual_framerate(&mut self, frames_dropped: u32) {
        self._actual_framerate = frames_dropped
    }
}
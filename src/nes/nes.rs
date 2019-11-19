use crate::cpu::cpu::Cpu;
use crate::cpu::databus::Databus;

pub struct NES {
    cpu: Cpu,
    databus: Databus,


    _actual_framerate: u32
}


impl NES {
    pub fn new() -> NES {
        NES {
            cpu: Cpu::new(),
            databus: Databus::new(),
            _actual_framerate: 0
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick(&mut self.databus);
    }

    pub fn get_databus(&self) -> &Databus {
        &self.databus
    }

    pub fn get_cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn reset(&mut self) {
        self.cpu.reset(&self.databus);
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.databus.load_rom(rom_data);
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
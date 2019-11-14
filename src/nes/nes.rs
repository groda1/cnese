use crate::cpu::cpu::Cpu;
use crate::cpu::databus::Databus;

pub struct NES {
    cpu: Cpu,
    databus: Databus,
}

impl NES {
    pub fn new() -> NES {
        NES {
            cpu: Cpu::new(),
            databus: Databus::new(),
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

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.databus.load_rom(rom_data);
    }
}
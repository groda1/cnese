use crate::cpu::cpu::Cpu;
use crate::cpu::databus::Databus;

pub struct NES {
    cpu: Cpu,
    databus: Databus,


    _framerate: u32
}


impl NES {
    pub fn new() -> NES {
        NES {
            cpu: Cpu::new(),
            databus: Databus::new(),
            _framerate: 0
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

    pub fn get_frames_dropped(&self) -> u32 { self._framerate }

    pub fn set_framerate(&mut self, frames_dropped: u32) {
        self._framerate = frames_dropped
    }
}
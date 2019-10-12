const DATABUS_SIZE: usize = std::u16::MAX as usize + 1;

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

    pub fn read_slize(&self, address: u16, len: usize) -> &[u8] {
        let index = address as usize;

        &(self.data)[index..index + len]

    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.data[address as usize] = data;
    }

    pub fn load_rom(&mut self, rom_data: Vec<u8>) {
        let mut i: usize = 0;
        for byte in rom_data {
            self.data[i] = byte;
            i += 1;
        }
    }
}

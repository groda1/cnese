

const START_ADDRESS:usize = 0x2000;

const NAMETABLE_SIZE:usize = 0x400;

pub struct Nametables {
    memory : [u8; NAMETABLE_SIZE * 2]
}

impl Nametables {
    pub fn new() -> Nametables {
        Nametables {
            memory: [0; NAMETABLE_SIZE * 2]

        }
    }
}

pub enum Mirroring {
    Horizontal = 0,
    Vertical = 1
}
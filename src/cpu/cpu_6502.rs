use super::state::State;
use super::databus::Databus;
use super::instruction;

pub struct Cpu {
    state: State
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            state: State::new()
        }
    }

    pub fn tick(&mut self, bus: &mut Databus) {
        //let opcode: u8 = bus.read(self.state.get_pc());


        //instruction::parse(opcode);

        let instruction = instruction::Instruction::parse(&mut self.state,  bus);

        instruction.execute(&mut self.state, bus);

        //println!("opcode {:#X}", opcode);
    }
}


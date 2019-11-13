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

        // TODO FIX SIZING
        let next_instruction_binary = bus.read_slice(self.state.get_pc(), 3);
        let instr = instruction::parse_instruction(next_instruction_binary);

        self.state.offset_next_pc(instr.get_size() as i8);
        instr.execute(&mut self.state, bus);
        self.state.update_pc();

    }

    pub fn get_state(&self) -> &State { &self.state }
}


use super::state::State;
use super::databus::Databus;
use super::instruction;
use crate::cpu::instruction::Instruction;

const NMI_VECTOR_ADDRESS: u16 = 0xFFFA;
const RES_VECTOR_ADDRESS: u16 = 0xFFFC;
const IRQ_VECTOR_ADDRESS: u16 = 0xFFFE;

pub struct Cpu {
    state: State,

    next_instruction: Instruction,
    unspent_cycles: u32,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            state: State::new(),
            next_instruction: instruction::DUMMY_INSTRUCTION,
            unspent_cycles: 0,
        }
    }

    pub fn set_irq_lo(&mut self) {}
    pub fn set_irq_hi(&mut self) {}

    pub fn set_nmi_hi(&mut self) {}
    pub fn set_nmi_lo(&mut self) {}

    pub fn reset(&mut self, bus: &Databus) {
        self.state.clear();

        let pc = bus.read_u16(RES_VECTOR_ADDRESS);
        self.state.set_next_pc(pc);
        self.state.update_pc();

        self._load_next_instruction(bus);

        self.unspent_cycles = 0;
    }

    pub fn tick(&mut self, bus: &mut Databus) {
        self.unspent_cycles += 1;

        let cycle_cost = self.next_instruction.get_cycle_cost(
            self.get_state(), bus) as u32;

        if self.unspent_cycles >= cycle_cost {
            self.unspent_cycles -= cycle_cost;

            let instruction = &self.next_instruction;
            self.state.set_next_pc(self.state.calculate_relative_pc(instruction.get_size() as i8));

            instruction.execute(&mut self.state, bus);

            self.state.update_pc();

            self._load_next_instruction(bus);
        }
    }


    pub fn get_state(&self) -> &State { &self.state }

    pub fn _load_next_instruction(&mut self, bus: &Databus) {
        let next_instruction_binary = bus.read_slice(self.state.get_pc(), 3);
        self.next_instruction = instruction::decode_instruction(next_instruction_binary);
    }
}


use super::state::State;
use super::databus::Databus;
use super::instruction;

const NMI_VECTOR_ADDRESS: u16 = 0xFFFA;
const RES_VECTOR_ADDRESS: u16 = 0xFFFC;
const IRQ_VECTOR_ADDRESS: u16 = 0xFFFE;

pub struct Cpu {
    state: State
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            state: State::new()
        }
    }

    pub fn set_irq_lo(&mut self) {}
    pub fn set_irq_hi(&mut self) {}

    pub fn set_nmi_hi(&mut self) {}
    pub fn set_nmi_lo(&mut self) {}

    pub fn reset(&mut self) {

    }


    pub fn tick(&mut self, bus: &mut Databus) {

        // TODO FIX SIZING
        let next_instruction_binary = bus.read_slice(self.state.get_pc(), 3);
        let instr = instruction::decode_instruction(next_instruction_binary);

        self.state.set_next_pc(self.state.calculate_relative_pc(instr.get_size() as i8));

        instr.execute(&mut self.state, bus);
        self.state.update_pc();
    }

    pub fn get_state(&self) -> &State { &self.state }
}


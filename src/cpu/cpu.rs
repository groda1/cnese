use super::state::State;
use super::databus::Databus;
use super::instruction;
use crate::cpu::instruction::Instruction;

pub const NMI_VECTOR_ADDRESS: u16 = 0xFFFA;
pub const RES_VECTOR_ADDRESS: u16 = 0xFFFC;
pub const IRQ_VECTOR_ADDRESS: u16 = 0xFFFE;

pub const STACK_OFFSET: u16 = 0x0100;

pub struct Cpu {
    state: State,

    next_instruction: Instruction,
    next_instruction_cost: u8,
    unspent_cycles: u8,

    cycle_count: u32,
    instruction_count: u32,

    irq: bool,
    nmi: bool,
    nmi_seen_hi: bool
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            state: State::new(),
            next_instruction: instruction::DUMMY_INSTRUCTION,
            next_instruction_cost: 0,
            unspent_cycles: 0,

            cycle_count: 0,
            instruction_count: 0,

            irq: true,
            nmi: true,
            nmi_seen_hi: true
        }
    }

    pub fn set_irq_lo(&mut self) { self.irq = false;}
    pub fn set_irq_hi(&mut self) { self.irq = true;}
    pub fn set_nmi_hi(&mut self) {
        self.nmi = true;
        self.nmi_seen_hi = true;
    }
    pub fn set_nmi_lo(&mut self) { self.nmi = false; }

    pub fn reset(&mut self, bus: &dyn Databus) {
        self.state.clear();

        let pc = bus.read_u16(RES_VECTOR_ADDRESS);
        self.state.set_next_pc(pc);
        self.state.update_pc();

        self._load_next_instruction(bus);

        self.unspent_cycles = 0;

        #[cfg(debug_assertions)] {
            println!("Reset NES, PC=${:x}", pc);
        }
    }

    pub fn tick(&mut self, bus: &mut dyn Databus) {
        self.unspent_cycles += 1;
        self.cycle_count += 1;

        if self.unspent_cycles >= self.next_instruction_cost {
            self.unspent_cycles -= self.next_instruction_cost;

            self._execute_next_instruction(bus);

        }
    }

    pub fn tick_instruction(&mut self, bus: &mut dyn Databus) {
        self.cycle_count += self.next_instruction_cost as u32;
        self._execute_next_instruction(bus);
    }

    pub fn get_state(&self) -> &State { &self.state }
    pub fn get_cycle_count(&self) -> u32 { self.cycle_count }
    pub fn get_instruction_count(&self) -> u32 { self.instruction_count }

    pub fn _execute_next_instruction(&mut self, bus: &mut dyn Databus) {
        let instruction = &self.next_instruction;

        self.state.set_next_pc(self.state.calculate_relative_pc(instruction.get_size() as i8));
        instruction.execute(&mut self.state, bus);
        self.state.update_pc();

        self._load_next_instruction(bus);

        self.instruction_count += 1;
    }

    pub fn _load_next_instruction(&mut self, bus: &dyn Databus) {
        if !self.nmi && self.nmi_seen_hi { // NMI
            self.nmi_seen_hi = false;
            self.next_instruction = instruction::NMI_INSTRUCTION;
        } else if !self.irq && !self.state.get_status_field(super::state::SR_MASK_INTERRUPT) { // IRQ
            self.next_instruction = instruction::IRQ_INSTRUCTION;
        } else {
            self.next_instruction = instruction::decode_instruction(bus, self.state.get_pc());
        }

        self.next_instruction_cost = self.next_instruction.calculate_cycle_cost(self.get_state(), bus);
    }

}


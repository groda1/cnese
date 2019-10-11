use super::databus::Databus;
use super::state::State;
use super::addressing;


pub type OpCode = fn(state: &mut State, bus: &mut Databus, operand: u16);


const NOT_IMPLEMENTED: OpCode = |state: &mut State, bus: &mut Databus, operand: u16| {
    println!("Not Implemented!");
};

const INX: OpCode = |state: &mut State, bus: &mut Databus, operand: u16| {
    state.x += 1;

    println!("INX (x=0x{:02x})  (pc = 0x{:04x})", state.x, state.get_pc());
};

const INY: OpCode = |state: &mut State, bus: &mut Databus, operand: u16| {
    state.y += 1;

    println!("INY (y=0x{:02x})  (pc = 0x{:04x})", state.y, state.get_pc());
};


const JMP: OpCode = |state: &mut State, bus: &mut Databus, operand: u16| {
    state.set_pc(operand);

    println!("JMP {:X}", operand);
};


pub struct Instruction {
    opcode: OpCode,
    addressing_mode: addressing::AddressingMode,
}

impl Instruction {
    fn create(opcode: OpCode, addressing_mode: addressing::AddressingMode) -> Instruction {
        Instruction {opcode, addressing_mode,
        }
    }

    pub fn parse(state: &mut State, bus: &mut Databus) -> Instruction {
        let opcode = bus.read(state.get_pc());

        INSTRUCTION_SET[opcode as usize]
    }

    pub fn execute(&self, state: &mut State, bus: &mut Databus) {
        state.offset_pc(1);

        let addressing = self.addressing_mode;
        let address = addressing(state, bus);

        let operation = self.opcode;

        operation(state, bus, address);
    }
}

impl Copy for Instruction {}

impl Clone for Instruction {
    fn clone(&self) -> Instruction {
        Instruction { opcode: self.opcode, addressing_mode: self.addressing_mode }
    }
}

lazy_static! {
    static ref INSTRUCTION_SET: Vec<Instruction> = {

        let unknown = Instruction { opcode: NOT_IMPLEMENTED, addressing_mode: addressing::IMPLIED};

        let mut instructions = vec![unknown; 256];

        instructions[0xC8] = Instruction::create(INY, addressing::IMPLIED);
        instructions[0xE8] = Instruction::create(INX, addressing::IMPLIED);
        instructions[0x4C] = Instruction::create(JMP, addressing::ABSOLUTE);



        instructions
    };
}


// let instruction: Instruction = INSTRUCTIONS[opcode as usize];


//if instruction as usize == NOT_IMPLEMENTED as usize {
//    panic!("Instruction not implemented: {:X}", opcode);
//}

//instruction



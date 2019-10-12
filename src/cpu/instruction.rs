use super::databus::Databus;
use super::state::State;
use super::addressing::AddressingMode;

use enum_map::EnumMap;


#[derive(Clone, Copy, Enum)]
enum Operation {
    INX,
    INY,
    JMP,
    UNKNOWN,
}

impl Operation {
    fn as_str(&self) -> &'static str {
        match *self {
            Operation::INX => "INX",
            Operation::INY => "INY",
            Operation::JMP => "JMP",
            _ => "Not Defined!"
        }
    }

    fn get_fn(&self) -> OperationFn {
        OPERATION_FN_MAP[*self]
    }
}

type OperationFn = fn(state: &mut State, bus: &mut Databus, operand: u16);

const NOT_IMPLEMENTED: OperationFn = |_state: &mut State, _bus: &mut Databus, _operand: u16| {
    println!("Not Implemented!");
};

const INX: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.x += 1;
};

const INY: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.y += 1;
};

const JMP: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.set_pc(operand);
};



lazy_static! {
    static ref OPERATION_FN_MAP: EnumMap<Operation, OperationFn> = {
        let map = enum_map! {
            Operation::INX => INX,
            Operation::INY => INY,
            Operation::JMP => JMP,
            Operation::UNKNOWN => NOT_IMPLEMENTED
        };
        map
    };
}

lazy_static! {
    static ref OPCODE_SET: Vec <Opcode> = {
        let unknown = Opcode::new(Operation::UNKNOWN, AddressingMode::UNKNOWN, 1);
        let mut opcodes = vec ! [unknown; 256];

        opcodes[0xc8] = Opcode::new(Operation::INY, AddressingMode::IMPLIED, 1);
        opcodes[0xe8] = Opcode::new(Operation::INX, AddressingMode::IMPLIED, 1);
        opcodes[0x4c] = Opcode::new(Operation::JMP, AddressingMode::ABSOLUTE, 3);


        opcodes
    };
}

#[derive(Clone, Copy)]
struct Opcode {
    operation : Operation,
    mode : AddressingMode,
    size : u8
}

impl Opcode {
    fn new(operation: Operation, mode: AddressingMode, size: u8) -> Opcode {
        Opcode { operation, mode, size }
    }
}

#[derive(Clone, Copy)]
pub struct Instruction {
    opcode: Opcode,
    operand: u16,
}

impl Instruction {
    fn new(opcode: Opcode, operand: u16) -> Instruction {
        Instruction { opcode, operand }
    }

    pub fn execute(&self, state: &mut State, bus: &mut Databus) {
        let evalued_operand = self.opcode.mode.eval(state, bus, self.operand);
        self.opcode.operation.get_fn()(state, bus, evalued_operand);
    }

    pub fn get_size(&self) -> u8 {
        self.opcode.size
    }

    pub fn format(&self) -> String {
        format!("{} {}", self.opcode.operation.as_str(), self.opcode.mode.format(self.operand))
    }
}

pub fn parse_instruction(prg: &[u8]) -> Instruction {
    let opcode =  OPCODE_SET[prg[0] as usize];

    let mut operand = 0;
    if opcode.size == 2 {
        operand = prg[1] as u16;
    } else if opcode.size == 3 {
        operand = ((prg[2] as u16) << 8) + prg[1] as u16;
    }

    Instruction::new(opcode, operand)
}

pub fn deassemble(rom: &[u8]) -> Vec<Instruction> {

    // TODO REFAC
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut i: usize = 0;
    let size = rom.len();

    while i < size {
        let opcode = OPCODE_SET[rom[i] as usize];

        let mut operand = 0;
        if opcode.size == 2 {
            operand = rom[i + 1] as u16;
        } else if opcode.size == 3 {
            operand = ((rom[i + 2] as u16) << 8) + rom[i + 1] as u16;
        }

        let x = Instruction::new(OPCODE_SET[rom[i] as usize], operand);
        instructions.push(x);

        i += opcode.size as usize;
    }

    instructions
}



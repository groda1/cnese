use super::databus::Databus;
use super::state::State;
use super::state;
use super::addressing::AddressingMode;

use enum_map::EnumMap;


#[derive(Clone, Copy, Enum)]
enum Operation {
    CLC,
    CLD,
    CLI,
    CLV,
    INX,
    INY,
    JMP,
    LDA,
    NOP,
    SEC,
    SED,
    SEI,

    UNKNOWN,
}

impl Operation {
    fn as_str(&self) -> &'static str {
        match *self {
            Operation::CLC => "CLC",
            Operation::CLD => "CLD",
            Operation::CLI => "CLI",
            Operation::CLV => "CLV",
            Operation::INX => "INX",
            Operation::INY => "INY",
            Operation::JMP => "JMP",
            Operation::LDA => "LDA",
            Operation::NOP => "NOP",
            Operation::SEC => "SEC",
            Operation::SED => "SED",
            Operation::SEI => "SEI",
            _ => "##"
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

//V = 0 when (0xFF00 + U1) - U2 >= 0xFE80 and (0xFF00 + U1) - U2 <= 0xFF7F
//V = 1 when (0xFF00 + U1) - U2 <  0xFE80 or  (0xFF00 + U1) - U2 >  0xFF7F


const CLC: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status(state::SR_MASK_CARRY, false);
};

const CLD: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status(state::SR_MASK_DECIMAL, false);
};

const CLI: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status(state::SR_MASK_INTERRUPT, false);
};

const CLV: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status(state::SR_MASK_OVERFLOW, false);
};

const INX: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.x = state.x.wrapping_add(1);

    state.set_status(state::SR_MASK_NEGATIVE, state.x >= 128);
    state.set_status(state::SR_MASK_ZERO, state.x == 0);
};

const INY: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.y = state.y.wrapping_add(1);

    state.set_status(state::SR_MASK_NEGATIVE, state.y >= 128);
    state.set_status(state::SR_MASK_ZERO, state.y == 0);
};

const JMP: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.set_pc(operand);
};

const LDA: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.acc = operand as u8;

    state.set_status(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status(state::SR_MASK_ZERO, state.acc == 0);
};

const NOP: OperationFn = |_state: &mut State, _bus: &mut Databus, _operand: u16| {};

const SEC: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status(state::SR_MASK_CARRY, true);
};

const SED: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status(state::SR_MASK_DECIMAL, true);
};

const SEI: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status(state::SR_MASK_INTERRUPT, true);
};

lazy_static! {
    static ref OPERATION_FN_MAP: EnumMap<Operation, OperationFn> = {
        let map = enum_map! {
            Operation::CLC => CLC,
            Operation::CLD => CLD,
            Operation::CLI => CLI,
            Operation::CLV => CLV,
            Operation::INX => INX,
            Operation::INY => INY,
            Operation::LDA => LDA,
            Operation::JMP => JMP,
            Operation::NOP => NOP,
            Operation::SEC => SEC,
            Operation::SED => SED,
            Operation::SEI => SEI,
            Operation::UNKNOWN => NOT_IMPLEMENTED
        };
        map
    };
}

lazy_static! {
    static ref OPCODE_SET: Vec <Opcode> = {
        let unknown = Opcode::new(Operation::UNKNOWN, AddressingMode::Unknown, 1, 0, false);
        let mut opcodes = vec ! [unknown; 256];

        opcodes[0x18] = Opcode::new(Operation::CLC, AddressingMode::Implied, 1, 2, false);
        opcodes[0xd8] = Opcode::new(Operation::CLD, AddressingMode::Implied, 1, 2, false);
        opcodes[0x58] = Opcode::new(Operation::CLI, AddressingMode::Implied, 1, 2, false);
        opcodes[0xb8] = Opcode::new(Operation::CLV, AddressingMode::Implied, 1, 2, false);

        opcodes[0xc8] = Opcode::new(Operation::INY, AddressingMode::Implied, 1, 2, false);
        opcodes[0xe8] = Opcode::new(Operation::INX, AddressingMode::Implied, 1, 2, false);

        opcodes[0xa9] = Opcode::new(Operation::LDA, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xa5] = Opcode::new(Operation::LDA, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xb5] = Opcode::new(Operation::LDA, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0xad] = Opcode::new(Operation::LDA, AddressingMode::Absolute, 3, 4, false);
        opcodes[0xbd] = Opcode::new(Operation::LDA, AddressingMode::AbsoluteIndexedX, 3, 4, true);
        opcodes[0xb9] = Opcode::new(Operation::LDA, AddressingMode::AbsoluteIndexedY, 3, 4, true);
        opcodes[0xa1] = Opcode::new(Operation::LDA, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0xb1] = Opcode::new(Operation::LDA, AddressingMode::IndirectIndexedY, 2, 5, true);

        opcodes[0xea] = Opcode::new(Operation::NOP, AddressingMode::Implied, 1, 2, false);

        opcodes[0x4c] = Opcode::new(Operation::JMP, AddressingMode::Absolute, 3, 3, false);
        opcodes[0x6c] = Opcode::new(Operation::JMP, AddressingMode::Indirect, 3, 5, false);

        opcodes[0x38] = Opcode::new(Operation::SEC, AddressingMode::Implied, 1, 2, false);
        opcodes[0xf8] = Opcode::new(Operation::SED, AddressingMode::Implied, 1, 2, false);
        opcodes[0x78] = Opcode::new(Operation::SEI, AddressingMode::Implied, 1, 2, false);

        opcodes
    };
}

#[derive(Clone, Copy)]
struct Opcode {
    operation : Operation,
    mode : AddressingMode,
    size : u8,
    cycles : u8,
    page_boundary_penalty: bool
}

impl Opcode {
    fn new(operation: Operation,
           mode: AddressingMode,
           size: u8,
           cycles: u8,
           page_boundary_penalty: bool) -> Opcode {
        Opcode { operation, mode, size, cycles, page_boundary_penalty }
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



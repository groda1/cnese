use super::databus::Databus;
use super::state::State;
use super::state;
use super::addressing::AddressingMode;

#[derive(Clone, Copy)]
enum Operation {
    ADC,
    AND,
    CLC,
    CLD,
    CLI,
    CLV,
    INX,
    INY,
    JMP,
    LDA,
    LDX,
    LDY,
    NOP,
    SBC,
    SEC,
    SED,
    SEI,
    STA,

    UNKNOWN,
}

impl Operation {
    fn as_str(&self) -> &'static str {
        match *self {
            Operation::ADC => "ADC",
            Operation::AND => "AND",
            Operation::CLC => "CLC",
            Operation::CLD => "CLD",
            Operation::CLI => "CLI",
            Operation::CLV => "CLV",
            Operation::INX => "INX",
            Operation::INY => "INY",
            Operation::JMP => "JMP",
            Operation::LDA => "LDA",
            Operation::LDX => "LDX",
            Operation::LDY => "LDY",
            Operation::NOP => "NOP",
            Operation::SBC => "SBC",
            Operation::SEC => "SEC",
            Operation::SED => "SED",
            Operation::SEI => "SEI",
            Operation::STA => "STA",
            _ => "##"
        }
    }

    fn get_fn(&self) -> OperationFn {
        match *self {
            Operation::ADC => ADC,
            Operation::AND => AND,
            Operation::CLC => CLC,
            Operation::CLD => CLD,
            Operation::CLI => CLI,
            Operation::CLV => CLV,
            Operation::INX => INX,
            Operation::INY => INY,
            Operation::LDA => LDA,
            Operation::LDX => LDX,
            Operation::LDY => LDY,
            Operation::JMP => JMP,
            Operation::NOP => NOP,
            Operation::SBC => SBC,
            Operation::SEC => SEC,
            Operation::SED => SED,
            Operation::SEI => SEI,
            Operation::STA => STA,
            Operation::UNKNOWN => NOT_IMPLEMENTED
        }
    }
}

type OperationFn = fn(state: &mut State, bus: &mut Databus, operand: u16);

const NOT_IMPLEMENTED: OperationFn = |_state: &mut State, _bus: &mut Databus, _operand: u16| {
    println!("Not Implemented!");
};

const ADC: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    adc(state, operand as u8);
};

const AND: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.acc &= operand as u8;

    state.set_status(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status(state::SR_MASK_ZERO, state.acc == 0);
};

const SBC: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    adc(state, !(operand as u8));
};

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

const LDX: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.x = operand as u8;

    state.set_status(state::SR_MASK_NEGATIVE, state.x >= 128);
    state.set_status(state::SR_MASK_ZERO, state.x == 0);
};

const LDY: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.y = operand as u8;

    state.set_status(state::SR_MASK_NEGATIVE, state.y >= 128);
    state.set_status(state::SR_MASK_ZERO, state.y == 0);
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

const STA: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    bus.write(operand, state.acc);
};


fn adc(state: &mut State, operand: u8) {
    let sum = state.acc.wrapping_add(operand).wrapping_add(state.get_status(state::SR_MASK_CARRY) as u8);
    let carry = (operand as u16 + state.acc as u16 + state.get_status(state::SR_MASK_CARRY) as u16) > 0xff;
    let overflow = (!(state.acc ^ operand) & (state.acc ^ sum) & 0x80) > 0;
    state.acc = sum;

    state.set_status(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status(state::SR_MASK_ZERO, state.acc == 0);
    state.set_status(state::SR_MASK_CARRY, carry);
    state.set_status(state::SR_MASK_OVERFLOW, overflow);
}

lazy_static! {
    static ref OPCODE_SET: Vec <Opcode> = {
        let unknown = Opcode::new(Operation::UNKNOWN, AddressingMode::Unknown, 1, 0, false);
        let mut opcodes = vec ! [unknown; 256];

        opcodes[0x69] = Opcode::new(Operation::ADC, AddressingMode::Immediate, 2, 2, false);
        opcodes[0x65] = Opcode::new(Operation::ADC, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0x75] = Opcode::new(Operation::ADC, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0x6d] = Opcode::new(Operation::ADC, AddressingMode::Absolute, 3, 4, false);
        opcodes[0x7d] = Opcode::new(Operation::ADC, AddressingMode::AbsoluteIndexedX, 3, 4, true);
        opcodes[0x79] = Opcode::new(Operation::ADC, AddressingMode::AbsoluteIndexedY, 3, 4, true);
        opcodes[0x61] = Opcode::new(Operation::ADC, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0x71] = Opcode::new(Operation::ADC, AddressingMode::IndirectIndexedY, 2, 5, true);

        opcodes[0x29] = Opcode::new(Operation::AND, AddressingMode::Immediate, 2, 2, false);
        opcodes[0x25] = Opcode::new(Operation::AND, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0x35] = Opcode::new(Operation::AND, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0x2d] = Opcode::new(Operation::AND, AddressingMode::Absolute, 3, 4, false);
        opcodes[0x3d] = Opcode::new(Operation::AND, AddressingMode::AbsoluteIndexedX, 3, 4, true);
        opcodes[0x39] = Opcode::new(Operation::AND, AddressingMode::AbsoluteIndexedY, 3, 4, true);
        opcodes[0x21] = Opcode::new(Operation::AND, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0x31] = Opcode::new(Operation::AND, AddressingMode::IndirectIndexedY, 2, 5, true);

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

        opcodes[0xa2] = Opcode::new(Operation::LDX, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xa6] = Opcode::new(Operation::LDX, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xb6] = Opcode::new(Operation::LDX, AddressingMode::ZeropageIndexedY, 2, 4, false);
        opcodes[0xae] = Opcode::new(Operation::LDX, AddressingMode::Absolute, 3, 4, false);
        opcodes[0xbe] = Opcode::new(Operation::LDX, AddressingMode::AbsoluteIndexedY, 3, 4, true);

        opcodes[0xa0] = Opcode::new(Operation::LDY, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xa4] = Opcode::new(Operation::LDY, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xb4] = Opcode::new(Operation::LDY, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0xac] = Opcode::new(Operation::LDY, AddressingMode::Absolute, 3, 4, false);
        opcodes[0xbc] = Opcode::new(Operation::LDY, AddressingMode::AbsoluteIndexedX, 3, 4, true);

        opcodes[0xea] = Opcode::new(Operation::NOP, AddressingMode::Implied, 1, 2, false);

        opcodes[0x4c] = Opcode::new(Operation::JMP, AddressingMode::Absolute, 3, 3, false);
        opcodes[0x6c] = Opcode::new(Operation::JMP, AddressingMode::Indirect, 3, 5, false);

        opcodes[0xe9] = Opcode::new(Operation::SBC, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xe5] = Opcode::new(Operation::SBC, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xf5] = Opcode::new(Operation::SBC, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0xed] = Opcode::new(Operation::SBC, AddressingMode::Absolute, 3, 4, false);
        opcodes[0xfd] = Opcode::new(Operation::SBC, AddressingMode::AbsoluteIndexedX, 3, 4, true);
        opcodes[0xf9] = Opcode::new(Operation::SBC, AddressingMode::AbsoluteIndexedY, 3, 4, true);
        opcodes[0xe1] = Opcode::new(Operation::SBC, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0xf1] = Opcode::new(Operation::SBC, AddressingMode::IndirectIndexedY, 2, 5, true);

        opcodes[0x38] = Opcode::new(Operation::SEC, AddressingMode::Implied, 1, 2, false);
        opcodes[0xf8] = Opcode::new(Operation::SED, AddressingMode::Implied, 1, 2, false);
        opcodes[0x78] = Opcode::new(Operation::SEI, AddressingMode::Implied, 1, 2, false);

        opcodes[0x85] = Opcode::new(Operation::STA, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0x95] = Opcode::new(Operation::STA, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0x8d] = Opcode::new(Operation::STA, AddressingMode::Absolute, 3, 4, false);
        opcodes[0x9d] = Opcode::new(Operation::STA, AddressingMode::AbsoluteIndexedX, 3, 5, false);
        opcodes[0x99] = Opcode::new(Operation::STA, AddressingMode::AbsoluteIndexedY, 3, 5, false);
        opcodes[0x81] = Opcode::new(Operation::STA, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0x91] = Opcode::new(Operation::STA, AddressingMode::IndirectIndexedY, 2, 6, false);

        opcodes
    };
}

#[derive(Clone, Copy)]
struct Opcode {
    operation: Operation,
    mode: AddressingMode,
    size: u8,
    cycles: u8,
    page_boundary_penalty: bool,
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
    let opcode = OPCODE_SET[prg[0] as usize];

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



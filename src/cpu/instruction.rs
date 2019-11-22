use super::databus::Databus;
use super::state::State;
use super::state;
use super::addressing::AddressingMode;
use super::cpu;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
enum Operation {
    ADC_IMM,
    ADC_MEM,
    AND_IMM,
    AND_MEM,
    ASL_ACC,
    ASL_MEM,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP_IMM,
    CMP_MEM,
    CPX_IMM,
    CPX_MEM,
    CPY_IMM,
    CPY_MEM,
    DEC,
    DEX,
    DEY,
    EOR_IMM,
    EOR_MEM,
    INC,
    INX,
    INY,
    JMP,
    LDA_IMM,
    LDA_MEM,
    LDX_IMM,
    LDX_MEM,
    LDY_IMM,
    LDY_MEM,
    NOP,
    SBC_IMM,
    SBC_MEM,
    SEC,
    SED,
    SEI,
    STA,
    RTI,

    UNKNOWN,
    INTERNAL_IRQ,
    INTERNAL_NMI,
}

impl Operation {
    fn as_str(&self) -> &'static str {
        match *self {
            Operation::ADC_IMM => "ADC",
            Operation::ADC_MEM => "ADC",
            Operation::AND_IMM => "AND",
            Operation::AND_MEM => "AND",
            Operation::ASL_ACC => "ASL",
            Operation::ASL_MEM => "ASL",
            Operation::BCC => "BCC",
            Operation::BCS => "BCS",
            Operation::BEQ => "BEQ",
            Operation::BIT => "BIT",
            Operation::BMI => "BMI",
            Operation::BNE => "BNE",
            Operation::BPL => "BPL",
            Operation::BRK => "BRK",
            Operation::BVC => "BVC",
            Operation::BVS => "BVS",
            Operation::CLC => "CLC",
            Operation::CLD => "CLD",
            Operation::CLI => "CLI",
            Operation::CLV => "CLV",
            Operation::CMP_IMM => "CMP",
            Operation::CMP_MEM => "CMP",
            Operation::CPX_IMM => "CPX",
            Operation::CPX_MEM => "CPX",
            Operation::CPY_IMM => "CPY",
            Operation::CPY_MEM => "CPY",
            Operation::DEC => "DEC",
            Operation::DEX => "DEX",
            Operation::DEY => "DEY",
            Operation::EOR_IMM => "EOR",
            Operation::EOR_MEM => "EOR",
            Operation::INC => "INC",
            Operation::INX => "INX",
            Operation::INY => "INY",
            Operation::JMP => "JMP",
            Operation::LDA_IMM => "LDA",
            Operation::LDA_MEM => "LDA",
            Operation::LDX_IMM => "LDX",
            Operation::LDX_MEM => "LDX",
            Operation::LDY_IMM => "LDY",
            Operation::LDY_MEM => "LDY",
            Operation::NOP => "NOP",
            Operation::SBC_IMM => "SBC",
            Operation::SBC_MEM => "SBC",
            Operation::SEC => "SEC",
            Operation::SED => "SED",
            Operation::SEI => "SEI",
            Operation::STA => "STA",
            Operation::RTI => "RTI",
            _ => "##"
        }
    }

    fn get_fn(&self) -> OperationFn {
        match *self {
            Operation::ADC_IMM => ADC_IMM,
            Operation::ADC_MEM => ADC_MEM,
            Operation::AND_IMM => AND_IMM,
            Operation::AND_MEM => AND_MEM,
            Operation::ASL_ACC => ASL_ACC,
            Operation::ASL_MEM => ASL_MEM,
            Operation::BCC => BCC,
            Operation::BCS => BCS,
            Operation::BEQ => BEQ,
            Operation::BIT => BIT,
            Operation::BMI => BMI,
            Operation::BNE => BNE,
            Operation::BPL => BPL,
            Operation::BRK => BRK,
            Operation::BVC => BVC,
            Operation::BVS => BVS,
            Operation::CLC => CLC,
            Operation::CLD => CLD,
            Operation::CLI => CLI,
            Operation::CLV => CLV,
            Operation::CMP_IMM => CMP_IMM,
            Operation::CMP_MEM => CMP_MEM,
            Operation::CPX_IMM => CPX_IMM,
            Operation::CPX_MEM => CPX_MEM,
            Operation::CPY_IMM => CPY_IMM,
            Operation::CPY_MEM => CPY_MEM,
            Operation::DEC => DEC,
            Operation::DEX => DEX,
            Operation::DEY => DEY,
            Operation::EOR_IMM => EOR_IMM,
            Operation::EOR_MEM => EOR_MEM,
            Operation::INC => INC,
            Operation::INX => INX,
            Operation::INY => INY,
            Operation::LDA_IMM => LDA_IMM,
            Operation::LDA_MEM => LDA_MEM,
            Operation::LDX_IMM => LDX_IMM,
            Operation::LDX_MEM => LDX_MEM,
            Operation::LDY_IMM => LDY_IMM,
            Operation::LDY_MEM => LDY_MEM,
            Operation::JMP => JMP,
            Operation::NOP => NOP,
            Operation::SBC_IMM => SBC_IMM,
            Operation::SBC_MEM => SBC_MEM,
            Operation::SEC => SEC,
            Operation::SED => SED,
            Operation::SEI => SEI,
            Operation::STA => STA,
            Operation::RTI => RTI,
            Operation::UNKNOWN => NOT_IMPLEMENTED,
            Operation::INTERNAL_IRQ => INTERNAL_IRQ_FN,
            Operation::INTERNAL_NMI => INTERNAL_NMI_FN,
        }
    }
}

type OperationFn = fn(state: &mut State, bus: &mut Databus, operand: u16);

const NOT_IMPLEMENTED: OperationFn = |_state: &mut State, _bus: &mut Databus, _operand: u16| {
    println!("Not Implemented!");
};

const ADC_IMM: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    _adc(state, operand as u8);
};

const ADC_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    _adc(state, bus.read(operand));
};

const AND_IMM: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.acc &= operand as u8;

    state.set_status_field(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.acc == 0);
};

const AND_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    state.acc &= bus.read(operand);

    state.set_status_field(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.acc == 0);
};

const ASL_ACC: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    let overflow = (state.acc & 0x80) > 0;
    state.acc <<= 1;

    state.set_status_field(state::SR_MASK_CARRY, overflow);
    state.set_status_field(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.acc == 0);
};

const ASL_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    let mut value = bus.read(operand);
    let overflow = (value & 0x80) > 0;
    value <<= 1;
    bus.write(operand, value);

    state.set_status_field(state::SR_MASK_CARRY, overflow);
    state.set_status_field(state::SR_MASK_NEGATIVE, value >= 128);
    state.set_status_field(state::SR_MASK_ZERO, value == 0);
};

const BCC: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    if _should_bcc(state) {
        state.set_next_pc(operand);
    }
};

const BCS: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    if _should_bcs(state) {
        state.set_next_pc(operand);
    }
};

const BEQ: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    if _should_beq(state) {
        state.set_next_pc(operand);
    }
};

const BIT: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    let op = bus.read(operand);
    state.set_status_field(state::SR_MASK_NEGATIVE, (op & state::SR_MASK_NEGATIVE) > 0);
    state.set_status_field(state::SR_MASK_OVERFLOW, (op & state::SR_MASK_OVERFLOW) > 0);
    state.set_status_field(state::SR_MASK_ZERO, (op & state.acc) > 0);
};

const BMI: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    if _should_bmi(state) {
        state.set_next_pc(operand);
    }
};

const BNE: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    if _should_bne(state) {
        state.set_next_pc(operand);
    }
};

const BPL: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    if _should_bpl(state) {
        state.set_next_pc(operand);
    }
};

const BRK: OperationFn = |state: &mut State, bus: &mut Databus, _operand: u16| {
    state.set_next_pc(state.calculate_relative_pc(1));
    _handle_interrupt(state, bus, cpu::IRQ_VECTOR_ADDRESS, true);
};

const BVC: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    if _should_bvc(state) {
        state.set_next_pc(operand);
    }
};

const BVS: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    if _should_bvs(state) {
        state.set_next_pc(operand);
    }
};

const CLC: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status_field(state::SR_MASK_CARRY, false);
};

const CLD: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status_field(state::SR_MASK_DECIMAL, false);
};

const CLI: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status_field(state::SR_MASK_INTERRUPT, false);
};

const CLV: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status_field(state::SR_MASK_OVERFLOW, false);
};

const CMP_IMM: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    _compare(state, operand as u8, state.acc);
};

const CMP_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    _compare(state, bus.read(operand), state.acc);
};

const CPY_IMM: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    _compare(state, operand as u8, state.y);
};

const CPY_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    _compare(state, bus.read(operand), state.y);
};

const CPX_IMM: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    _compare(state, operand as u8, state.x);
};

const CPX_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    _compare(state, bus.read(operand), state.x);
};

const DEC: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    let value = bus.read(operand).wrapping_sub(1);
    bus.write(operand, value);

    state.set_status_field(state::SR_MASK_NEGATIVE, value >= 128);
    state.set_status_field(state::SR_MASK_ZERO, value == 0);
};

const DEX: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.x = state.x.wrapping_sub(1);

    state.set_status_field(state::SR_MASK_NEGATIVE, state.x >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.x == 0);
};

const DEY: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.y = state.y.wrapping_sub(1);

    state.set_status_field(state::SR_MASK_NEGATIVE, state.y >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.y == 0);
};

const EOR_IMM: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.acc ^= operand as u8;

    state.set_status_field(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.acc == 0);
};

const EOR_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    state.acc ^= bus.read(operand);

    state.set_status_field(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.acc == 0);
};

const INC: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    let value = bus.read(operand).wrapping_add(1);
    bus.write(operand, value);

    state.set_status_field(state::SR_MASK_NEGATIVE, value >= 128);
    state.set_status_field(state::SR_MASK_ZERO, value == 0);
};

const INX: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.x = state.x.wrapping_add(1);

    state.set_status_field(state::SR_MASK_NEGATIVE, state.x >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.x == 0);
};

const INY: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.y = state.y.wrapping_add(1);

    state.set_status_field(state::SR_MASK_NEGATIVE, state.y >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.y == 0);
};

const JMP: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.set_next_pc(operand);
};

const LDA_IMM: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.acc = operand as u8;

    state.set_status_field(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.acc == 0);
};

const LDA_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    state.acc = bus.read(operand);

    state.set_status_field(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.acc == 0);
};

const LDX_IMM: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.x = operand as u8;

    state.set_status_field(state::SR_MASK_NEGATIVE, state.x >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.x == 0);
};

const LDX_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    state.x = bus.read(operand);

    state.set_status_field(state::SR_MASK_NEGATIVE, state.x >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.x == 0);
};

const LDY_IMM: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    state.y = operand as u8;

    state.set_status_field(state::SR_MASK_NEGATIVE, state.y >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.y == 0);
};

const LDY_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    state.y = bus.read(operand);

    state.set_status_field(state::SR_MASK_NEGATIVE, state.y >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.y == 0);
};

const NOP: OperationFn = |_state: &mut State, _bus: &mut Databus, _operand: u16| {};

const SBC_IMM: OperationFn = |state: &mut State, _bus: &mut Databus, operand: u16| {
    _sbc(state, operand as u8);
};

const SBC_MEM: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    _sbc(state, bus.read(operand));
};

const SEC: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status_field(state::SR_MASK_CARRY, true);
};

const SED: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status_field(state::SR_MASK_DECIMAL, true);
};

const SEI: OperationFn = |state: &mut State, _bus: &mut Databus, _operand: u16| {
    state.set_status_field(state::SR_MASK_INTERRUPT, true);
};

const STA: OperationFn = |state: &mut State, bus: &mut Databus, operand: u16| {
    bus.write(operand, state.acc);
};

const RTI: OperationFn = |state: &mut State, bus: &mut Databus, _operand: u16| {
    let status_u8 = _pull_stack(state, bus);

    let mut status = state::Status::from_u8(status_u8);
    status.set(state::SR_MASK_BREAK, false);
    state.set_status(status);

    let pc_lo = _pull_stack(state, bus);
    let pc_hi = _pull_stack(state, bus);

    let next_pc = ((pc_hi as u16) << 8) + pc_lo as u16;
    state.set_next_pc(next_pc);
};

const INTERNAL_IRQ_FN: OperationFn = |state: &mut State, bus: &mut Databus, _operand: u16| {
    _handle_interrupt(state, bus, cpu::IRQ_VECTOR_ADDRESS, false);
};

const INTERNAL_NMI_FN: OperationFn = |state: &mut State, bus: &mut Databus, _operand: u16| {
    _handle_interrupt(state, bus, cpu::NMI_VECTOR_ADDRESS, false);
};

lazy_static! {
    static ref OPCODE_SET: Vec <Opcode> = {
        let unknown = Opcode::new(Operation::UNKNOWN, AddressingMode::Unknown, 1, 0, false);
        let mut opcodes = vec![unknown; 256];

        opcodes[0x69] = Opcode::new(Operation::ADC_IMM, AddressingMode::Immediate, 2, 2, false);
        opcodes[0x65] = Opcode::new(Operation::ADC_MEM, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0x75] = Opcode::new(Operation::ADC_MEM, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0x6d] = Opcode::new(Operation::ADC_MEM, AddressingMode::Absolute, 3, 4, false);
        opcodes[0x7d] = Opcode::new(Operation::ADC_MEM, AddressingMode::AbsoluteIndexedX, 3, 4, true);
        opcodes[0x79] = Opcode::new(Operation::ADC_MEM, AddressingMode::AbsoluteIndexedY, 3, 4, true);
        opcodes[0x61] = Opcode::new(Operation::ADC_MEM, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0x71] = Opcode::new(Operation::ADC_MEM, AddressingMode::IndirectIndexedY, 2, 5, true);

        opcodes[0x29] = Opcode::new(Operation::AND_IMM, AddressingMode::Immediate, 2, 2, false);
        opcodes[0x25] = Opcode::new(Operation::AND_MEM, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0x35] = Opcode::new(Operation::AND_MEM, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0x2d] = Opcode::new(Operation::AND_MEM, AddressingMode::Absolute, 3, 4, false);
        opcodes[0x3d] = Opcode::new(Operation::AND_MEM, AddressingMode::AbsoluteIndexedX, 3, 4, true);
        opcodes[0x39] = Opcode::new(Operation::AND_MEM, AddressingMode::AbsoluteIndexedY, 3, 4, true);
        opcodes[0x21] = Opcode::new(Operation::AND_MEM, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0x31] = Opcode::new(Operation::AND_MEM, AddressingMode::IndirectIndexedY, 2, 5, true);

        opcodes[0x0a] = Opcode::new(Operation::ASL_ACC, AddressingMode::Accumulator, 1, 2, false);
        opcodes[0x06] = Opcode::new(Operation::ASL_MEM, AddressingMode::Zeropage, 2, 5, false);
        opcodes[0x16] = Opcode::new(Operation::ASL_MEM, AddressingMode::ZeropageIndexedX, 2, 6, false);
        opcodes[0x0e] = Opcode::new(Operation::ASL_MEM, AddressingMode::Absolute, 3, 6, false);
        opcodes[0x1e] = Opcode::new(Operation::ASL_MEM, AddressingMode::AbsoluteIndexedX, 3, 7, false);

        opcodes[0x90] = Opcode::new(Operation::BCC, AddressingMode::Relative, 2, 2, true);
        opcodes[0xb0] = Opcode::new(Operation::BCS, AddressingMode::Relative, 2, 2, true);
        opcodes[0xf0] = Opcode::new(Operation::BEQ, AddressingMode::Relative, 2, 2, true);

        opcodes[0x24] = Opcode::new(Operation::BIT, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0x2c] = Opcode::new(Operation::BIT, AddressingMode::Absolute, 3, 4, false);

        opcodes[0x30] = Opcode::new(Operation::BMI, AddressingMode::Relative, 2, 2, true);
        opcodes[0xd0] = Opcode::new(Operation::BNE, AddressingMode::Relative, 2, 2, true);
        opcodes[0x10] = Opcode::new(Operation::BPL, AddressingMode::Relative, 2, 2, true);
        opcodes[0x50] = Opcode::new(Operation::BVC, AddressingMode::Relative, 2, 2, true);
        opcodes[0x70] = Opcode::new(Operation::BVS, AddressingMode::Relative, 2, 2, true);

        opcodes[0x00] = Opcode::new(Operation::BRK, AddressingMode::Implied, 1, 7, false);

        opcodes[0x18] = Opcode::new(Operation::CLC, AddressingMode::Implied, 1, 2, false);
        opcodes[0xd8] = Opcode::new(Operation::CLD, AddressingMode::Implied, 1, 2, false);
        opcodes[0x58] = Opcode::new(Operation::CLI, AddressingMode::Implied, 1, 2, false);
        opcodes[0xb8] = Opcode::new(Operation::CLV, AddressingMode::Implied, 1, 2, false);

        opcodes[0xc9] = Opcode::new(Operation::CMP_IMM, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xc5] = Opcode::new(Operation::CMP_MEM, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xd5] = Opcode::new(Operation::CMP_MEM, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0xcd] = Opcode::new(Operation::CMP_MEM, AddressingMode::Absolute, 3, 4, false);
        opcodes[0xdd] = Opcode::new(Operation::CMP_MEM, AddressingMode::AbsoluteIndexedX, 3, 4, true);
        opcodes[0xd9] = Opcode::new(Operation::CMP_MEM, AddressingMode::AbsoluteIndexedY, 3, 4, true);
        opcodes[0xc1] = Opcode::new(Operation::CMP_MEM, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0xd1] = Opcode::new(Operation::CMP_MEM, AddressingMode::IndirectIndexedY, 2, 5, true);

        opcodes[0xe0] = Opcode::new(Operation::CPX_IMM, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xe4] = Opcode::new(Operation::CPX_MEM, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xec] = Opcode::new(Operation::CPX_MEM, AddressingMode::Absolute, 3, 4, false);

        opcodes[0xc0] = Opcode::new(Operation::CPY_IMM, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xc4] = Opcode::new(Operation::CPY_MEM, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xcc] = Opcode::new(Operation::CPY_MEM, AddressingMode::Absolute, 3, 4, false);

        opcodes[0xc6] = Opcode::new(Operation::DEC, AddressingMode::Zeropage, 2, 5, false);
        opcodes[0xd6] = Opcode::new(Operation::DEC, AddressingMode::ZeropageIndexedX, 2, 6, false);
        opcodes[0xce] = Opcode::new(Operation::DEC, AddressingMode::Absolute, 3, 6, false);
        opcodes[0xde] = Opcode::new(Operation::DEC, AddressingMode::AbsoluteIndexedX, 3, 7, false);

        opcodes[0xca] = Opcode::new(Operation::DEX, AddressingMode::Implied, 1, 2, false);
        opcodes[0x88] = Opcode::new(Operation::DEY, AddressingMode::Implied, 1, 2, false);

        opcodes[0x49] = Opcode::new(Operation::EOR_IMM, AddressingMode::Immediate, 2, 2, false);
        opcodes[0x45] = Opcode::new(Operation::EOR_MEM, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0x55] = Opcode::new(Operation::EOR_MEM, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0x4d] = Opcode::new(Operation::EOR_MEM, AddressingMode::Absolute, 3, 4, false);
        opcodes[0x5d] = Opcode::new(Operation::EOR_MEM, AddressingMode::AbsoluteIndexedX, 3, 4, true);
        opcodes[0x59] = Opcode::new(Operation::EOR_MEM, AddressingMode::AbsoluteIndexedY, 3, 4, true);
        opcodes[0x41] = Opcode::new(Operation::EOR_MEM, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0x51] = Opcode::new(Operation::EOR_MEM, AddressingMode::IndirectIndexedY, 2, 5, true);

        opcodes[0xe6] = Opcode::new(Operation::INC, AddressingMode::Zeropage, 2, 5, false);
        opcodes[0xf6] = Opcode::new(Operation::INC, AddressingMode::ZeropageIndexedX, 2, 6, false);
        opcodes[0xee] = Opcode::new(Operation::INC, AddressingMode::Absolute, 3, 6, false);
        opcodes[0xfe] = Opcode::new(Operation::INC, AddressingMode::AbsoluteIndexedX, 3, 7, false);

        opcodes[0xc8] = Opcode::new(Operation::INY, AddressingMode::Implied, 1, 2, false);
        opcodes[0xe8] = Opcode::new(Operation::INX, AddressingMode::Implied, 1, 2, false);

        opcodes[0xa9] = Opcode::new(Operation::LDA_IMM, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xa5] = Opcode::new(Operation::LDA_MEM, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xb5] = Opcode::new(Operation::LDA_MEM, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0xad] = Opcode::new(Operation::LDA_MEM, AddressingMode::Absolute, 3, 4, false);
        opcodes[0xbd] = Opcode::new(Operation::LDA_MEM, AddressingMode::AbsoluteIndexedX, 3, 4, true);
        opcodes[0xb9] = Opcode::new(Operation::LDA_MEM, AddressingMode::AbsoluteIndexedY, 3, 4, true);
        opcodes[0xa1] = Opcode::new(Operation::LDA_MEM, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0xb1] = Opcode::new(Operation::LDA_MEM, AddressingMode::IndirectIndexedY, 2, 5, true);

        opcodes[0xa2] = Opcode::new(Operation::LDX_IMM, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xa6] = Opcode::new(Operation::LDX_MEM, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xb6] = Opcode::new(Operation::LDX_MEM, AddressingMode::ZeropageIndexedY, 2, 4, false);
        opcodes[0xae] = Opcode::new(Operation::LDX_MEM, AddressingMode::Absolute, 3, 4, false);
        opcodes[0xbe] = Opcode::new(Operation::LDX_MEM, AddressingMode::AbsoluteIndexedY, 3, 4, true);

        opcodes[0xa0] = Opcode::new(Operation::LDY_IMM, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xa4] = Opcode::new(Operation::LDY_MEM, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xb4] = Opcode::new(Operation::LDY_MEM, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0xac] = Opcode::new(Operation::LDY_MEM, AddressingMode::Absolute, 3, 4, false);
        opcodes[0xbc] = Opcode::new(Operation::LDY_MEM, AddressingMode::AbsoluteIndexedX, 3, 4, true);

        opcodes[0xea] = Opcode::new(Operation::NOP, AddressingMode::Implied, 1, 2, false);

        opcodes[0x4c] = Opcode::new(Operation::JMP, AddressingMode::Absolute, 3, 3, false);
        opcodes[0x6c] = Opcode::new(Operation::JMP, AddressingMode::Indirect, 3, 5, false);

        opcodes[0xe9] = Opcode::new(Operation::SBC_IMM, AddressingMode::Immediate, 2, 2, false);
        opcodes[0xe5] = Opcode::new(Operation::SBC_MEM, AddressingMode::Zeropage, 2, 3, false);
        opcodes[0xf5] = Opcode::new(Operation::SBC_MEM, AddressingMode::ZeropageIndexedX, 2, 4, false);
        opcodes[0xed] = Opcode::new(Operation::SBC_MEM, AddressingMode::Absolute, 3, 4, false);
        opcodes[0xfd] = Opcode::new(Operation::SBC_MEM, AddressingMode::AbsoluteIndexedX, 3, 4, true);
        opcodes[0xf9] = Opcode::new(Operation::SBC_MEM, AddressingMode::AbsoluteIndexedY, 3, 4, true);
        opcodes[0xe1] = Opcode::new(Operation::SBC_MEM, AddressingMode::IndexedIndirectX, 2, 6, false);
        opcodes[0xf1] = Opcode::new(Operation::SBC_MEM, AddressingMode::IndirectIndexedY, 2, 5, true);

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

        opcodes[0x40] = Opcode::new(Operation::RTI, AddressingMode::Implied, 1, 6, false);

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

    pub fn is_branch(&self) -> bool {
        match self.operation {
            Operation::BCC
            | Operation::BCS
            | Operation::BEQ
            | Operation::BMI
            | Operation::BNE
            | Operation::BPL
            | Operation::BVC
            | Operation::BVS
            => true,

            _ => false
        }
    }
    pub fn will_branch(&self, state: &State) -> bool {
        match self.operation {
            Operation::BCC => _should_bcc(state),
            Operation::BCS => _should_bcs(state),
            Operation::BEQ => _should_beq(state),
            Operation::BMI => _should_bmi(state),
            Operation::BNE => _should_bne(state),
            Operation::BPL => _should_bpl(state),
            Operation::BVC => _should_bvc(state),
            Operation::BVS => _should_bvs(state),

            _ => unreachable!()
        }
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
    pub fn get_cycle_cost(&self, state: &State, bus: &Databus) -> u8 {
        let mut cost = self.opcode.cycles;

        if self.opcode.is_branch() {
            if self.opcode.will_branch(state) {
                let next_pc = state.get_next_pc();
                let branch_pc = state.calculate_relative_pc(self.operand as i8);

                if (next_pc & 0xff00) != (branch_pc & 0xff00) {
                    cost += 2;
                } else {
                    cost += 1;
                }
            }
        } else if self.opcode.page_boundary_penalty
            && self.opcode.mode.crossing_page_boundry(state, bus, self.operand) {
            cost += 1;
        }

        return cost;
    }

    pub fn get_size(&self) -> u8 {
        self.opcode.size
    }

    pub fn format(&self) -> String {
        format!("{} {}", self.opcode.operation.as_str(), self.opcode.mode.format(self.operand))
    }
}

pub fn decode_instruction(prg: &[u8]) -> Instruction {
    let opcode = OPCODE_SET[prg[0] as usize];

    let operand;
    match opcode.size {
        1 => operand = 0,
        2 => operand = prg[1] as u16,
        3 => operand = ((prg[2] as u16) << 8) + prg[1] as u16,
        _ => unreachable!()
    }

    Instruction::new(opcode, operand)
}

pub fn deassemble(rom: &[u8]) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut i: usize = 0;
    let size = rom.len();

    while i < (size - 6) {
        let instruction = decode_instruction(&rom[i..i+3]);
        i += instruction.opcode.size as usize;

        instructions.push(instruction);
    }

    instructions
}

pub static DUMMY_INSTRUCTION: Instruction = Instruction {
    opcode: Opcode {
        operation: Operation::UNKNOWN,
        mode: AddressingMode::Unknown,
        size: 0,
        cycles: 0,
        page_boundary_penalty: false,
    },
    operand: 0,
};

pub static IRQ_INSTRUCTION: Instruction = Instruction {
    opcode: Opcode {
        operation: Operation::INTERNAL_IRQ,
        mode: AddressingMode::Unknown,
        size: 0,
        cycles: 6,
        page_boundary_penalty: false,
    },
    operand: 0,
};

pub static NMI_INSTRUCTION: Instruction = Instruction {
    opcode: Opcode {
        operation: Operation::INTERNAL_NMI,
        mode: AddressingMode::Unknown,
        size: 0,
        cycles: 6,
        page_boundary_penalty: false,
    },
    operand: 0,
};

fn _should_bcc(state: &State) -> bool { !state.get_status_field(state::SR_MASK_CARRY) }

fn _should_bcs(state: &State) -> bool { state.get_status_field(state::SR_MASK_CARRY) }

fn _should_beq(state: &State) -> bool { state.get_status_field(state::SR_MASK_ZERO) }

fn _should_bmi(state: &State) -> bool { state.get_status_field(state::SR_MASK_NEGATIVE) }

fn _should_bne(state: &State) -> bool { !state.get_status_field(state::SR_MASK_ZERO) }

fn _should_bpl(state: &State) -> bool { !state.get_status_field(state::SR_MASK_NEGATIVE) }

fn _should_bvc(state: &State) -> bool { !state.get_status_field(state::SR_MASK_OVERFLOW) }

fn _should_bvs(state: &State) -> bool { state.get_status_field(state::SR_MASK_OVERFLOW) }


fn _adc(state: &mut State, operand: u8) {
    let sum = state.acc.wrapping_add(operand).wrapping_add(state.get_status_field(state::SR_MASK_CARRY) as u8);
    let carry = (operand as u16 + state.acc as u16 + state.get_status_field(state::SR_MASK_CARRY) as u16) > 0xff;
    let overflow = (!(state.acc ^ operand) & (state.acc ^ sum) & 0x80) > 0;
    state.acc = sum;

    state.set_status_field(state::SR_MASK_NEGATIVE, state.acc >= 128);
    state.set_status_field(state::SR_MASK_ZERO, state.acc == 0);
    state.set_status_field(state::SR_MASK_CARRY, carry);
    state.set_status_field(state::SR_MASK_OVERFLOW, overflow);
}

fn _sbc(state: &mut State, operand: u8) {
    _adc(state, !operand);
}

fn _compare(state: &mut State, mem: u8, operand: u8) {
    let sum = operand.wrapping_add(!mem).wrapping_add(1);
    let carry = (operand as u16 + (!mem) as u16) > 0xff;

    state.set_status_field(state::SR_MASK_NEGATIVE, sum >= 128);
    state.set_status_field(state::SR_MASK_ZERO, sum == 0);
    state.set_status_field(state::SR_MASK_CARRY, carry);
}

fn _handle_interrupt(state: &mut State, bus: &mut Databus, interrupt_vector: u16, b_flag: bool) {
    let pc_hi = (state.get_next_pc() >> 8) as u8;
    let pc_lo = (state.get_next_pc() & 0xff) as u8;

    _push_stack(state, bus, pc_hi);
    _push_stack(state, bus, pc_lo);

    let mut status = *state.get_status_ref();
    status.set(state::SR_MASK_BREAK, b_flag);
    _push_stack(state, bus, status.get_as_u8());

    state.set_status_field(state::SR_MASK_INTERRUPT, true);
    state.set_next_pc(bus.read_u16(interrupt_vector));
}

fn _push_stack(state: &mut State, bus: &mut Databus, data: u8) {
    bus.write(cpu::STACK_OFFSET + state.get_sp() as u16, data);
    state.dec_sp();
}

fn _pull_stack(state: &mut State, bus: &Databus) -> u8 {
    state.inc_sp();
    let data = bus.read(cpu::STACK_OFFSET + state.get_sp() as u16);

    data
}


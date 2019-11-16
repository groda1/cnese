use super::state::State;
use super::databus::Databus;

pub type AddressingModeFn = fn(state: &State, bus: &Databus, operand: u16) -> u16;

pub const DO_NOTHING: AddressingModeFn = |_state: &State, _bus: &Databus, _operand: u16| -> u16 { 0 };
pub const IMMEDIATE: AddressingModeFn = |_state: &State, _bus: &Databus, operand: u16| -> u16 { operand };

pub const RELATIVE: AddressingModeFn = |state: &State, _bus: &Databus, operand: u16| -> u16 {
    state.calculate_relative_pc(operand as i8)
};

pub const ABSOLUTE: AddressingModeFn = |_state: &State, _bus: &Databus, operand: u16| -> u16 { operand };
pub const ABSOLUTE_INDEXED_X: AddressingModeFn = |state: &State, _bus: &Databus, operand: u16| -> u16 {
    operand.wrapping_add(state.x as u16)
};

pub const ABSOLUTE_INDEXED_Y: AddressingModeFn = |state: &State, _bus: &Databus, operand: u16| -> u16 {
    operand.wrapping_add(state.y as u16)
};

pub const ZEROPAGE: AddressingModeFn = |_state: &State, _bus: &Databus, operand: u16| -> u16 { operand };

pub const ZEROPAGE_INDEXED_X: AddressingModeFn = |state: &State, _bus: &Databus, operand: u16| -> u16 {
    ((operand as u8).wrapping_add(state.x)) as u16
};

pub const ZEROPAGE_INDEXED_Y: AddressingModeFn = |state: &State, _bus: &Databus, operand: u16| -> u16 {
    ((operand as u8).wrapping_add(state.y)) as u16
};

pub const INDIRECT: AddressingModeFn = |_state: &State, bus: &Databus, operand: u16| -> u16 {
    bus.read_u16(operand)
};

pub const INDEXED_INDIRECT_X: AddressingModeFn = |state: &State, bus: &Databus, operand: u16| -> u16 {
    let addr = ((operand as u8).wrapping_add(state.x)) as u16;

    bus.read_u16(addr)
};

pub const INDIRECT_INDEXED_Y: AddressingModeFn = |state: &State, bus: &Databus, operand: u16| -> u16 {
    bus.read_u16(operand) + state.y as u16
};


#[derive(Clone, Copy)]
pub enum AddressingMode {
    Unknown,
    Implied,
    Immediate,
    Absolute,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    Zeropage,
    ZeropageIndexedX,
    ZeropageIndexedY,
    Relative,
    Accumulator,
    Indirect,
    IndexedIndirectX,
    IndirectIndexedY,
}

impl AddressingMode {
    pub fn eval(&self, state: &State, bus: &Databus, operand: u16) -> u16 {
        self.get_fn()(state, bus, operand)
    }

    pub fn crossing_page_boundry(&self, state: &State, bus: &Databus, operand: u16) -> bool {
        match *self {
            AddressingMode::AbsoluteIndexedX => _crossing_page(operand, state.x),
            AddressingMode::AbsoluteIndexedY => _crossing_page(operand, state.y),
            AddressingMode::IndirectIndexedY => _crossing_page(bus.read_u16(operand), state.y),
            _ => false
        }
    }

    fn get_fn(&self) -> AddressingModeFn {
        match *self {
            AddressingMode::Implied => DO_NOTHING,
            AddressingMode::Immediate => IMMEDIATE,
            AddressingMode::Absolute => ABSOLUTE,
            AddressingMode::AbsoluteIndexedX => ABSOLUTE_INDEXED_X,
            AddressingMode::AbsoluteIndexedY => ABSOLUTE_INDEXED_Y,
            AddressingMode::Zeropage => ZEROPAGE,
            AddressingMode::ZeropageIndexedX => ZEROPAGE_INDEXED_X,
            AddressingMode::ZeropageIndexedY => ZEROPAGE_INDEXED_Y,
            AddressingMode::Relative => RELATIVE,
            AddressingMode::Accumulator => DO_NOTHING,
            AddressingMode::Indirect => INDIRECT,
            AddressingMode::IndexedIndirectX => INDEXED_INDIRECT_X,
            AddressingMode::IndirectIndexedY => INDIRECT_INDEXED_Y,
            AddressingMode::Unknown => DO_NOTHING
        }
    }

    pub fn format(&self, operand: u16) -> String {
        match *self {
            AddressingMode::Implied => format!(""),
            AddressingMode::Immediate => format!("#${:02X}", operand),
            AddressingMode::Absolute => format!("${:04X}", operand),
            AddressingMode::AbsoluteIndexedX => format!("${:04X},X", operand),
            AddressingMode::AbsoluteIndexedY => format!("${:04X},Y", operand),
            AddressingMode::Zeropage => format!("${:02X}", operand),
            AddressingMode::ZeropageIndexedX => format!("${:02X},X", operand),
            AddressingMode::ZeropageIndexedY => format!("${:02X},Y", operand),
            AddressingMode::Relative => format!("${:02X}", operand as i8),
            AddressingMode::Accumulator => format! {"A"},
            AddressingMode::Indirect => format! {"(${:04X})", operand},
            AddressingMode::IndexedIndirectX => format! {"(${:02X},X)", operand},
            AddressingMode::IndirectIndexedY => format! {"(${:02X}),Y", operand},
            _ => format!("##")
        }
    }
}

fn _crossing_page(operand: u16, offset: u8) -> bool {
    (operand & 0xff00) != ((operand + offset as u16) & 0xff00)
}

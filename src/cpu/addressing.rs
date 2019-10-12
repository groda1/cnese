use super::state::State;
use super::databus::Databus;

use enum_map::EnumMap;

pub type AddressingModeFn = fn(state: &State, bus: &Databus, operand: u16) -> u16;


pub const DO_NOTHING: AddressingModeFn = |_state: &State, _bus: &Databus, _operand: u16| -> u16 { 0 };
pub const IMMEDIATE: AddressingModeFn = |_state: &State, _bus: &Databus, operand: u16| -> u16 { operand };
pub const RELATIVE: AddressingModeFn = |_state: &State, _bus: &Databus, operand: u16| -> u16 { operand };

pub const ABSOLUTE: AddressingModeFn = |_state: &State, _bus: &Databus, operand: u16| -> u16 { operand };
pub const ABSOLUTE_INDEXED_X: AddressingModeFn = |state: &State, _bus: &Databus, operand: u16| -> u16 {
    // TODO check for page boundary

    operand + state.x as u16
};

pub const ABSOLUTE_INDEXED_Y: AddressingModeFn = |state: &State, _bus: &Databus, operand: u16| -> u16 {
    // TODO check for page boundary

    operand + state.y as u16
};

//pub const ZEROPAGE: AddressingMode = || -> u16 {
//    32
//};
//
//pub const ZEROPAGE_INDEXED_X: AddressingMode = || -> u16 {
//    32
//};
//
//pub const ZEROPAGE_INDEXED_Y: AddressingMode = || -> u16 {
//    32
//};
//
//pub const INDIRECT: AddressingMode = || -> u16 {
//    let x = 32;
//
//    32 * x;
//};
//
//pub const INDEXED_INDERECT_X: AddressingMode = || -> u16 {
//    32
//};
//
//pub const INDIRECT_INDEXED_Y: AddressingMode = || -> u16 {
//    32
//};


#[derive(Clone, Copy, Enum)]
pub enum AddressingMode {
    UNKNOWN,
    IMPLIED,
    IMMEDIATE,
    ABSOLUTE,
    ABSOLUTE_INDEXED_X,
    ABSOLUTE_INDEXED_Y,
    RELATIVE,
    ACCUMULATOR,
}


impl AddressingMode {
    pub fn eval(&self, state: &State, bus: &Databus, operand: u16) -> u16 {
        self.get_fn()(state, bus, operand)
    }


    fn get_fn(&self) -> AddressingModeFn {
        ADDRESS_MODE_FN_MAP[*self]
    }

    pub fn format(&self, operand: u16) -> String {
        match *self {
            AddressingMode::IMPLIED => format!(""),
            AddressingMode::IMMEDIATE => format!("#${:02x}", operand),
            AddressingMode::ABSOLUTE => format!("${:04x}", operand),
            AddressingMode::ABSOLUTE_INDEXED_X => format!("${:04x},X", operand),
            AddressingMode::ABSOLUTE_INDEXED_Y => format!("${:04x},Y", operand),
            AddressingMode::RELATIVE => format!{"${:02x}", operand},
            AddressingMode::ACCUMULATOR => format!{"A"},
            _ => format!("UNKNOWN")
        }
    }
}

lazy_static! {
    static ref ADDRESS_MODE_FN_MAP: EnumMap<AddressingMode, AddressingModeFn> = {
        let map = enum_map! {
            AddressingMode::IMPLIED => DO_NOTHING,
            AddressingMode::IMMEDIATE => IMMEDIATE,
            AddressingMode::ABSOLUTE => ABSOLUTE,
            AddressingMode::ABSOLUTE_INDEXED_X => ABSOLUTE_INDEXED_X,
            AddressingMode::ABSOLUTE_INDEXED_Y => ABSOLUTE_INDEXED_Y,
            AddressingMode::RELATIVE => RELATIVE,
            AddressingMode::ACCUMULATOR => DO_NOTHING,
            AddressingMode::UNKNOWN => DO_NOTHING,
        };
        map
    };
}

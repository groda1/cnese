use super::state::State;
use super::databus::Databus;

use enum_map::EnumMap;

pub type AddressingModeFn = fn(state: &State, bus: &Databus, operand: u16) -> u16;


pub const UNKNOWN: AddressingModeFn = |_state: &State, _bus: &Databus, _operand: u16| -> u16 { 0 };
pub const IMPLIED: AddressingModeFn = |_state: &State, _bus: &Databus, _operand: u16| -> u16 { 0 };
pub const ABSOLUTE: AddressingModeFn = |_state: &State, _bus: &Databus, operand: u16| -> u16 { operand };
pub const IMMEDIATE: AddressingModeFn = |_state: &State, _bus: &Databus, operand: u16| -> u16 { operand };

//
//pub const RELATIVE: AddressingMode = || -> u16 {
//    32
//};
//

//pub const ACCUMULATOR: AddressingMode = |state : &mut State, bus : &mut Databus| -> u16 {
//    0
//};

//
//pub const ABSOLUTE_INDEXED_X: AddressingMode = || -> u16 {
//    32
//};
//
//pub const ABSOLUTE_INDEXED_Y: AddressingMode = || -> u16 {
//    32
//};
//
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
    IMPLIED,
    IMMEDIATE,
    ABSOLUTE,
    UNKNOWN,
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
            _ => format!("UNKNOWN")
        }
    }
}

lazy_static! {
    static ref ADDRESS_MODE_FN_MAP: EnumMap<AddressingMode, AddressingModeFn> = {
        let map = enum_map! {
            AddressingMode::IMPLIED => IMPLIED,
            AddressingMode::IMMEDIATE => IMMEDIATE,
            AddressingMode::ABSOLUTE => ABSOLUTE,
            AddressingMode::UNKNOWN => UNKNOWN
        };
        map
    };
}

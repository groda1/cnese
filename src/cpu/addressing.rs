use super::state::State;
use super::databus::Databus;

pub type AddressingMode = fn(state: &mut State, bus: &mut Databus) -> u16;

//pub const ACCUMULATOR: AddressingMode = |state : &mut State, bus : &mut Databus| -> u16 {
//    0
//};


//pub const IMMEDIATE: AddressingMode = || -> u16 {
//    32
//};
//

pub const IMPLIED: AddressingMode = |state: &mut State, bus: &mut Databus| -> u16 {
    0
};
//
//pub const RELATIVE: AddressingMode = || -> u16 {
//    32
//};
//

pub const ABSOLUTE: AddressingMode = |state: &mut State, bus: &mut Databus| -> u16 {
    let lo = bus.read(state.get_pc());
    state.offset_pc(1);
    let hi = bus.read(state.get_pc());
    state.offset_pc(1);

    ((hi as u16) << 8) + lo as u16
};

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



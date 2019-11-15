#[macro_use]
extern crate lazy_static;

mod cpu;

mod nes;
use nes::nes::NES;

mod util;
mod ui;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let rom = util::file::read_file(path);

    let mut nes = NES::new();
    nes.load_rom(rom.as_slice());

    nes.reset();

    let _result = ui::main::run(&mut nes).unwrap();
}


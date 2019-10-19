#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate enum_map;

mod cpu;
use cpu::cpu::Cpu;
use cpu::databus::Databus;
use cpu::instruction;

mod util;
mod ui;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args {:?}", args);

    let path = &args[1];

    let rom = util::file::read_file(path);
    let mut cpu = Cpu::new();
    let mut bus = Databus::new();
    bus.load_rom(rom);

    ui::main::run(&mut cpu, &mut bus);
}


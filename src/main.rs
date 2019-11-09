#[macro_use]
extern crate lazy_static;

mod cpu;
use cpu::cpu::Cpu;
use cpu::databus::Databus;
use cpu::instruction;

mod util;
mod ui;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    let path = &args[1];

    let a:i8 = 120;
    let b:i8 = 40;

    let (result, v) = a.overflowing_add(b);

    println!("{} {}", result, v);

    let rom = util::file::read_file(path);
    let mut cpu = Cpu::new();
    let mut bus = Databus::new();
    bus.load_rom(rom);

    let _result = ui::main::run(&mut cpu, &mut bus).unwrap();
}


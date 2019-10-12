#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate enum_map;

mod cpu;
use cpu::cpu::Cpu;
use cpu::databus::Databus;
use cpu::instruction;

mod util;




fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("args {:?}", args);

    let path = &args[1];

    let rom = util::file::derp(path);


    println!("mem {:x?}", rom);

    let mut cpu = Cpu::new();
    let mut bus = Databus::new();

    let deassembled_instructions = instruction::deassemble(rom.as_slice());


    for a in deassembled_instructions {
        println!("håå {}", a.format());
    }


    bus.load_rom(rom);

    loop {
        cpu.tick(&mut bus);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

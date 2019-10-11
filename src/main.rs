#[macro_use]
extern crate lazy_static;

mod cpu;
//use cpu::cpu_6502::State;

use cpu::cpu_6502::Cpu;
use crate::cpu::databus::Databus;

//use cpu::addressing;

mod util;




fn main() {
    let args: Vec<String> = std::env::args().collect();

    println!("args {:?}", args);

    let path = &args[1];


    let rom = util::file::derp(path);


    println!("mem {:x?}", rom);

    let mut cpu = Cpu::new();
    let mut bus = Databus::new();

    bus.load_rom(rom);


    loop {
        cpu.tick(&mut bus);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }



   // println!("LOL {}", x(&mut foo2));



}

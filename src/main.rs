#[macro_use]
extern crate lazy_static;

mod cpu;
mod nes;
mod util;
mod gfx;
mod ppu;

use nes::nes::NES;
use nes::ines;
use nes::cartridge::cartridge;

fn main() {
    println!("CNESE");

    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];


    let mut cartridge = Option::None;

    if path.ends_with("bin") {
        let rom = util::file::read_file(path);

        cartridge = Option::Some(cartridge::create_cartridge_from_raw(&rom)
            .map_err(|e| println!("Failed to parse RAW image {}", e))
            .unwrap());
    } else if path.ends_with("nes") {
        cartridge = Option::Some(ines::open_ines(path)
            .map_err(|e| println!("Failed to parse iNES file {}", e))
            .unwrap());
    }

    match cartridge {
        None => {
            println!("No valid cartridge. Exiting..");
            return;
        },
        Some(c) => {
            let mut nes = NES::new(c);
            nes.reset();
            let _result = gfx::main::run(&mut nes).unwrap();
        }
    }


}


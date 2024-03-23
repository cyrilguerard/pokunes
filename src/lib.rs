use crate::{nes::Nes, rom::Rom};

pub mod cpu;
pub mod nes;
pub mod rom;

pub fn emulate(rom_file: &str) {
    let rom = Rom::load_from_file(rom_file.to_owned()).unwrap();

    let nes = Nes::new();
    let nes = nes::insert_rom(nes, rom);
    let nes = nes::power_on(nes).unwrap();

    let mut nes = nes;
    loop {
        nes = nes::next_cycle(nes).unwrap();
        // let interrupt = cpu.tick().unwrap();
        // if interrupt {
        //     if cpu.valid() {
        //         println!("STATUS: PASSED");
        //     } else {
        //         println!("STATUS: FAILED");
        //     }
        //     return;
        // }
    }
}

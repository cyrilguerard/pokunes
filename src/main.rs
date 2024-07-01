// https://github.com/christopherpow/nes-test-roms
// CHR-ROM: Character ROM
// PRG ROM: Program ROM

use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: pokunes <rom_path>");
        exit(-1);
    }

    let rom = &args[1];
    pokunes::emulate(rom);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

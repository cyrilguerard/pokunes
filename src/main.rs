extern crate lazy_static;

// https://github.com/christopherpow/nes-test-roms
// CHR-ROM: Character ROM
// PRG ROM: Program ROM

fn main() {
    pokunes::emulate("/home/cyril/workspace/rust/pokunes/roms/01-basics.nes");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

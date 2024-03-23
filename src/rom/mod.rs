use std::{fmt::Debug, fs, io};

const HEADER_SIZE: usize = 16;
const HEADER_PRG_ROM: usize = 4;
const HEADER_FLAGS_6: usize = 6;

#[derive(Clone)]
pub struct Rom {
    pub prg_rom_size: usize,
    pub prg_rom: Vec<u8>,
    pub trainer: bool,
}

impl Into<Vec<u8>> for Rom {
    fn into(self) -> Vec<u8> {
        self.prg_rom
    }
}

impl Debug for Rom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rom")
            .field("prg_rom_size", &self.prg_rom_size)
            .field("trainer", &self.trainer)
            .finish()
    }
}

struct Flags6 {
    trainer: bool,
}

impl Rom {
    pub fn load_from_file(filepath: String) -> Result<Self, io::Error> {
        let data = fs::read(filepath)?;
        let (data, prg_rom_size) = Self::read_prg_rom_size(data);
        let (data, flag6) = Self::read_flags_6(data);
        let prg_rom_start_idx = HEADER_SIZE + if flag6.trainer { 512 } else { 0 };
        let prg_rom_end_idx = prg_rom_start_idx + prg_rom_size - 1;
        let rom = Self {
            prg_rom_size,
            prg_rom: Vec::from(&data[prg_rom_start_idx..prg_rom_end_idx]),
            trainer: flag6.trainer,
        };
        println!("[ROM] LOADED: {:?}", rom);
        Ok(rom)
    }

    fn read_prg_rom_size(data: Vec<u8>) -> (Vec<u8>, usize) {
        let prg_rom_size = usize::from(data[HEADER_PRG_ROM]) * 16 * 1024;
        (data, prg_rom_size)
    }

    fn read_flags_6(data: Vec<u8>) -> (Vec<u8>, Flags6) {
        let flags = data[HEADER_FLAGS_6];
        let trainer = flags & 0b00000100 != 0;
        (data, Flags6 { trainer })
    }
}

// println!("OK: {}", cpu.valid());

// let data = fs::read().unwrap();
// println!("Vec Size: {}", data.len());

// // header
// print!("Constant NES: ");
// for i in 0..=3 {
//     print!("{:02X?} ", data[i]);
// }
// println!();

// println!("PRG-ROM size LSB: {}KB", data[4] * 16);
// println!("CHR-ROM size LSB: {}KB", data[5] * 8);  // 0 means CHR RAM
// println!("Flags 6 - Mapper, mirroring, battery, trainer: {:#010b}", data[6]);
// println!("Flags 7 - Mapper, VS/Playchoice, NES 2.0: {:#010b}", data[7]);
// println!("Flags 8 - PRG-RAM size (rarely used extension): {:#010b}", data[8]);
// println!("Flags 9 - TV system (rarely used extension): {:#010b}", data[9]);
// println!("Flags 10 - TV system, PRG-RAM presence (unofficial, rarely used extension): {:#010b}", data[9]);

// print!("Unused padding: ");
// for i in 11..=15 {
//     print!("{:02X?} ", data[i]);
// }
// println!();

// for i in 16..data.len() {
//     if data[i] != 255 {
//         print!("{:02X?} ({}) ", data[i], i);
//     }
// }
// println!();

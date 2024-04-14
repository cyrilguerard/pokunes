use crate::PokunesError;
use std::fmt::Debug;

pub enum MemoryErrorReason {
    InvalidRomSize(String, usize),
    InvalidAddress(u16),
}

impl Debug for MemoryErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryErrorReason::InvalidRomSize(msg, size) => f
                .debug_tuple("InvalidRomSize")
                .field(&format!("{}", msg).to_uppercase())
                .field(&format!("{}ko", size / 1024).to_uppercase())
                .finish(),
            MemoryErrorReason::InvalidAddress(address) => f
                .debug_tuple("InvalidAddress")
                .field(&format!("{:02x}", address).to_uppercase())
                .finish(),
        }
    }
}

#[derive(Default)]
pub struct Memory {
    prg_rom: Vec<u8>,
}

pub fn load_rom(mem: Memory, prg_rom: Vec<u8>) -> Result<Memory, PokunesError> {
    match prg_rom.len() {
        0x4000 | 0x8000 => Ok(Memory { prg_rom, ..mem }),
        _ => Err(PokunesError::MemoryError(
            MemoryErrorReason::InvalidRomSize(
                "Only 16ko or 32ko roms are supported".to_string(),
                prg_rom.len(),
            ),
        )),
    }
}

pub fn read_u8(mem: Memory, address: u16) -> Result<(Memory, u8), PokunesError> {
    match address {
        0x8000..=0xFFFF => {
            let prg_rom_addr = (usize::from(address) - 0x8000) % mem.prg_rom.len();
            let value = mem.prg_rom[prg_rom_addr as usize];
            Ok((mem, value))
        }
        _ => Err(PokunesError::MemoryError(
            MemoryErrorReason::InvalidAddress(address),
        )),
    }
}

pub fn read_u16(mem: Memory, address: u16) -> Result<(Memory, u16), PokunesError> {
    let (mem, first) = read_u8(mem, address)?;
    let (mem, second) = read_u8(mem, address + 1)?;
    Ok((mem, u16::from_le_bytes([first, second])))
}

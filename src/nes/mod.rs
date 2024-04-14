use crate::{
    cpu::{self, Cpu},
    mem::{self, Memory},
    rom::Rom,
    PokunesError,
};

pub enum PowerStatus {
    On,
    Off,
}

#[derive(Debug)]
pub enum NesErrorReason {
    NoRomInserted,
    NotPowerOn,
}

pub struct Nes {
    power_status: PowerStatus,
    cpu: Cpu,
    mem: Memory,
    rom: Option<Rom>,
}

impl Nes {
    pub fn new() -> Self {
        Self {
            power_status: PowerStatus::Off,
            cpu: Default::default(),
            mem: Default::default(),
            rom: None,
        }
    }
}

pub fn insert_rom(nes: Nes, rom: Rom) -> Result<Nes, PokunesError> {
    println!("[NES] Inserting ROM: {:}", rom.name);

    let prg_rom: Vec<u8> = rom.clone().into();
    let mem = mem::load_rom(nes.mem, prg_rom)?;
    let (cpu, mem) = cpu::reset(nes.cpu, mem)?;

    println!("[NES] ROM inserted");
    Ok(Nes {
        cpu,
        mem,
        rom: Some(rom),
        ..nes
    })
}

pub fn power_on(nes: Nes) -> Result<Nes, PokunesError> {
    println!("[NES] Powering up");
    nes.rom
        .ok_or(PokunesError::NesError(NesErrorReason::NoRomInserted))
        .map(|rom| {
            println!("[NES] Powered up");
            Nes {
                power_status: PowerStatus::On,
                rom: Some(rom),
                ..nes
            }
        })
}

pub fn next_cycle(nes: Nes) -> Result<Nes, PokunesError> {
    if let PowerStatus::Off = nes.power_status {
        Err(PokunesError::NesError(NesErrorReason::NotPowerOn))
    } else {
        let (cpu, mem) = cpu::tick(nes.cpu, nes.mem)?;
        Ok(Nes { cpu, mem, ..nes })
    }
}

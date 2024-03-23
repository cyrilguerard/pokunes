use crate::{
    cpu::{self, Cpu, CpuError},
    rom::Rom,
};

pub enum PowerStatus {
    On,
    Off,
}

#[derive(Debug)]
pub enum NesError {
    NoRomInserted,
    NotPowerOn,
    CpuFailure(CpuError),
}

pub struct Nes {
    power_status: PowerStatus,
    cpu: Cpu,
    rom: Option<Rom>,
}

impl Nes {
    pub fn new() -> Self {
        Self {
            power_status: PowerStatus::Off,
            cpu: Default::default(),
            rom: None,
        }
    }
}

pub fn insert_rom(nes: Nes, rom: Rom) -> Nes {
    Nes {
        cpu: cpu::load_program(Cpu::default(), rom.clone().into()),
        rom: Some(rom),
        ..nes
    }
}

pub fn power_on(nes: Nes) -> Result<Nes, NesError> {
    nes.rom.ok_or(NesError::NoRomInserted).map(|rom| Nes {
        power_status: PowerStatus::On,
        rom: Some(rom),
        ..nes
    })
}

pub fn next_cycle(nes: Nes) -> Result<Nes, NesError> {
    if let PowerStatus::Off = nes.power_status {
        Err(NesError::NotPowerOn)
    } else {
        let cpu = cpu::tick(nes.cpu).map_err(|err| NesError::CpuFailure(err))?;
        Ok(Nes { cpu, ..nes })
    }
}

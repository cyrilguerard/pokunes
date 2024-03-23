use std::collections::HashMap;

use crate::cpu::{self, AddressingMode, Cpu, CpuError};
use lazy_static::lazy_static;

pub struct CpuOp {
    pub code: u8,
    pub apply: fn(Cpu) -> Result<Cpu, CpuError>,
}

const SUPPORTED_CPU_OPS: [CpuOp; 0] = [];

lazy_static! {
    pub static ref CPU_OPS: HashMap<u8, CpuOp> = {
        SUPPORTED_CPU_OPS
            .into_iter()
            .map(|op| (op.code, op))
            .collect()
    };
}

pub fn sei(cpu: Cpu) -> Result<Cpu, CpuError> {
    Ok(Cpu {
        processor_status: cpu::update_processor_status(
            cpu.processor_status,
            cpu::ProcessorStatusFlags::InterruptDisable,
            true,
        ),
        ..cpu
    })
}

// pub fn jmp(cpu: Cpu, addressing_mode: AddressingMode) -> Result<Cpu, CpuError> {
//     if let AddressingMode::Absolute = addressing_mode {
//         let (prg_rom, jmp_address) = cpu::read_prg_rom_u16(cpu.prg_rom, cpu.program_counter);
//         println!("{}", jmp_address);
//         Ok(Cpu {
//             program_counter: jmp_address,
//             prg_rom,
//             ..cpu
//         })
//     } else {
//         Err(CpuError::UnsupportedAddressingMode(
//             "JMP".to_owned(),
//             addressing_mode,
//         ))
//     }
// }

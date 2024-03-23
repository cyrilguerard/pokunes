use crate::cpu::{self, Cpu, CpuError};

pub type CpuOpFn = fn(Cpu) -> Result<Cpu, CpuError>;

#[derive(Debug)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    Relative,
    Absolute,
    AbsoluteX,
}

pub struct CpuOp {
    pub code: u8,
    pub accronym: &'static str,
    pub apply: CpuOpFn,
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

pub fn create_jmp(addressing_mode: AddressingMode) -> Result<CpuOpFn, CpuError> {
    Ok(match addressing_mode {
        AddressingMode::Absolute => |cpu: Cpu| {
            let (cpu, jump_address) = cpu::read_next_prg_rom_u16(cpu);
            jmp(cpu, jump_address)
        },
        _ => Err(CpuError::UnsupportedAddressingMode(
            "JMP".to_owned(),
            addressing_mode,
        ))?,
    })
}

pub fn jmp(cpu: Cpu, jump_address: u16) -> Result<Cpu, CpuError> {
    Ok(cpu::move_program_counter(cpu, jump_address))
}

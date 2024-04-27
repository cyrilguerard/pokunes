use crate::{
    cpu::{self, Cpu},
    mem::{self, Memory},
    PokunesError,
};

use super::CpuErrorReason;

pub type CpuOpFn = fn(Cpu, Memory, AddressingMode) -> Result<(Cpu, Memory), PokunesError>;

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
    pub addressing_mode: AddressingMode,
}

pub fn sei(
    cpu: Cpu,
    mem: Memory,
    addressing_mode: AddressingMode,
) -> Result<(Cpu, Memory), PokunesError> {
    let (cpu, mem, _) = read_parameter(cpu, mem, addressing_mode)?;
    let cpu = cpu::update_processor_status(cpu, cpu::ProcessorStatusFlags::InterruptDisable, true);
    Ok((cpu, mem))
}

pub fn jmp(
    cpu: Cpu,
    mem: Memory,
    addressing_mode: AddressingMode,
) -> Result<(Cpu, Memory), PokunesError> {
    let (cpu, mem, jump_address) = read_parameter(cpu, mem, addressing_mode)?;
    let cpu = cpu::move_program_counter(cpu, jump_address.unwrap());
    Ok((cpu, mem))
}

fn read_parameter(
    cpu: Cpu,
    mem: Memory,
    addressing_mode: AddressingMode,
) -> Result<(Cpu, Memory, Option<u16>), PokunesError> {
    match addressing_mode {
        AddressingMode::Implied => Ok((cpu, mem, None)),
        AddressingMode::Absolute => {
            let (mem, parameter) = mem::read_u16(mem, cpu.program_counter)?;
            let program_counter = cpu.program_counter + 1;
            let cpu = cpu::move_program_counter(cpu, program_counter);
            Ok((cpu, mem, Some(parameter)))
        }
        _ => Err(PokunesError::CpuError(
            CpuErrorReason::UnsupportedAddressingMode("JMP".to_owned(), addressing_mode),
        ))?,
    }
}

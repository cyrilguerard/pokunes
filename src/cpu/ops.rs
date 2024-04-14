use crate::{
    cpu::{self, Cpu},
    PokunesError,
};

pub type CpuOpFn = fn(Cpu) -> Result<Cpu, PokunesError>;

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

pub fn sei(cpu: Cpu) -> Result<Cpu, PokunesError> {
    let cpu = cpu::update_processor_status(cpu, cpu::ProcessorStatusFlags::InterruptDisable, true);
    let program_counter = cpu.program_counter + 1;
    Ok(cpu::move_program_counter(cpu, program_counter))
}

// pub fn jmp(cpu: Cpu, jump_address: u16) -> Result<Cpu, PokunesError> {
//     Ok(cpu::move_program_counter(cpu, jump_address))
// }

// pub fn create_jmp(addressing_mode: AddressingMode) -> Result<CpuOpFn, PokunesError> {
//     Ok(match addressing_mode {
//         AddressingMode::Absolute => |cpu: Cpu| {
//             let (mem, jump_address) = mem::read_u16(cpu.memory, cpu.program_counter + 1)?;
//             jmp(Cpu { memory: mem, ..cpu }, jump_address)
//         },
//         _ => Err(PokunesError::CpuError(
//             CpuErrorReason::UnsupportedAddressingMode("JMP".to_owned(), addressing_mode),
//         ))?,
//     })
// }

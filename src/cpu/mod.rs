use bitflags::bitflags;
use std::fmt::Debug;

use crate::{
    mem::{self, Memory},
    PokunesError,
};

use self::ops::{AddressingMode, CpuOp};

pub mod ops;

pub enum CpuErrorReason {
    UnsupportedOperation(u8),
    UnsupportedAddressingMode(String, AddressingMode),
}

impl Debug for CpuErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuErrorReason::UnsupportedOperation(op_code) => f
                .debug_tuple("UnsupportedOperation")
                .field(&format!("${:02x}", op_code).to_uppercase())
                .finish(),
            CpuErrorReason::UnsupportedAddressingMode(op_name, addressing_mode) => f
                .debug_tuple("UnsupportedAddressingMode")
                .field(&format!("{:?}", op_name))
                .field(&format!("{:?}", addressing_mode))
                .finish(),
        }
    }
}

const RESET_VECTOR_ADDRESS: u16 = 0xFFFC;

bitflags! {
    #[derive(Debug, Default)]
    pub struct ProcessorStatusFlags: u8 {
        const CarryFlag         = 0b10000000;
        const ZeroFlag          = 0b01000000;
        const InterruptDisable  = 0b00100000;
        const DecimalMode       = 0b00010000;
        const BreakCommand      = 0b00001000;
        const OverflowFlag      = 0b00000100;
        const NegativeFlag      = 0b00000010;
    }
}

#[derive(Default)]
pub struct Cpu {
    program_counter: u16,
    processor_status: ProcessorStatusFlags,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            program_counter: 0,
            processor_status: ProcessorStatusFlags::empty(),
        }
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field(
                "program_counter",
                &format!("{:02x}", &self.program_counter).to_uppercase(),
            )
            .field("processor_status", &self.processor_status)
            .finish()
    }
}

pub fn reset(_cpu: Cpu, mem: Memory) -> Result<(Cpu, Memory), PokunesError> {
    println!("[CPU] Reseting...");
    let (mem, program_start_addr) = mem::read_u16(mem, RESET_VECTOR_ADDRESS)?;
    let cpu = move_program_counter(Cpu::new(), program_start_addr);
    println!("[CPU] Reset");
    Ok((cpu, mem))
}

pub fn tick(cpu: Cpu, mem: Memory) -> Result<(Cpu, Memory), PokunesError> {
    println!("[CPU] Tick...");
    read_next_op(cpu, mem).and_then(|(cpu, mem, cpu_op)| {
        println!(
            "[CPU] =========================================================== {}[{:02x}] ===========================================================",
            cpu_op.accronym, cpu_op.code
        );
        println!("[CPU] <<<<: {:?}", cpu);
        let cpu = (cpu_op.apply)(cpu)?;
        println!("[CPU] >>>> : {:?}", cpu);
        println!("[CPU] ===============================================================================================================================");

        println!("[CPU] Tick done");
        Ok((cpu, mem))
    })
}

fn read_next_op(cpu: Cpu, mem: Memory) -> Result<(Cpu, Memory, CpuOp), PokunesError> {
    println!(
        "[CPU] Read next op at: {}",
        &format!("{:02x}", cpu.program_counter).to_uppercase()
    );
    let (mem, op_code) = mem::read_u8(mem, cpu.program_counter)?;
    let op = match op_code {
        0x78 => CpuOp {
            code: op_code,
            accronym: "SEI",
            apply: ops::sei,
        },
        // 0x4C => CpuOp {
        //     code: op_code,
        //     accronym: "JMP",
        //     apply: ops::create_jmp(AddressingMode::Absolute)?,
        // },
        // 0x8D => CpuOp {
        //     code: op_code,
        //     accronym: "STA",
        //     apply: |cpu| {
        //         let (memory, _) = mem::read_u16(memory, cpu.program_counter)?;
        //         Ok(cpu)
        //     },
        // },
        // 0xA9 => CpuOp {
        //     code: op_code,
        //     accronym: "LDA",
        //     apply: |cpu| {
        //         let (memory, _) = mem::read_u16(memory, cpu.program_counter)?;
        //         Ok(cpu)
        //     },
        // },
        _ => Err(PokunesError::CpuError(
            CpuErrorReason::UnsupportedOperation(op_code),
        ))?,
    };
    Ok((cpu, mem, op))
}

fn move_program_counter(cpu: Cpu, address: u16) -> Cpu {
    println!(
        "[CPU] Move program_counter: {}",
        (format!("{:04x}", address)).to_uppercase()
    );
    Cpu {
        program_counter: address,
        ..cpu
    }
}

fn update_processor_status(cpu: Cpu, flag: ProcessorStatusFlags, enable: bool) -> Cpu {
    println!("[CPU] Set processor_status: {:?}={:?}", flag, enable);
    let processor_status = if enable {
        cpu.processor_status | flag
    } else {
        cpu.processor_status - flag
    };
    Cpu {
        processor_status,
        ..cpu
    }
}

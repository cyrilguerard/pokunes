use bitflags::bitflags;
use std::fmt::Debug;

use self::ops::{AddressingMode, CpuOp};

pub mod ops;

pub enum CpuError {
    UnsupportedOperation(u8),
    UnsupportedAddressingMode(String, AddressingMode),
}

impl Debug for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::UnsupportedOperation(op_code) => f
                .debug_tuple("UnsupportedOperation")
                .field(&format!("${:02x}", op_code).to_uppercase())
                .finish(),
            CpuError::UnsupportedAddressingMode(op_name, addressing_mode) => f
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
    prg_rom: Vec<u8>,
    program_counter: u16,
    processor_status: ProcessorStatusFlags,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            prg_rom: vec![],
            program_counter: 0,
            processor_status: ProcessorStatusFlags::empty(),
        }
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field(
                "prg_rom",
                &format!(
                    "[{:02x} {:02x} ...]",
                    self.prg_rom[to_address(self.program_counter)],
                    self.prg_rom[to_address(self.program_counter + 1)]
                )
                .to_uppercase(),
            )
            .field("program_counter", &self.program_counter)
            .field("processor_status", &self.processor_status)
            .finish()
    }
}

pub fn load_program(cpu: Cpu, prg_rom: Vec<u8>) -> Cpu {
    let (cpu, first_op_address) = read_next_prg_rom_u16(Cpu {
        prg_rom,
        processor_status: ProcessorStatusFlags::empty(),
        program_counter: RESET_VECTOR_ADDRESS,
        ..cpu
    });
    move_program_counter(cpu, first_op_address)
}

pub fn tick(cpu: Cpu) -> Result<Cpu, CpuError> {
    read_next_op(cpu).and_then(|(cpu, cpu_op)| {
        println!(
            "[CPU] =========================================================== {}[{:02x}] ===========================================================",
            cpu_op.accronym, cpu_op.code
        );
        println!("[CPU] BEFORE: {:?}", cpu);
        let cpu = (cpu_op.apply)(cpu)?;
        println!("[CPU] AFTER : {:?}", cpu);
        println!("[CPU] ===============================================================================================================================");
        Ok(cpu)
    })
}

fn read_next_op(cpu: Cpu) -> Result<(Cpu, CpuOp), CpuError> {
    let (cpu, op_code) = read_next_prg_rom(cpu);
    let op = match op_code {
        0x78 => CpuOp {
            code: op_code,
            accronym: "SEI",
            apply: ops::sei,
        },
        0x4C => CpuOp {
            code: op_code,
            accronym: "JMP",
            apply: ops::create_jmp(AddressingMode::Absolute)?,
        },
        _ => Err(CpuError::UnsupportedOperation(op_code))?,
    };
    Ok((cpu, op))
}

fn to_address(program_counter: u16) -> usize {
    ((program_counter - 0x8000) % 0xC000).into()
}

fn read_next_prg_rom(cpu: Cpu) -> (Cpu, u8) {
    let address = to_address(cpu.program_counter);
    let value = cpu.prg_rom[address];
    let program_counter = cpu.program_counter.clone() + 1;
    let cpu = move_program_counter(cpu, program_counter);
    (cpu, value)
}

fn read_next_prg_rom_u16(cpu: Cpu) -> (Cpu, u16) {
    let (cpu, first) = read_next_prg_rom(cpu);
    let (cpu, second) = read_next_prg_rom(cpu);
    (cpu, u16::from_le_bytes([first, second]))
}

fn move_program_counter(cpu: Cpu, address: u16) -> Cpu {
    Cpu {
        program_counter: address,
        ..cpu
    }
}

fn update_processor_status(
    processor_status: ProcessorStatusFlags,
    flag: ProcessorStatusFlags,
    enable: bool,
) -> ProcessorStatusFlags {
    if enable {
        processor_status | flag
    } else {
        processor_status - flag
    }
}

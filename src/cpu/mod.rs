use bitflags::bitflags;
// use lazy_static::lazy_static;
// use std::collections::HashMap;
use std::fmt::Debug;

use self::ops::CpuOp;

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

pub fn load_program(cpu: Cpu, prg_rom: Vec<u8>) -> Cpu {
    move_program_counter(
        Cpu {
            prg_rom,
            processor_status: ProcessorStatusFlags::empty(),
            ..cpu
        },
        RESET_VECTOR_ADDRESS,
    )
}

pub fn tick(cpu: Cpu) -> Result<Cpu, CpuError> {
    read_next_op(cpu).and_then(|(cpu, cpu_op)| (cpu_op.apply)(cpu))
}

fn read_next_op(cpu: Cpu) -> Result<(Cpu, CpuOp), CpuError> {
    let (cpu, op_code) = read_next_prg_rom(cpu);
    let op = match op_code {
        78 => CpuOp {
            code: 78,
            apply: ops::sei,
        },
        _ => Err(CpuError::UnsupportedOperation(op_code))?,
    };
    Ok((cpu, op))
}

fn read_next_prg_rom(cpu: Cpu) -> (Cpu, u8) {
    let address = (cpu.program_counter - 0x8000) % 0xC000;
    let value = cpu.prg_rom[address as usize];
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

// pub fn load_program(&mut self, prg_rom: Vec<u8>) {
//     self.prg_rom = prg_rom;
//     self.program_counter = self.read_prg_rom_u16(RESET_VECTOR_ADDRESS);
// }

// pub fn tick(&mut self) -> Result<bool, CpuError> {
//     self.execute_next()
// }

// pub fn valid(&self) -> bool {
//     self.read_prg_rom(0xC000) == 0u8
// }

// fn update_processor_status(&mut self, status_flag: ProcessorStatusFlags, enable: bool) {
//     if enable {
//         self.processor_status |= status_flag;
//     } else {
//         self.processor_status -= status_flag;
//     }
// }

// fn op_sei(&mut self) {
//     self.update_processor_status(ProcessorStatusFlags::InterruptDisable, true);
// }

// fn op_jmp(&mut self, addressing_mode: &AddressingMode) {
//     if let AddressingMode::Absolute = addressing_mode {
//         let jmp_address = self.read_prg_rom_u16(self.program_counter);
//         println!("{}", jmp_address);
//         self.program_counter = jmp_address;
//     } else {
//         panic!("BOOM")
//     }
// }

// fn op_ldx(&mut self, addressing_mode: &AddressingMode) {
//     if let AddressingMode::Immediate = addressing_mode {
//         self.read_prg_rom_u16(self.program_counter);
//         self.program_counter += 2;
//     } else {
//         panic!("BOOM")
//     }
// }

// fn execute_next(&mut self) -> Result<bool, CpuError> {
//     let op_code = self.read_prg_rom(self.program_counter);
//     let next_byte = self.read_prg_rom(self.program_counter + 1);
//     let next_next_byte = self.read_prg_rom(self.program_counter + 2);
//     println!("{:02x} {:02x} {:02x}", op_code, next_byte, next_next_byte);

//     let op = CPU_OPS
//         .get(&op_code)
//         .ok_or(CpuError::UnsupportedOperation(op_code))?;
//     self.program_counter += 1 as u16;

//     match op_code {
//         0x78 => self.op_sei(),
//         0x4C => self.op_jmp(&op.addressing_mode),
//         _ => (),
//     }

//     println!("{:?}", op);
//     Ok(op_code == 0x00)
// match op_code {
//     0x4C => {
//         println!("JMP");
//         self.program_counter += 3;
//     },
//     0x78 => {
//         println!("SEI");
//         self.program_counter += 1;
//     },
//     0x8E => {
//         println!("STX");
//         self.program_counter += 3;
//     },
//     0x9A => {
//         println!("TXS");
//         self.program_counter += 1;
//     },
//     0xA2 => {
//         println!("LDX");
//         self.program_counter += 2;
//     },
//     0xD8 => {
//         println!("CLD");
//         self.program_counter += 1;
//     },
//     0xE8 => {
//         println!("CLD");
//         self.program_counter += 1;
//     },
//     _ => panic!("[CPU] unsupported operation: {:02x}", op_code),
// }
// }

// fn read_prg_rom(&self, address: u16) -> u8 {
//     let address = (address - 0x8000) % 0xC000;
//     self.prg_rom[address as usize]
// }

// fn read_prg_rom_u16(&self, address: u16) -> u16 {
//     let first = self.read_prg_rom(address);
//     let second = self.read_prg_rom(address + 1);
//     u16::from_le_bytes([first, second])
// }
// }

// const SUPPORTED_CPU_OPS: [CpuOp; 2] = [
//     CpuOp {
//         code: 0x78,
//         accronym: "SEI", // Set Interrupt Disable: Set the interrupt disable flag to one.
//         bytes: 1,
//         cycles: 2,
//         addressing_mode: AddressingMode::Implied,
//     },
//     CpuOp {
//         code: 0x4C,
//         accronym: "JMP", // Jump: Sets the program counter to the address specified by the operand.
//         bytes: 3,
//         cycles: 3,
//         addressing_mode: AddressingMode::Absolute,
//     },
// CpuOp {
//     code: 0xD8,
//     accronym: "CLD", // Clear Decimal Mode: Sets the decimal mode flag to zero.
//     bytes: 1,
//     cycles: 2,
//     addressing_mode: AddressingMode::Implied,
// },
// CpuOp {
//     code: 0xA2,
//     accronym: "LDX", // Load X Register: Loads a byte of memory into the X register setting the zero and negative flags as appropriate.
//     bytes: 2,
//     cycles: 2,
//     addressing_mode: AddressingMode::Immediate,
// },
// CpuOp {
//     code: 0x9A,
//     accronym: "TXS", // Transfer X to Stack Pointer: Copies the current contents of the X register into the stack register.
//     bytes: 1,
//     cycles: 2,
//     addressing_mode: AddressingMode::Implied,
// },
// CpuOp {
//     code: 0xE8,
//     accronym: "INX", // Increment X Register: Adds one to the X register setting the zero and negative flags as appropriate.
//     bytes: 1,
//     cycles: 2,
//     addressing_mode: AddressingMode::Implied,
// },
// CpuOp {
//     code: 0x8E,
//     accronym: "STX", // Store X Register: Stores the contents of the X register into memory.
//     bytes: 3,
//     cycles: 4,
//     addressing_mode: AddressingMode::Absolute,
// },
// CpuOp {
//     code: 0x20,
//     accronym: "JSR", // Jump to Subroutine: The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.
//     bytes: 3,
//     cycles: 6,
//     addressing_mode: AddressingMode::Absolute,
// },
// CpuOp {
//     code: 0x48,
//     accronym: "PHA", // Push Accumulator: Pushes a copy of the accumulator on to the stack.
//     bytes: 1,
//     cycles: 3,
//     addressing_mode: AddressingMode::Implied,
// },
// CpuOp {
//     code: 0xA9,
//     accronym: "LDA", // Load Accumulator: Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
//     bytes: 2,
//     cycles: 2,
//     addressing_mode: AddressingMode::Immediate,
// },
// CpuOp {
//     code: 0x8D,
//     accronym: "STA", // Store Accumulator: Stores the contents of the accumulator into memory.
//     bytes: 3,
//     cycles: 4,
//     addressing_mode: AddressingMode::Absolute,
// },
// CpuOp {
//     code: 0x68,
//     accronym: "PLA", // Pull Accumulator: Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags are set as appropriate.
//     bytes: 1,
//     cycles: 4,
//     addressing_mode: AddressingMode::Implied,
// },
// CpuOp {
//     code: 0x60,
//     accronym: "RTS", // Return from Subroutine: The RTS instruction is used at the end of a subroutine to return to the calling routine. It pulls the program counter (minus one) from the stack.
//     bytes: 1,
//     cycles: 6,
//     addressing_mode: AddressingMode::Implied,
// },
// CpuOp {
//     code: 0x9D,
//     accronym: "STA", // Store Accumulator: Stores the contents of the accumulator into memory.
//     bytes: 3,
//     cycles: 5,
//     addressing_mode: AddressingMode::AbsoluteX,
// },
// CpuOp {
//     code: 0xCA,
//     accronym: "STA", // Store Accumulator: Stores the contents of the accumulator into memory.
//     bytes: 3,
//     cycles: 5,
//     addressing_mode: AddressingMode::AbsoluteX,
// },
// CpuOp {
//     code: 0xCA,
//     accronym: "DEX", // Decrement X Register: Subtracts one from the X register setting the zero and negative flags as appropriate.
//     bytes: 1,
//     cycles: 2,
//     addressing_mode: AddressingMode::Implied,
// },
// CpuOp {
//     code: 0x10,
//     accronym: "BPL", // Branch if Positive: If the negative flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
//     bytes: 2,
//     cycles: 2, //TODO: check this
//     addressing_mode: AddressingMode::Relative,
// },
// CpuOp {
//     code: 0xAA,
//     accronym: "TAX", // Transfer Accumulator to X: Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.
//     bytes: 1,
//     cycles: 2,
//     addressing_mode: AddressingMode::Implied,
// },
// CpuOp {
//     code: 0xA8,
//     accronym: "TAY", // Transfer Accumulator to Y: Copies the current contents of the accumulator into the Y register and sets the zero and negative flags as appropriate.
//     bytes: 1,
//     cycles: 2,
//     addressing_mode: AddressingMode::Implied,
// },
// CpuOp {
//     code: 0x28,
//     accronym: "PLP", // Pull Processor Status: Pulls an 8 bit value from the stack and into the processor flags. The flags will take on new states as determined by the value pulled.
//     bytes: 1,
//     cycles: 4,
//     addressing_mode: AddressingMode::Implied,
// },
// CpuOp {
//     code: 0x85,
//     accronym: "STA", // Store Accumulator: Stores the contents of the accumulator into memory.
//     bytes: 2,
//     cycles: 3,
//     addressing_mode: AddressingMode::ZeroPage,
// },
// CpuOp {
//     code: 0xA5,
//     accronym: "LDA", // Load Accumulator: Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
//     bytes: 2,
//     cycles: 3,
//     addressing_mode: AddressingMode::ZeroPage,
// },
// CpuOp {
//     code: 0xC9,
//     accronym: "CMP", // Compare: This instruction compares the contents of the accumulator with another memory held value and sets the zero and carry flags as appropriate.
//     bytes: 2,
//     cycles: 3,
//     addressing_mode: AddressingMode::Immediate,
// },
// CpuOp {
//     code: 0xB0,
//     accronym: "BCS", // Branch if Carry Set: If the carry flag is set then add the relative displacement to the program counter to cause a branch to a new location.
//     bytes: 2,
//     cycles: 2, //TODO: check this
//     addressing_mode: AddressingMode::Relative,
// },
// CpuOp {
//     code: 0xD0,
//     accronym: "BNE", // Branch if Not Equal: If the zero flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
//     bytes: 2,
//     cycles: 2, //TODO: check this
//     addressing_mode: AddressingMode::Relative,
// },
// CpuOp {
//     code: 0x46,
//     accronym: "LSR", // Logical Shift Right: Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
//     bytes: 2,
//     cycles: 5,
//     addressing_mode: AddressingMode::ZeroPage,
// },
// CpuOp {
//     code: 0x69,
//     accronym: "ADC", // Add with Carry: This instruction adds the contents of a memory location to the accumulator together with the carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
//     bytes: 2,
//     cycles: 2,
//     addressing_mode: AddressingMode::Immediate,
// },
// CpuOp {
//     code: 0x65,
//     accronym: "ADC", // Add with Carry: This instruction adds the contents of a memory location to the accumulator together with the carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
//     bytes: 2,
//     cycles: 3,
//     addressing_mode: AddressingMode::ZeroPage,
// },
// CpuOp {
//     code: 0x0A,
//     accronym: "ASL", // Arithmetic Shift Left: This operation shifts all the bits of the accumulator or memory contents one bit left. Bit 0 is set to 0 and bit 7 is placed in the carry flag. The effect of this operation is to multiply the memory contents by 2 (ignoring 2's complement considerations), setting the carry if the result will not fit in 8 bits.
//     bytes: 1,
//     cycles: 2,
//     addressing_mode: AddressingMode::Accumulator,
// },
// CpuOp {
//     code: 0x00,
//     accronym: "BRK", // Force Interrupt: The BRK instruction forces the generation of an interrupt request. The program counter and processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag in the status set to one.
//     bytes: 1,
//     cycles: 7,
//     addressing_mode: AddressingMode::Implied,
// },
// ];

// lazy_static! {
//     static ref CPU_OPS: HashMap<u8, CpuOp> = {
//         SUPPORTED_CPU_OPS
//             .into_iter()
//             .map(|op| (op.code, op))
//             .collect()
//     };
// }

// pub struct CpuOp {
//     code: u8,
//     accronym: &'static str,
//     bytes: u8,
//     cycles: u8,
//     addressing_mode: AddressingMode,
// }

// impl Debug for CpuOp {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("CpuOp")
//             .field("code", &format!("${:02x}", self.code).to_uppercase())
//             .field("accronym", &self.accronym)
//             .field("bytes", &self.bytes)
//             .field("cycles", &self.cycles)
//             .field("addressing_mode", &self.addressing_mode)
//             .finish()
//     }
// }

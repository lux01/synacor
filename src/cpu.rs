//! CPU Emulator
//!
//! Emulates the SynCpu architecture.

use byteorder::{LittleEndian, ReadBytesExt};
use instruction::{SynInt, Instruction};

use std::char;
use std::io::{Cursor, Error, stdin, Read};
use std::convert::From;
use std::fmt;

/// The size of RAM for 15-bit addressing.
const RAM_SIZE: usize = 32768;

/// The different error states that the CPU can encounter
#[derive(Debug)]
pub enum SynCpuErr {
    /// There was no error
    Ok,
    /// A pop instruction with an empty stack is an error
    PopOnEmptyStack,
    /// The CPU attempted to read an invalid instruction
    InstructionParseError(Error),
    /// The CPU does not yet support the given instruction
    UnimplementedInstruction(Instruction),
}

impl From<Error> for SynCpuErr {
    #[inline]
    fn from(val: Error) -> SynCpuErr {
        SynCpuErr::InstructionParseError(val)
    }
}

impl fmt::Display for SynCpuErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::SynCpuErr::*;
        match *self {
            Ok => write!(f, "Ok"),
            PopOnEmptyStack => write!(f, "Pop called on empty stack"),
            InstructionParseError(ref err) => write!(f, "Instruction parse error: {}", err),
            UnimplementedInstruction(instr) => write!(f, "Unimplemented instruction: {}", instr),
        }
    }
}

/// An emulator for the SynCpu architecture.
pub struct SynCpu {
    /// The programme counter
    pub pc: u16,
    /// Eight 16-bit registers in the CPU
    pub registers: [u16; 8],
    /// Memory with 15-bit address space storing 16-bit values
    pub ram: [u16; RAM_SIZE],
    /// An unbounded stack which holds individual 16-bit values
    pub stack: Vec<u16>,
    /// Set to true if the execution has halted.
    pub halted: bool,
    /// An enum describing the error that halted execution, if any.
    pub error: SynCpuErr,
}

/// Tests whether a given SynInt is a register or a literal
#[inline]
pub fn is_reg(x: SynInt) -> bool {
    (x as usize) >= RAM_SIZE
}

/// Converts a given SynInt into a String
pub fn syn_int_str(x: SynInt) -> String {
    if is_reg(x) {
        format!("r{}", x as usize - RAM_SIZE)
    } else {
        format!("{}", x)
    }
}

/// Converts a given SynInt into a String. Display literals as hex values.
pub fn syn_int_str_hex(x: SynInt) -> String {
    if is_reg(x) {
        format!("r{}", x as usize - RAM_SIZE)
    } else {
        format!("0x{:0>4x}", x)
    }
}

impl SynCpu {

    /// Converts the given SynInt into a literal value, either directly
    /// or by reading a register
    #[inline]
    fn val(&self, x: SynInt) -> u16 {
        if is_reg(x) {
            self.registers[(x as usize) - RAM_SIZE]
        } else {
            x
        }
    }

    /// Sets the value of the register specified by x to the value a.
    #[inline]
    fn set_reg(&mut self, x: SynInt, a: u16) {
        if is_reg(x) {
            self.registers[(x as usize) - RAM_SIZE] = a;
        }
    }

    /// Constructs a new emulator given an input program.
    pub fn new(program: &[u8]) -> Result<SynCpu, SynCpuErr> {
        let mut cpu = SynCpu {
            pc: 0,
            registers: [0; 8],
            ram: [0xbeef; RAM_SIZE],
            stack: Vec::new(),
            halted: false,
            error: SynCpuErr::Ok,
        };

        let prog_len = program.len();
        let mut rdr = Cursor::new(program);
        let mut i = 0;
        
        while rdr.position() != prog_len as u64 {
            cpu.ram[i] = rdr.read_u16::<LittleEndian>()?;
            i += 1;
        }
        
        Ok(cpu)
    }

    /// Returns the instruction contained at the program counter offset in memory.
    #[inline]
    pub fn next_instruction(&self) -> Instruction {
        self.instruction_at(self.pc as usize)
    }


    /// Returns the instruction contained at the given offset in memory.
    #[inline]
    pub fn instruction_at(&self, offset: usize) -> Instruction {
        Instruction::read_instr(&self.ram[offset..])
    }
    
    /// Exectue a single instruction on the CPU.
    pub fn step(&mut self) {

        // If the CPU has halted already, stop.
        if self.halted {
            return;
        }

        // Read the instruction from memory at the program counter
        use instruction::Instruction::*;
        let instr = self.next_instruction();

        // Execute the instruction
        match instr {
            Halt => {
                self.halted = true;
            },
            
            Set { dst, a } => {
                let val = self.val(a);
                self.set_reg(dst, val);
                self.pc += instr.size();
            },
            Push { src } => {
                let v = self.val(src);
                self.stack.push(v);
                self.pc += instr.size();
            },
            Pop { dst } => {
                if self.stack.is_empty() {
                    self.halted = true;
                    self.error = SynCpuErr::PopOnEmptyStack;
                }
                let val = self.stack.pop().unwrap();
                self.set_reg(dst, val);
                self.pc += instr.size();
            },
            Eq { dst, a, b } => {
                let val = if self.val(a) == self.val(b) {
                    1
                } else {
                    0
                };

                self.set_reg(dst, val);
                self.pc += instr.size();
            },
            Gt { dst, a, b } => {
                let val = if self.val(a) > self.val(b) {
                    1
                } else {
                    0
                };
                self.set_reg(dst, val);
                self.pc += instr.size();
            },

            Jmp { dst } => {
                self.pc = dst;
            },
            Jt { src, dst } => {
                if self.val(src) != 0 {
                    self.pc = dst;
                } else {
                    self.pc += instr.size();
                }
            },
            Jf { src, dst } => {
                if self.val(src) == 0 {
                    self.pc = dst;
                } else {
                    self.pc += instr.size();
                }
            },

            Add { dst, a, b } => {
                let sum = (self.val(a) as u64) + (self.val(b) as u64);
                self.set_reg(dst, (sum % (RAM_SIZE as u64)) as u16);
                self.pc += instr.size();
            },
            Mult { dst, a, b } => {
                let mul = (self.val(a) as u64) * (self.val(b) as u64);
                self.set_reg(dst, (mul % (RAM_SIZE as u64)) as u16);
                self.pc += instr.size();
            },
            Mod { dst, a, b } => {
                let mul = self.val(a) % self.val(b);
                self.set_reg(dst, mul);
                self.pc += instr.size();
            },
            And { dst, a, b } => {
                let val = self.val(a) & self.val(b);
                self.set_reg(dst, val);
                self.pc += instr.size();
            },
            Or { dst, a, b } => {
                let val = self.val(a) | self.val(b);
                self.set_reg(dst, val);
                self.pc += instr.size();
            },
            Not { dst, a } => {
                let val = 0b111111111111111 ^ self.val(a);
                self.set_reg(dst, val);
                self.pc += instr.size();
            },
            ReadMem { dst, src } => {
                let address = self.val(src);
                let val = self.ram[address as usize];
                self.set_reg(dst, val);
                self.pc += instr.size();
            },
            WriteMem { dst, src } => {
                let address = self.val(dst);
                self.ram[address as usize] = self.val(src);
                self.pc += instr.size();
            },
            Call { dst } => {
                let next_instr = self.pc + instr.size();
                self.stack.push(next_instr);
                self.pc = self.val(dst);
            },
            Ret => {
                if self.stack.is_empty() {
                    self.halted = true;
                    self.error = SynCpuErr::PopOnEmptyStack;
                }
                let val = self.stack.pop().unwrap();
                self.pc = val;
            }
            Out { value } => {
                print!("{}", char::from_u32(self.val(value) as u32).unwrap());
                self.pc += instr.size();
            },
            In { dst } => {
                let mut buf = [0; 1];
                stdin().read_exact(&mut buf).unwrap();
                self.set_reg(dst, buf[0] as u16);
                self.pc += instr.size();
            },
            Noop => { self.pc += 1; },
            _ => {
                self.halted = true;
                self.error = SynCpuErr::UnimplementedInstruction(instr);
            }
        };
    }

}

//! CPU Emulator
//!
//! Emulates the SynCpu architecture.

pub mod data;
pub mod status;
pub mod instruction;

pub use self::data::Data;
pub use self::status::Status;
pub use self::instruction::{Operation, Instruction};


/// An emulator for the SynCpu architecture.
#[derive(Clone)]
pub struct SynCpu {
    /// The programme counter
    pub pc: u16,
    /// Set to true if the execution has halted.
    pub halted: bool,
    /// An enum describing the error that halted execution, if any.
    pub status: status::Status,
    /// A buffer for reads from stdin
    stdin_buf: String,
    /// Set to true if the CPU is explicitly stepping
    step: bool
}

const MOD_BASE: u32 = 32768;

impl SynCpu {
    /// Constructs a new VM with a given receiver for input.
    /// Returns the VM and a receiver for output.
    pub fn new() -> SynCpu {
        SynCpu {
            pc: 0,
            halted: false,
            status: status::Status::default(),
            stdin_buf: String::new(),
            step: false,
        }
    }

    /// Returns the next instruction to be evaluated.
    pub fn peek_instr(&self, data: &Data) -> Operation {
        Operation::next(&data[self.pc..])
    }
    
    /// Evaluates the next instruction given the system data
    /// returns any potential output for stdout.
    pub fn step(&mut self, data: &mut Data, input: &[u16]) -> Option<u16> {
        let next_instr = self.peek_instr(&data);

        if let Operation::Breakpoint(instr) = next_instr {
            if self.status == Status::Ok {
                self.status = Status::Interrupted;
                return None;
            }
        }
        let next_instr = next_instr.instr();
        let mut out = None;
        
        use self::Instruction::*;
        match next_instr {
            Halt => {
                self.halted = true;
            },
            Set(dst, a) => {
                let val = data.val(a);
                data[dst] = val;
            },
            Push(src) => {
                let val = data.val(src);
                data.push(val);
            },
            Pop(dst) => {
                if data.is_stack_empty() {
                    self.status = Status::PopOnEmptyStack;
                    self.halted = true;
                } else {
                    data[dst] = data.pop();
                }
            },
            Eq(dst, a, b) => {
                if data.val(a) == data.val(b) {
                    data[dst] = 1;
                } else {
                    data[dst] = 0;
                }
            },
            Gt(dst, a, b) => {
                if data.val(a) > data.val(b) {
                    data[dst] = 1;
                } else {
                    data[dst] = 0;
                }
            },
            Jmp(dst) => {
                self.pc = data.val(dst);
            },
            Jt(src, dst) => {
                if data.val(src) != 0 {
                    self.pc = data.val(dst);
                } else {
                    self.pc += 3;
                }
            },
            Jf(src, dst) => {
                if data.val(src) == 0 {
                    self.pc = data.val(dst);
                } else {
                    self.pc += 3;
                }
            },
            Add(dst, a, b) => {
                let val = (data.val(a) as u32 + data.val(b) as u32) % MOD_BASE;
                data[dst] = val as u16;
            },
            Mult(dst, a, b) => {
                let val = (data.val(a) as u32 * data.val(b) as u32) & MOD_BASE;
                data[dst] = val as u16;
            },
            Mod(dst, a, b) => {
                let val = data.val(a) % data.val(b);
                data[dst] = val;
            },
            And(dst, a, b) => {
                let val = data.val(a) & data.val(b);
                data[dst] = val;
            },
            Or(dst, a, b) => {
                let val = data.val(a) | data.val(b);
                data[dst] = val;
            },
            Not(dst, a) => {
                let val = 0b111111111111111 ^ data.val(a);
                data[dst] = val;
            },
            ReadMem(dst, src) => {
                let mem_addr = data.val(src);
                let val = data[mem_addr];
                data[dst] = val;
            },
            WriteMem(dst, src) => {
                let mem_addr = data.val(dst);
                let val = data.val(src);
                data[mem_addr] = val;
            },
            Call(dst) => {
                data.push(self.pc + 2);
                self.pc = data.val(dst);
            },
            Ret => {
                if data.is_stack_empty() {
                    self.halted = true;
                } else {
                    self.pc = data.pop();
                }
            },
            Out(val) => {
                let val = data.val(val);
                out = Some(val);
            },
            In(dst) => {
                data[dst] = input[0];
            },
            Noop => {
                
            },
            _Unknown => {
                self.status = Status::InstructionParseError;
                self.halted = true;
            }

        }

        // The instruction knows how much to increment the pc by
        self.pc += next_instr.size();
        
        out
    }
    
}

//! CPU Emulator
//!
//! Emulates the SynCpu architecture.

pub mod data;
pub mod status;
pub mod instruction;

pub use self::data::Data;
pub use self::status::Status;
pub use self::instruction::Instruction;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender, channel};

/// The Stdin type used by the CPU
pub type Stdin = Receiver<u16>;
/// The Stdout type used by the CPU
pub type Stdout = Sender<u16>;

/// An emulator for the SynCpu architecture.
#[derive(Clone)]
pub struct SynCpu {
    /// The programme counter
    pub pc: u16,
    /// Set to true if the execution has halted.
    pub halted: bool,
    /// An enum describing the error that halted execution, if any.
    pub status: status::Status,
    /// A receiver for inputs from the terminal
    pub stdin: Arc<Mutex<Stdin>>,
    /// A sender for outputs to the terminal
    pub stdout: Stdout,
}

const MOD_BASE: u32 = 32768;

impl SynCpu {
    /// Constructs a new VM with a given receiver for input.
    /// Returns the VM and a receiver for output.
    pub fn new(stdin: Stdin) -> (SynCpu, Stdin) {
        let (stdout_tx, stdout_rx) = channel();
        let cpu = SynCpu {
            pc: 0,
            halted: false,
            status: status::Status::default(),
            stdin: Arc::new(Mutex::new(stdin)),
            stdout: stdout_tx,
        };
        (cpu, stdout_rx)
    }

    /// Returns the next instruction to be evaluated.
    pub fn peek_instr(&self, data: &Data) -> Instruction {
        Instruction::next(&data[self.pc..])
    }
    
    /// Evaluates the next instruction given the system data
    /// and returns the instruction evaluated.
    pub fn step(&mut self, data: &mut Data) -> Instruction {
        let next_instr = self.peek_instr(&data);

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
                match self.stdout.send(val) {
                    Err(_) => {
                        self.status = Status::StdoutWriteError;
                        self.halted = true;
                    },
                    _ => {},
                }
            },
            In(dst) => {
                match self.stdin.lock().unwrap().recv() {
                    Err(_) => {
                        self.status = Status::StdinReadError;
                        self.halted = true;
                    },
                    Ok(val) => {
                        data[dst] = val;
                    }
                }
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
        
        next_instr
    }
    
}

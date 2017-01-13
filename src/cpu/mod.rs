//! CPU Emulator
//!
//! Emulates the SynCpu architecture.

pub mod data;
pub mod status;
pub mod instruction;

pub use self::data::Data;
pub use self::status::Status;
pub use self::instruction::{Operation, Instruction};

use chan;
use chan_signal;
use chan_signal::Signal;

use termion::{color, style};

use std::char;
use std::io::{stdin, Read};
use std::thread;

/// An emulator for the SynCpu architecture.
#[derive(Clone)]
pub struct SynCpu {
    /// The programme counter
    pub pc: u16,
    /// Set to true if the execution has halted.
    pub halted: bool,
    /// An enum describing the error that halted execution, if any.
    pub status: status::Status,
    /// The VM data
    pub data: Data,
    /// A buffer for reads from stdin
    pub stdin_buf: Vec<char>,
}

const MOD_BASE: u32 = 32768;


impl SynCpu {
    /// Constructs a new VM with a given receiver for input.
    /// Returns the VM and a receiver for output.
    pub fn new(data: Data) -> SynCpu {
        SynCpu {
            pc: 0,
            halted: false,
            status: status::Status::default(),
            data: data,
            stdin_buf: Vec::new(),
        }
    }

    /// Returns the next instruction to be evaluated.
    pub fn peek_op(&self) -> Operation {
        self.peek_op_at(self.pc)
    }

    /// Returns the next instruction at the given offset in RAM.
    pub fn peek_op_at(&self, offset: u16) -> Operation {
        Operation::next(&self.data[offset..])
    }

    
    /// Run the CPU until a breakpoint is hit, exectuion halts
    /// naturally, or an interrupt signal is received.
    pub fn run(&mut self) {
        let signal = chan_signal::notify(&[Signal::INT, Signal::KILL]);
        
        loop {
            chan_select! {
                default => {
                    if self.halted {
                        println!("{red}Halted.{reset}",
                                 red = color::Fg(color::Red),
                                 reset = style::Reset);
                        return;
                    }
                    let next_op = self.peek_op();
                    if next_op.is_breakpoint() {
                        println!("{red}Breakpoint hit.{reset}",
                                 red = color::Fg(color::Red),
                                 reset = style::Reset);
                        return;
                    } else {
                        self.step();
                    }
                },
                signal.recv() => {
                    println!("{red}Received signal. Breaking.{reset}",
                             red = color::Fg(color::Red),
                             reset = style::Reset);
                    return;
                }
            }
        }
    }
    
    /// Evaluates the next instruction given the system data
    /// returns any potential output for stdout.
    pub fn step(&mut self) {
        let next_instr = self.peek_op().instr();

        use self::Instruction::*;
        match next_instr {
            Halt => {
                self.halted = true;
            },
            Set(dst, a) => {
                let val = self.data.val(a);
                self.data[dst] = val;
            },
            Push(src) => {
                let val = self.data.val(src);
                self.data.push(val);
            },
            Pop(dst) => {
                if self.data.is_stack_empty() {
                    self.status = Status::PopOnEmptyStack;
                    self.halted = true;
                } else {
                    self.data[dst] = self.data.pop();
                }
            },
            Eq(dst, a, b) => {
                if self.data.val(a) == self.data.val(b) {
                    self.data[dst] = 1;
                } else {
                    self.data[dst] = 0;
                }
            },
            Gt(dst, a, b) => {
                if self.data.val(a) > self.data.val(b) {
                    self.data[dst] = 1;
                } else {
                    self.data[dst] = 0;
                }
            },
            Jmp(dst) => {
                self.pc = self.data.val(dst);
            },
            Jt(src, dst) => {
                if self.data.val(src) != 0 {
                    self.pc = self.data.val(dst);
                } else {
                    self.pc += 3;
                }
            },
            Jf(src, dst) => {
                if self.data.val(src) == 0 {
                    self.pc = self.data.val(dst);
                } else {
                    self.pc += 3;
                }
            },
            Add(dst, a, b) => {
                let val = (self.data.val(a) as u32 + self.data.val(b) as u32) % MOD_BASE;
                self.data[dst] = val as u16;
            },
            Mult(dst, a, b) => {
                let val = (self.data.val(a) as u32 * self.data.val(b) as u32) % MOD_BASE;
                self.data[dst] = val as u16;
            },
            Mod(dst, a, b) => {
                let val = self.data.val(a) % self.data.val(b);
                self.data[dst] = val;
            },
            And(dst, a, b) => {
                let val = self.data.val(a) & self.data.val(b);
                self.data[dst] = val;
            },
            Or(dst, a, b) => {
                let val = self.data.val(a) | self.data.val(b);
                self.data[dst] = val;
            },
            Not(dst, a) => {
                let val = 0b111111111111111 ^ self.data.val(a);
                self.data[dst] = val;
            },
            ReadMem(dst, src) => {
                let mem_addr = self.data.val(src);
                let val = self.data[mem_addr];
                self.data[dst] = val;
            },
            WriteMem(dst, src) => {
                let mem_addr = self.data.val(dst);
                let val = self.data.val(src);
                self.data[mem_addr] = val;
            },
            Call(dst) => {
                self.data.push(self.pc + 2);
                self.pc = self.data.val(dst);
            },
            Ret => {
                if self.data.is_stack_empty() {
                    self.halted = true;
                } else {
                    self.pc = self.data.pop();
                }
            },
            Out(val) => {
                let val = self.data.val(val);
                print!("{}", char::from_u32(val as u32).unwrap());
            },
            In(dst) => {
                if self.stdin_buf.is_empty() {
                    let signal = chan_signal::notify(&[Signal::INT, Signal::KILL]);
                    use std::sync::mpsc::{self, TryRecvError};
                    let (tx, rx) = chan::sync(0);
                    let (_ctx, crx) = mpsc::channel::<()>();

                    thread::spawn(move || {
                        let mut buf = String::new();
                        while let Err(TryRecvError::Empty) = crx.try_recv() {
                            let mut byte_buf = [0; 1];
                            stdin().read_exact(&mut byte_buf).unwrap();
                            let c = char::from_u32(byte_buf[0] as u32).unwrap();
                            buf.push(c);
                            if c == '\n' {
                                let mut buf = buf.chars().collect::<Vec<_>>();
                                buf.reverse();
                                tx.send(buf);
                                return;
                            }
                        }
                    });
                    
                    chan_select! {
                        signal.recv() => {
                            println!("{red}Breaking during stdin read. Please enter two newlines before attempting to use the debug prompt.{clear}",
                                     red = color::Fg(color::Red),
                                     clear = style::Reset);
                            return;
                        },
                        rx.recv() -> buf => {
                            self.stdin_buf = buf.unwrap();
                        }
                    }
                }
                let c = self.stdin_buf.pop().unwrap();
                self.data[dst] = c as u16;
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
    }
    
}

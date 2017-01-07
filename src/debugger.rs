//! SynCpu Debugger
//!
//! A simple debugger wrapper for SynCpus.
use cpu::SynCpu;
use instruction::{Instruction, SynInt};

use std::collections::VecDeque;
use std::io;
use std::io::{Write};
use std::usize;
use std::u16;

/// A debugger wrapper for a SynCpu with execution history.
pub struct Debugger {
    /// A command history of program counters and executed instructions
    exec_history: VecDeque<(SynInt, Instruction)>,
    /// The number of instructions to store in the history
    history_size: usize,
    /// Breakpoints on which to pause execution
    breakpoints: Vec<SynInt>,
    /// The SynCpu being debugged
    cpu: SynCpu,
}

macro_rules! stdout {
    ($($exprs:expr),*) => {{
        let mut stdout = io::stdout();
        write!(&mut stdout, $($exprs),*).unwrap();
        stdout.flush().unwrap();
    }}
}

macro_rules! stdin {
    () => {{
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        buf
    }}
}

impl Debugger {
    /// Creates a new debugger for the given CPU with a command history of size n.
    pub fn new(cpu: SynCpu, n: usize) -> Debugger {
        Debugger {
            exec_history: VecDeque::new(),
            history_size: n,
            breakpoints: Vec::new(),
            cpu: cpu,
        }

        
    }

    /// The debugger prompt function
    pub fn prompt(&mut self) {
        loop {
            stdout!(" (SynCpu: 0x{:0>4x}) > ", self.cpu.pc);
            
            let prompt_line = stdin!();
            let words = prompt_line.split_whitespace().collect::<Vec<_>>();

            if words.is_empty() {
                continue;
            }

            match words[0] {
                "help" | "h" | "?" => {
                    print!("\tAvailable debugger commands:\n\
                              \t\tquit,        q - Quit the debugger.\n\
                              \t\tregisters,   r - Print the values of the CPU registers.\n\
                              \t\tstack,      st - Displays the stack contents.\n\
                              \t\tstep,        s - Step through one CPU instruction.\n\
                              \t\tbacktrace,  bt - Display the execution history.\n\
                              \t\tlist,        l - List the instructions around the current execution point.\n\
                              \t\tcontinue,    c - Run the CPU until it halts or a breakpoint is hit.\n\
                              \t\tbreakpoint, bp - List all, add, or remove breakpoints.\n\
                              \t\tstatus,    ok? - Returns the execution and error state of the CPU.\n\
                              ");
                }
                "quit" | "q" => {
                    break;
                },
                "registers" | "r" => {
                    println!("\tr0 = 0x{:0>4x},\tr1 = 0x{:0>4x},\tr2 = 0x{:0>4x},\tr3 = 0x{:0>4x}",
                             self.cpu.registers[0],
                             self.cpu.registers[1],
                             self.cpu.registers[2],
                             self.cpu.registers[3]);
                    println!("\tr4 = 0x{:0>4x},\tr5 = 0x{:0>4x},\tr6 = 0x{:0>4x},\tr7 = 0x{:0>4x}",
                             self.cpu.registers[4],
                             self.cpu.registers[5],
                             self.cpu.registers[6],
                             self.cpu.registers[7]);
                },
                "stack" | "st" => {
                    if self.cpu.stack.is_empty() {
                        println!("\tEmpty stack");
                    } else {
                        for val in self.cpu.stack.iter() {
                            println!("\t{:0>4x}", val);
                        }
                    }
                },
                "step" | "s" => {
                    if words.len() > 1 {
                        match words[1].parse() {
                            Ok(n) => self.step_n(n),
                            Err(_) => self.step(),
                        }
                    } else {
                        self.step()
                    }
                },
                "backtrace" | "bt" => {
                    for &(pc, instr) in self.exec_history.iter() {
                        println!("\t0x{:0>4x}: {}", pc, instr);
                    }
                },
                "list" | "l" => {
                    let n = if words.len() > 1 {
                        match words[1].parse() {
                            Ok(n) => n,
                            Err(_) => 5,
                        }
                    } else {
                        5
                    };
                    let mut total_offset = self.cpu.pc as usize;
                    for _ in 0..n {
                        let instr = self.cpu.instruction_at(total_offset);
                        println!("\t0x{:0>4x}: {}",
                                 total_offset,
                                 instr);
                        total_offset += instr.size() as usize;
                    }
                },
                "continue" | "c" => {
                    self.step_n(usize::MAX);
                },
                "breakpoint" | "bp" => {
                    if words.len() < 3 {
                        // List breakpoints
                        if self.breakpoints.is_empty() {
                            println!("\tNo breakpoints set.");
                        } else {
                            println!("\tCurrent breakpoints:");
                            for (i, bp) in self.breakpoints.iter().enumerate() {
                                println!("\t\t{: >4}: 0x{:0>4x}", i, bp);
                            }
                        }
                    } else {
                        match words[1] {
                            "set" => {
                                for bp in &words[2..] {
                                    let bp_val = if bp.starts_with("0x") {
                                        u16::from_str_radix(&bp[2..], 16).unwrap()
                                    } else {
                                        u16::from_str_radix(bp, 16).unwrap()
                                    };
                                    println!("\tAdded breakpoint at 0x{:0>4x}.", bp_val);
                                    self.breakpoints.push(bp_val);
                                }
                            },
                            "unset" => {
                                let bp_idx = words[2].parse().unwrap();
                                let bp = self.breakpoints.remove(bp_idx);
                                println!("\tRemoved breakpoint {} = 0x{:0>4x}", bp_idx, bp);
                            },
                            _ => {
                                println!("\tUnknown option: {}", words[1]);
                            },
                        }
                    }
                },
                "status" | "ok?" => {
                    println!("\t{}", self.cpu.error);
                }
                _ => {
                    println!("\tUnknown command: {}", words[0]);
                }
            }
        }
    }

    /// Step through a single instruction.
    pub fn step(&mut self) {
        if self.cpu.halted {
            println!("\tCPU halted");
            return;
        }
        
        let next_instr = self.cpu.next_instruction();
        let pc = self.cpu.pc;
        if self.exec_history.len() == self.history_size {
            self.exec_history.pop_back();
        }
        self.exec_history.push_front((pc, next_instr));

        self.cpu.step();
    }

    /// Step through up to n instructions, stopping if the CPU halts
    /// or if a breakpoint is hit
    fn step_n(&mut self, n: usize) {
        for _ in 0 .. n {
            if self.breakpoints.contains(&self.cpu.pc) {
                println!("\tBreakpoint hit");
                return;
            }

            if self.cpu.halted {
                println!("\tCPU halted");
                println!("\t{}", self.cpu.error);
                break;
            }
            
            self.step();
        }
    }
}

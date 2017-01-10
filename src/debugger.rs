//! SynCpu Debugger
//!
//! A simple debugger wrapper for SynCpus.

use cpu::{SynCpu, Data, Instruction};

use termion;
use termion::{style, event, input, raw, cursor};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;

use std::char;
use std::io::{stdout, stdin, Write};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Registers,
    Help,
    Unknown,
    Step,
    Run,
    Quit,
}

pub struct Debugger {
    pub original_binary: Vec<u8>,
    pub data: Data,
    pub cpu: SynCpu,
    pub stdin: Sender<u16>,
    pub stdout: Receiver<u16>,
    pub stdout_hist: Vec<u16>,
}

impl Debugger {
    pub fn new(binary: Vec<u8>) -> Debugger {
        let (stdin_tx, stdin_rx) = mpsc::channel();
        let (cpu, stdout_rx) = SynCpu::new(stdin_rx);
        
        let dbg = Debugger {
            data: Data::from_bin(&binary).unwrap(),
            original_binary: binary,
            cpu: cpu,
            stdin: stdin_tx,
            stdout: stdout_rx,
            stdout_hist: Vec::new(),
        };

        println!("{bold}Synacor VM Debugger (SVMDB) v0.1.0{reset}",
                 bold = style::Bold,
                 reset = style::Reset);
        println!("Input binary loaded successfully.\nCPU ready.\n");

        dbg
    }

    pub fn prompt(&mut self) -> Command {
        print!("(SVMDB: 0x{:0>4x}) > ", self.cpu.pc);
        stdout().flush().unwrap();

        let cmd = {
            let mut buf = String::new();
            let mut stdout = stdout().into_raw_mode().unwrap();
            
            for c in stdin().keys() {
                match c.unwrap() {
                    Key::Char('\n') => {
                        write!(&mut stdout, "\r\n").unwrap();
                        break;
                    }
                    Key::Char(ch) => {
                        write!(&mut stdout, "{}", ch).unwrap();
                        stdout.flush().unwrap();
                        buf.push(ch);
                    },
                    Key::Backspace => {
                        if !buf.is_empty() {
                            buf.pop();
                            write!(&mut stdout, "{pos} {pos}",
                                   pos = cursor::Left(1)).unwrap();
                            stdout.flush().unwrap();
                        }
                    },
                    _ => {}
                }
            }
            buf
        };

        match &cmd[..] {
            "help" | "h" => Command::Help,
            "registers" | "r" => Command::Registers,
            "quit" | "q" => Command::Quit,
            "step" | "s" => Command::Step,
            "run" => Command::Run,
            _ => Command::Unknown,
        }
    }

    pub fn step(&mut self) {
        let next_instr = self.cpu.peek_instr(&self.data);

        match next_instr {
            Instruction::In(_) => {
                for k in stdin().keys() {
                    match k.unwrap() {
                        Key::Char(c) => {
                            self.stdin.send(c as u16).unwrap();
                            break;
                        }
                        _ => {},
                    }
                }
            },
            _ => {},
        }

        self.cpu.step(&mut self.data);

        loop {
            match self.stdout.try_recv() {
                Ok(c) => print!("{}", char::from_u32(c as u32).unwrap()),
                Err(_) => break,
            }
        }
    }

    pub fn run(&mut self) {
        let (tx, rx) = mpsc::channel();

        let cpu_thread = {
            let mut cpu = self.cpu.clone();
            let data = self.data.clone();
            thread::spawn(move || {
                println!("CPU spawned");
                let mut data = data;
                while let Err(_) = rx.try_recv() {
                    self.step();
                }
                data
            })
        };

        {
            let cin = self.stdin.clone();
            thread::spawn(move || {
            for k in stdin().keys() {
                match k.unwrap() {
                    Key::Char(c) => {
                        cin.send(c as u16).unwrap();
                    },
                    Key::F(12) => {
                        tx.send(()).unwrap();
                        return;
                    },
                    _ => {},
                }
            }
            });
        }

        self.data = cpu_thread.join().unwrap();
    }
   
    pub fn run_command(&mut self, command: Command) {
        match command {
            Command::Help => {
                println!("Available commands: ");
                println!("\thelp (h) - Prints this message.");
                println!("\tregisters (r) - Prints the contents of the registers.");
                println!("\tquit (q) - Quits SVMDB");
            },
            Command::Registers => {
                println!("\tr0 = 0x{:0>4x}, r1 = 0x{:0>4x}, r2 = 0x{:0>4x}, r3 = 0x{:0>4x}",
                         self.data.registers[0],
                         self.data.registers[1],
                         self.data.registers[2],
                         self.data.registers[3]);
                println!("\tr4 = 0x{:0>4x}, r5 = 0x{:0>4x}, r6 = 0x{:0>4x}, r7 = 0x{:0>4x}",
                         self.data.registers[4],
                         self.data.registers[5],
                         self.data.registers[6],
                         self.data.registers[7]);
            },
            Command::Step => {
                self.step();
            },
            Command::Run => {
                self.run();
            }
            Command::Unknown => {
                println!("Unknown command.");
            },
            Command::Quit => { },
        }
    }
}

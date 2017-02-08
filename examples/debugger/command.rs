//! Debugger commands
use std::collections::HashSet;
use std::convert::From;
use std::char;
use std::u16;

use debugger::Debugger;

use synacor::{SynCpu, Data, Operation};

/// The commands runnable by the debugger
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Registers,
    Help,
    Unknown,
    Step,
    Run,
    Quit,
    Breakpoint,
    Memory,
    Restart,
    Disassemble,
    DumpMemory,
    SetRegister,
    PrintStack,
    Jump
}

impl<'a> From<&'a str> for Command {
    fn from(s: &'a str) -> Command {
        match s {
            "q" | "quit" => Command::Quit,
            "h" | "?" | "help" => Command::Help,
            "s" | "step" => Command::Step,
            "r" | "registers" => Command::Registers,
            "c" | "run" => Command::Run,
            "bp" | "breakpoint" => Command::Breakpoint,
            "m" | "memory" => Command::Memory,
            "restart" => Command::Restart,
            "list" | "l" => Command::Disassemble,
            "dump" => Command::DumpMemory,
            "set" => Command::SetRegister,
            "ps" | "stack" => Command::PrintStack,
            "jump" => Command::Jump,
            _ => Command::Unknown,
        }
    }
}

impl Command {
    pub fn execute(&self, dbg: &mut Debugger, args: &[&str]) {
        use self::Command::*;
        match *self {
            Help => {
                println!("The following commands are available. Short forms are listed \
                          in brackets after the long form. Options, if any are listed \
                          after the short forms");
                println!("\thelp (h, ?)               - Print this message");
                println!("\tstep (s) [n]              - Step through n instructions (default = 1)");
                println!("\tregisters (r)             - Print the registers");
                println!("\trun (c)                   - Run execution until a breakpoint is hit or the CPU halts.");
                println!("\tbreakpoint (bp)           - Set, unset, or list breakpoints.");
                println!("\tmemory (m) [lines] [addr] - Print 20 lines of 8 16-bit entries from RAM, starting at addr. Default lines = 10, default addr = pc");
                println!("\trestart                   - Restart the program.");
                println!("\tlist (l) [n] [addr]       - Disassemble the next n instructions, starting at addr. (default n = 10, addr = pc)");
                println!("\tdump [file]               - Dump the full contents of RAM to the specified file.");
                println!("\tset [n] [value]           - Set register n to the given (decimal) value.");
                println!("\tstack (ps)                - Print the contents of the stack.");
                println!("\tjump [addr]               - Set the programme counter to the given address (in hexadecimal).");
            },
            Step => {
                let times = if args.is_empty() {
                    1
                } else if let Ok(n) = args[0].parse::<usize>() {
                    n
                } else {
                    println!("Usage: step [n] - n is an optional integer (default: 1), the number of steps to take.");
                    return;
                };

                for _ in 0..times {
                    println!(" [0x{:0>4x}]: {}",
                             dbg.cpu.pc,
                             dbg.cpu.peek_op());
                    dbg.cpu.step();
                }
                
            },
            Run => {
                dbg.cpu.run();
            },
            Registers => {
                println!("r0 = 0x{:0>4x}, r1 = 0x{:0>4x}, r2 = 0x{:0>4x}, r3 = 0x{:0>4x}",
                         dbg.cpu.data.registers[0],
                         dbg.cpu.data.registers[1],
                         dbg.cpu.data.registers[2],
                         dbg.cpu.data.registers[3]);
                println!("r4 = 0x{:0>4x}, r5 = 0x{:0>4x}, r6 = 0x{:0>4x}, r7 = 0x{:0>4x}",
                         dbg.cpu.data.registers[4],
                         dbg.cpu.data.registers[5],
                         dbg.cpu.data.registers[6],
                         dbg.cpu.data.registers[7]);
            },
            Breakpoint => {
                let usage = "breakpoint list         - Lists all breakpoints.\n\
                             breakpoint set [addr]   - Set a breakpoint at the given address.\n\
                             breakpoint unset [addr] - Unset the breakpoint at the given address.";
                if args.is_empty() {
                    println!("{}", usage);
                    return;
                }

                match args[0] {
                    "list" => {
                        println!("Set breakpoints:");
                        for addr in dbg.breakpoints.iter() {
                            println!("\t0x{:0>4x}", addr);
                        }
                    },
                    "set" => {
                        let addrs = (&args[1..]).iter()
                            .map(|addr| if addr.starts_with("0x") {
                                usize::from_str_radix(&addr[2..], 16)
                            } else {
                                usize::from_str_radix(addr, 16)
                            })
                            .filter(|addr| addr.is_ok())
                            .map(|addr| addr.unwrap());

                        for addr in addrs {
                            if Operation::is_valid(addr, &dbg.cpu.data.ram) {
                                dbg.cpu.data.ram[addr] |= 0xcc00;
                                dbg.breakpoints.insert(addr);
                                println!("Added breakpoint at 0x{:0>4x}", addr);
                            } else {
                                println!("Address 0x{:0>4x} is not a valid instruction",
                                         addr);
                            }
                        }
                    },
                    "unset" => {
                        let addrs = (&args[1..]).iter()
                            .map(|addr| if addr.starts_with("0x") {
                                usize::from_str_radix(&addr[2..], 16)
                            } else {
                                usize::from_str_radix(addr, 16)
                            })
                            .filter(|addr| addr.is_ok())
                            .map(|addr| addr.unwrap());

                        for addr in addrs {
                            if dbg.breakpoints.contains(&addr) {
                                dbg.breakpoints.remove(&addr);
                                dbg.cpu.data.ram[addr] &= 0x00ff;
                                println!("Breakpoint 0x{:0>4x} removed", addr);
                            } else {
                                println!("Address 0x{:0>4x} is not a breakpoint.",
                                         addr);
                            }
                        }
                    },
                    _ => {
                        println!("{}", usage);
                        return;
                    }
                }
            },
            Memory => {
                let start = if let Some(n) = args.get(0).and_then(|word| {
                    if word.starts_with("0x") {
                        usize::from_str_radix(&word[2..], 16).ok()
                    } else {
                        usize::from_str_radix(word, 16).ok()
                    }
                }) {
                    n
                } else {
                    dbg.cpu.pc as usize
                };

                let lines = if let Some(l) = args.get(1).and_then(|x| x.parse().ok()) {
                    l
                } else {
                    10
                };
                
                println!("addr  0000 1111 2222 3333 4444 5555 6666 7777  01234567");
                println!("-----|----|----|----|----|----|----|----|----||--------|");
                for j in 0..lines {
                    if start + 8*j > dbg.cpu.data.ram.len() {
                        break;
                    }
                    let mut hexs = String::new();
                    let mut printable = String::new();

                    print!("{:0>4x}: ", start + 8*j);
                    
                    for i in 0..8 {
                        let offset = start + i + 8*j;
                        if offset >= dbg.cpu.data.ram.len() {
                            hexs.push_str("END!");
                            break;
                        }
                        let val = dbg.cpu.data.ram[offset];
                        if let Some(c) = char::from_u32(val as u32) {
                            if c.is_alphanumeric() {
                                printable.push(c);
                            } else {
                                printable.push('.');
                            }
                        } else {
                            printable.push('.');
                        }

                        hexs.push_str(&format!("{:0>4x} ", val));
                    }
                    println!("{:40} {}", hexs, printable);
                }
            },
            Restart => {
                let data = Data::from_bin(&dbg.original_binary).unwrap();
                dbg.cpu = SynCpu::new(data);
                dbg.cpu.stdin_buf = dbg.original_replay.clone();
                dbg.breakpoints = HashSet::new();
            },
            Disassemble => {
                let n = if let Some(num) = args.get(0).and_then(|x| x.parse().ok()) {
                    num
                } else {
                    10
                };

                let mut pc = if let Some(num) = args.get(1).and_then(|x| x.parse().ok()) {
                    num
                } else {
                    dbg.cpu.pc
                };
                
                for _ in 0..n {
                    use synacor::Instruction::*;

                    let instr = dbg.cpu.peek_op_at(pc);
                    println!("0x{:0>4x}: {}", pc, instr);

                    pc += match instr.instr() {
                        Halt | Ret => 1,
                        Jmp(_) | Call(_) => 2,
                        Jt(_,_) | Jf(_,_) => 3,
                        x => x.size()
                    };
                }
            },
            DumpMemory => {
                use std::fs::File;
                use byteorder::{LittleEndian, WriteBytesExt};
                    
                let mut fname = String::new();
                for word in args {
                    fname.push_str(word);
                }

                if fname.is_empty() {
                    println!("Please specify a file name for the output");
                    return;
                }

                let file = File::create(fname);
                if let Err(e) = file {
                    println!("Failed to create output file: {}", e);
                } else {
                    let mut file = file.unwrap();

                    for entry in dbg.cpu.data.ram.iter() {
                        file.write_u16::<LittleEndian>(*entry).unwrap();
                    }
                }
            },
            SetRegister => {
                let reg_num = if let Some(val) = args.get(0)
                    .and_then(|x| x.parse::<usize>().ok()) {
                        if val < 8 {
                            val
                        } else {
                            println!("Register number must be between 0 and 7");
                            return;
                        }
                    } else {
                        println!("Register number must be between 0 and 7");
                        return;
                    };

                let val = if let Some(val) = args.get(1)
                    .and_then(|x| x.parse::<u16>().ok()) {
                        val
                    } else {
                        println!("Register value must be a 16-bit unsigned integer.");
                        return;
                    };
                
                dbg.cpu.data.registers[reg_num] = val;
            },
            PrintStack => {
                println!("Stack contents: ");
                for (i, val) in dbg.cpu.data.stack.iter().enumerate() {
                    println!("\t[{}]: 0x{:0>4x}", i, val);
                }
            },
            Jump => {
                let offset = if let Some(val) = args.get(0)
                    .and_then(|x| if x.starts_with("0x") {
                        u16::from_str_radix(&x[2..], 16).ok()
                    }else {
                        u16::from_str_radix(x, 16).ok()
                    }) {
                        val
                    } else {
                        println!("Jump target must be a valid 16-bit hexadecimal unsigned integer.");
                        return;
                    };

                dbg.cpu.pc = offset;
            }
            Quit | Unknown => {}
        }
    }
}



//! SynCpu Debugger
//!
//! A simple debugger wrapper for SynCpus.

use synacor::cpu::{SynCpu, Data, Injection};
use termion::{style};
use libc;
use libc::{SIGINT, signal};


use command::Command;

use std::io::{stdout, stdin, Write, Read};
use std::convert::Into;
use std::collections::HashSet;

pub struct Debugger {
    pub original_binary: Vec<u8>,
    pub original_replay: Vec<char>,
    pub cpu: SynCpu,
    pub breakpoints: HashSet<usize>,
}

extern "C" fn ignore_interrupt(_: libc::c_int) {
    print!("\n              > ");
    stdout().flush().unwrap();
}

fn check_cargo() -> bool{
    unsafe {
        use std::fs::File;
        
        let parent_pid = libc::getppid();
        let mut f = File::open(format!("/proc/{}/comm", parent_pid)).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        buf == "cargo\n"
    }
}

impl Debugger {

    pub fn new(binary: Vec<u8>, replay: Vec<char>, injections: &[Injection]) -> Debugger {
        let mut data = Data::from_bin(&binary).unwrap();

        for injection in injections {
            injection.inject(&mut data);
        }
        
        let mut cpu = SynCpu::new(data);
        cpu.stdin_buf = replay.clone();
        
        Debugger {
            original_binary: binary,
            original_replay: replay,
            cpu: cpu,
            breakpoints: HashSet::new(),
        }
        
    }

    pub fn main_loop(&mut self) {
        if check_cargo() {
            println!("Warning! The VM is running under cargo, interrupts handling has been disabled.");
        } else {
            unsafe {
                use libc::{c_int, c_void, sighandler_t};
                signal(SIGINT,
                       ignore_interrupt as extern fn(c_int) as *mut c_void as sighandler_t
                );
            }
        }
        println!("{bold}Synacor VM debugger version 0.1.0{reset}",
                 bold = style::Bold,
                 reset = style::Reset);
       
        loop {
            print!("\n(SVM: 0x{:0>4x}) > ", self.cpu.pc);
            stdout().flush().unwrap();
            let mut buf = String::new();
            let result = stdin().read_line(&mut buf);

            if let Err(_) = result {
                println!("");
                stdout().flush().unwrap();
                continue;
            }
            
            let words = buf.split_whitespace().collect::<Vec<_>>();


            if words.is_empty() {
                continue;
            }
            let cmd: Command = words[0].into();
            if cmd == Command::Quit {
                return;
            } else if cmd == Command::Unknown {
                println!("Unknown command: {:?}", buf);
            } else {
                cmd.execute(self, &words[1..]);
            }
        }
    }

}

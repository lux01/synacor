//! SynCpu Debugger
//!
//! A simple debugger wrapper for SynCpus.

#[macro_use]
mod macros;

use cpu::{SynCpu, Data};
use termion::{style};
use chan;
use chan_signal;
use chan_signal::Signal;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Step,
    Run,
    Quit,
}

pub struct Debugger {
    pub original_binary: Vec<u8>,
    pub data: Data,
    pub cpu: SynCpu,
    mode: Mode,
}

impl Debugger {
    pub fn new(binary: Vec<u8>) -> Debugger {
        let cpu = SynCpu::new();
        Debugger {
            data: Data::from_bin(&binary).unwrap(),
            original_binary: binary,
            cpu: cpu,
            mode: Mode::Step,
        }
    }

    pub fn main_loop(&mut self) {
        println!("{bold}Synacor VM version 0.1.0){reset}",
                 bold = style::Bold,
                 reset = style::Reset);
    }

}

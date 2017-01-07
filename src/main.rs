//! # Synacor Challenge
//!
//! A Rust based runtime for the Synacor challenge architecture.
#![warn(missing_docs)]

extern crate byteorder;

pub mod instruction;
pub mod cpu;
pub mod debugger;

use std::io::Read;
use std::fs::File;

use cpu::SynCpu;

const BINARY_NAME: &'static str = "challenge.bin";

fn main() {
    let binary = {
        let mut buffer = Vec::new();
        let mut in_file = File::open(BINARY_NAME)
            .expect("Failed to open challenge binary.");
        in_file.read_to_end(&mut buffer)
            .expect("Failed to read in binary contents.");
        buffer
    };

    let cpu = SynCpu::new(&binary)
        .expect("Failed to create CPU");

    let mut gdb = debugger::Debugger::new(cpu, 20);

    gdb.prompt();
}

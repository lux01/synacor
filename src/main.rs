//! # Synacor Challenge
//!
//! A Rust based runtime for the Synacor challenge architecture.
#![warn(missing_docs)]

extern crate byteorder;
extern crate termion;
#[macro_use] extern crate chan;
extern crate chan_signal;

pub mod cpu;
pub mod syn_int;

mod debugger;
use debugger::Debugger;

use std::io::Read;
use std::fs::File;

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
    
    let mut dbg = Debugger::new(binary);

    dbg.main_loop();

    println!("Goodbye!");
}

//! # Synacor Challenge
//!
//! A Rust based runtime for the Synacor challenge architecture.
#![warn(missing_docs)]

extern crate byteorder;
extern crate termion;
#[macro_use] extern crate chan;
extern crate chan_signal;
extern crate libc;
extern crate synacor;

mod command;
mod debugger;
use debugger::Debugger;

use std::io::Read;
use std::fs::File;
use std::env::args;

fn main() {
    let binary = if let Some(val) = args().nth(1) {
        let mut buffer = Vec::new();
        let mut in_file = File::open(val)
            .expect("Failed to open challenge binary.");
        in_file.read_to_end(&mut buffer)
            .expect("Failed to read in binary contents.");
        buffer
    } else {
        println!("Usage: debugger <binary> [replay] [injections]");
        return;
    };

    let replay = if let Some(val) = args().nth(2) {
        let mut buffer = String::new();
        let mut replay_file = File::open(val)
            .expect("Failed to open replay file");
        replay_file.read_to_string(&mut buffer)
            .expect("Failed to read in replay file");
        let mut buffer: Vec<_> = buffer.chars().collect();
        buffer.reverse();
        println!("Replay buffer loaded");
        buffer
    } else {
        Vec::new()
    };

    let injections = if let Some(val) = args().nth(3) {
        let mut buffer = String::new();
        let mut injection_file = File::open(val)
            .expect("Failed to open injection file");
        injection_file.read_to_string(&mut buffer)
            .expect("Failed to read in injection file");
        synacor::Injection::from_json(&buffer)
    } else {
        vec![]
    };
   
    let mut dbg = Debugger::new(binary, replay, &injections);
    dbg.main_loop();
    
    println!("Goodbye!");
}


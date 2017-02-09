extern crate synacor;

use std::io::Read;
use std::fs::File;
use std::env::args;

use synacor::{SynCpu, Data, Injection};

fn main() {
    // Load the binary, replay and injections
    let binary = if let Some(val) = args().nth(1) {
        let mut buffer = Vec::new();
        let mut in_file = File::open(val)
            .expect("Failed to open challenge binary.");
        in_file.read_to_end(&mut buffer)
            .expect("Failed to read in binary contents.");
        buffer
    } else {
        println!("Usage: synvm <binary> [replay] [injections]");
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
        Injection::from_json(&buffer)
    } else {
        vec![]
    };

    // Prepare the CPU
    let mut data = Data::from_bin(&binary)
        .expect("Failed to load decode program binary.");

    for injection in injections {
        injection.inject(&mut data);
    }

    let mut cpu = SynCpu::new(data);
    cpu.stdin_buf = replay;
    cpu.loud = false;

    // Run the CPU
    cpu.run();

}

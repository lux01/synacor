extern crate synacor;

use synacor::{Data, Operation};

use std::io::{Read, Write};
use std::fs::File;
use std::env::args;

fn main() {
    let binary = if let Some(val) = args().nth(1) {
        let mut buffer = Vec::new();
        let mut in_file = File::open(val)
            .expect("Failed to open binary.");
        in_file.read_to_end(&mut buffer)
            .expect("Failed to read in binary contents.");
        buffer
    } else {
        println!("Usage: disassembler <binary> <output>");
        return;
    };

    let mut out_file = if let Some(val) = args().nth(2) {
        File::create(val)
            .expect("Failed to create output file.")
    } else {
        println!("Usage: disassembler <binary> <output>");
        return;
    };
    
    let data = Data::from_bin(&binary)
        .expect("Failed to parse binary");
    
    let mut pc: u16 = 0;
    while pc != data.ram.len() as u16 {
        let instr = Operation::next(&data[pc..]).instr();

        writeln!(&mut out_file, "0x{:0>4x}: {:#}", pc, instr)
            .expect("Failed to write output line");

        pc += instr.word_size();
    }
}

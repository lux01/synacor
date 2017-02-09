//! Code injections
//!
//! This module

use serde_json;
use cpu::Data;

/// A struct for injecting arbitrary data into a binary
#[derive(Debug, Serialize, Deserialize)]
pub struct Injection {
    /// The memory address to start injecting at
    addr: u16,
    /// The sequence of words to inject
    payload: Vec<u16>,
}



impl Injection {

    /// Creates a vector of injections from a json string
    pub fn from_json(json: &str) -> Vec<Injection> {
        match serde_json::from_str(json) {
            Ok(vec) => vec,
            Err(e) => {
                println!("Deserialization error: {}", e);
                vec![]
            }
        }
    }

    /// Inject the payload
    pub fn inject(&self, data: &mut Data) {
        let mut pc = self.addr;

        for i in self.payload.iter() {
            data.ram[pc as usize] = *i;
            pc += 1;
        }
    }
}

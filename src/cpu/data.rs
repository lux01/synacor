//! CPU Memory and registers structure

use std::io;
use std::io::Cursor;
use std::ops::{Index, IndexMut, RangeFrom};

use syn_int::SynInt;

use byteorder::{LittleEndian, ReadBytesExt};

/// The size of RAM for 15-bit addressing.
/// Also the modular basis for all integer arithmetic
pub const RAM_SIZE: usize = 32768;

/// The data structures accessible on this architecture.
#[derive(Clone)]
pub struct Data {
    /// Eight 16-bit registers
    pub registers: [u16; 8],
    /// A 15-bit address space storing 16-bit values
    pub ram: Vec<u16>,
    /// An unbounded stack storing 16-bit values,
    pub stack: Vec<u16>,
}

impl Data {
    /// Constructs a new CPU data structure given a program binary.
    pub fn from_bin(binary: &[u8]) -> io::Result<Data> {
        let mut data = Data {
            registers: [0; 8],
            ram: vec![0; RAM_SIZE],
            stack: Vec::new(),
        };

        let bin_len = binary.len();
        let mut rdr = Cursor::new(binary);
        let mut idx = 0;

        while rdr.position() != bin_len as u64 {
            data.ram[idx] = rdr.read_u16::<LittleEndian>()?;
            idx += 1;
        }
        
        Ok(data)
    }

    /// Pops a value from the stack. Panics if the stack is empty
    pub fn pop(&mut self) -> u16 {
        self.stack.pop().unwrap()
    }

    /// Push a value onto the stack.
    pub fn push(&mut self, val: u16) {
        self.stack.push(val)
    }

    /// Checks whether the stack is empty or not
    pub fn is_stack_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Convert a SynInt to a u16 either directly or
    /// by accessing a register.
    pub fn val(&self, idx: SynInt) -> u16 {
        match idx {
            SynInt::Literal(x) => x,
            SynInt::Register(r) => self.registers[r],
        }
    }
}

impl<'a> Index<SynInt> for Data {
    type Output = u16;

    fn index(&self, idx: SynInt) -> &u16 {
        match idx {
            SynInt::Literal(_) => panic!("Attempted to read a register with a literal"),
            SynInt::Register(r) => self.registers.index(r),
        }
    }
}
impl Index<u16> for Data {
    type Output = u16;

    fn index(&self, idx: u16) -> &u16 {
        self.ram.index((idx as usize % RAM_SIZE) as usize)
    }
}

impl Index<usize> for Data {
    type Output = u16;
    
    fn index(&self, idx: usize) -> &u16 {
        self.ram.index(idx % RAM_SIZE)
    }
}

impl Index<RangeFrom<u16>> for Data {
    type Output = [u16];

    fn index(&self, idx: RangeFrom<u16>) -> &[u16] {
        self.ram.index(idx.start as usize ..)
    }
}
impl IndexMut<SynInt> for Data {

    fn index_mut(&mut self, idx: SynInt) -> &mut u16 {
        match idx {
            SynInt::Literal(_) => panic!("Attempted to write to a register using a literal."),
            SynInt::Register(r) => self.registers.index_mut(r),
        }
    }
}
impl IndexMut<u16> for Data {
    fn index_mut(&mut self, idx: u16) -> &mut u16 {
        self.ram.index_mut((idx as usize % RAM_SIZE) as usize)
    }
}

impl IndexMut<usize> for Data {
    fn index_mut(&mut self, idx: usize) -> &mut u16 {
        self.ram.index_mut(idx % RAM_SIZE)
    }
}



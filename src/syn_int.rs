//! The integer type used in SynCpu binaries.

use std::fmt;

/// An enum wrapping the two possible values that an integer can have
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SynInt {
    /// A literal 16-bit value
    Literal(u16),
    /// A register on the CPU
    Register(usize),
}

impl From<u16> for SynInt {
    fn from(val: u16) -> SynInt {
        if val < 32768 {
            SynInt::Literal(val)
        } else {
            SynInt::Register((val - 32768) as usize)
        }
    }
}

impl From<SynInt> for u16 {
    fn from(val: SynInt) -> u16 {
        match val {
            SynInt::Literal(x) => x,
            SynInt::Register(x) => x as u16 + 32768,
        }
    }
}

impl fmt::Display for SynInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SynInt::Literal(val) => write!(f, "{}", val),
            SynInt::Register(val) => write!(f, "r{}", val),
        }
    }
}

impl fmt::LowerHex for SynInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SynInt::Literal(val) => write!(f, "0x{:0>4x}", val),
            SynInt::Register(val) => write!(f, "r{}", val),
        }
    }
}

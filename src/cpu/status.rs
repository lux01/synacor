//! CPU Status information

use std::fmt;
use std::error;
use std::default::Default;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    /// The CPU is operating normally
    Ok,
    /// An attempt to pop on an empty stack was performed
    PopOnEmptyStack,
    /// An instruction could not be parsed
    InstructionParseError,
    /// An unimplemented instruction was requested
    UnimplementedInstruction,
    /// The CPU could not write to stdout
    StdoutWriteError,
    /// The CPU could not read from stdin
    StdinReadError,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Status::*;
        match *self {
            Ok => write!(f, "Ok"),
            PopOnEmptyStack => write!(f, "Pop on empty stack"),
            InstructionParseError => write!(f, "Instruction parse error"),
            UnimplementedInstruction => write!(f, "Unimplemented instruction error"),
            StdoutWriteError => write!(f, "Stdout write error"),
            StdinReadError => write!(f, "Stdin read error"),
        }
    }
}

impl error::Error for Status {
    fn description(&self) -> &str {
        use self::Status::*;
        match *self {
            Ok => "Ok",
            PopOnEmptyStack => "Pop on empty stack",
            InstructionParseError => "Instruction parse error",
            UnimplementedInstruction => "Unimplemented instruction error",
            StdoutWriteError => "Stdout write error",
            StdinReadError => "Stdin read error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Ok
    }
}

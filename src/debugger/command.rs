//! Debugger commands
use std::convert::From;
use std::fmt;

/// The commands runnable by the debugger
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Registers,
    Help,
    Unknown,
    Step,
    Run,
    Quit,
}

impl From<&str> for Command {
    fn from(s: &str) -> Command {
        match s {
            "q" | "quit" => Command::Quit,
            "h" | "?" | "help" => Command::Help,
            "s" | "step" => Command::Step,
            "r" | "registers" => Command::Registers,
            "c" | "run" => Command::Run,
            _ => Command::Unknown,
        }
    }
}

impl Command {
    fn execute(&self,
               dbg: &mut Debugger,
               args: Vec<&str>,
               stdout: &Sender<String>) {
        use self::Command::*;
        match *self {
            Help => {
                
            }
        }
    }
}


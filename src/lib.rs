//! # Synacor Challenge
//!
//! A rust based virtual machine for the Synacor challenge.
#![warn(missing_docs)]

extern crate byteorder;
extern crate termion;
#[macro_use] extern crate chan;
extern crate chan_signal;

pub mod cpu;

pub use cpu::{Data, Status, Operation, Instruction, SynCpu};

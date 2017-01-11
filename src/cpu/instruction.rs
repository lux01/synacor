//! Instruction decoder
//!
//! This module provides a decoder and enum type for the 22 different instructions
//! supported by this architecture. These are summarised in the following table:
//!
//! | Variant | Opcode | Assembly | Description |
//! | --- | --- | --- | --- |
//! | `Halt` | `0` | `halt    ` | Stop execution and terminate the program. |
//! | `Set` | `1` |  `set     dst a` | Set register `dst` to the value of `a`. |
//! | `Push` | `2` | `push    src` | Push `a` onto the stack. |
//! | `Pop` | `3` |  `pop     dst` | Remove the top element from the stack and write it into `a`. |
//! | `Eq` | `4` | `eq dst a b` | Set `dst` to `1` if `a` is equal to `b`, otherwise set it to `0`. |
//! | `Gt` | `5` | `gt dst a b` | Set `dst` to `1` if `a` is greater than `b`, otherwise set it to `0`. |
//! | `Jmp` | `6` | `jmp dst` | Jump unconditionally to `dst`. |
//! | `Jt` | `7` | `jmnz src dst` | If `src` is non-zero, jump to `dst`. |
//! | `Jf` | `8` | `jmpz src dst` | If `src` is zero, jump to `dst`. |
//! | `Add` | `9` | `add dst a b` | Assign into `dst` the sum of `a` and `b` (mod 2^(15)) |
//! | `Mult` | `10` | `mul dst a b` | Assign into `dst` the product of `a` and `b` (mod 2^(15)) |
//! | `Mod` | `11` | `mod dst a b` | Assign into `dst` the remainder of `a` divided by `b`. |
//! | `And` | `12` | `and dst a b` | Store in `dst` the bitwise AND of `a` and `b`. |
//! | `Or` | `13` | `or dst a b` | Store in `dst` the bitwise OR of `a` and `b`. |
//! | `Not` | `14` | `not dst a` | Store the 15-bit inverse of `a` into `dst`. |
//! | `ReadMem` | `15` | `rmem dst src` | Read memory at address `src` and write it to `dst` |
//! | `WriteMem` | `16` | `wmem dst src` | Write the value from `src` into memory at address `src`. |
//! | `Call` | `17` | `call dst` | Write the address of the next instruction to the stack and jump to `dst`.
//! | `Ret` | `18` | `ret` | Remove the top element from the stack and jump to it, halt on an empty stack. |
//! | `Out` | `19` | `out val` | Write the ASCII character code `val` to the terminal. |
//! | `In` | `20` | `in dst` | Read a character from the terminal and write its ASCII code to `dst`. |
//! | `Noop` | `21` | `noop` | No operation. |

use syn_int::SynInt;

use std::fmt;

/// A VM extension to support interrupt instructions.
///
/// Interrupt symbols are used so the debugger can pause execution
/// and inspect the VM. They are implemented using the upper byte
/// of a 16-bit instruction. If the upper byte is equal to `0xcc`
/// (the `int 3` opcode in x86) then the instruction is a breakpoint,
/// otherwise it is a regular instruction.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operation {
    /// A regular instruction
    Regular(Instruction),
    /// A debugging breakpoint, execution should pause before
    /// this instruction is exectued.
    Breakpoint(Instruction),
}

impl Operation {

    pub fn next(ram: &[u16]) -> Operation {
        use self::Instruction::*;
        let instr = match 0x00ff & ram[0] {
            0 => Halt,
            1 => Set(ram[1].into(), ram[2].into()),
            2 => Push(ram[1].into()),
            3 => Pop(ram[1].into()),
            4 => Eq(ram[1].into(), ram[2].into(), ram[3].into()),
            5 => Gt(ram[1].into(), ram[2].into(), ram[3].into()),
            6 => Jmp(ram[1].into()),
            7 => Jt(ram[1].into(), ram[2].into()),
            8 => Jf(ram[1].into(), ram[2].into()),
            9 => Add(ram[1].into(), ram[2].into(), ram[3].into()),
            10 => Mult(ram[1].into(), ram[2].into(), ram[3].into()),
            11 => Mod(ram[1].into(), ram[2].into(), ram[3].into()),
            12 => And(ram[1].into(), ram[2].into(), ram[3].into()),
            13 => Or(ram[1].into(), ram[2].into(), ram[3].into()),
            14 => Not(ram[1].into(), ram[1].into()),
            15 => ReadMem(ram[1].into(), ram[2].into()),
            16 => WriteMem(ram[1].into(), ram[2].into()),
            17 => Call(ram[1].into()),
            18 => Ret,
            19 => Out(ram[1].into()),
            20 => In(ram[1].into()),
            21 => Noop,
            _ => _Unknown,
        };

        if (ram[0] >> 8) & 0xcc == 0xcc {
            Operation::Breakpoint(instr)
        } else {
            Operation::Regular(instr)
        }
    }

    pub fn instr(self) -> Instruction {
        match self {
            Operation::Regular(i) => i,
            Operation::Breakpoint(i) => i,
        }
    }
    
}

/// Enum representation of all the supported instructions.
///
/// `SynInt`s are used instead of raw `u16` values for arguments. See the module
/// documentation for more infromation about what these operations do and what
/// the arguments mean.
#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Instruction {
    Halt,
    Set(SynInt, SynInt),
    Push(SynInt),
    Pop(SynInt),
    Eq(SynInt, SynInt, SynInt),
    Gt(SynInt, SynInt, SynInt),
    Jmp(SynInt),
    Jt(SynInt, SynInt),
    Jf(SynInt, SynInt),
    Add(SynInt, SynInt, SynInt),
    Mult(SynInt, SynInt, SynInt),
    Mod(SynInt, SynInt, SynInt),
    And(SynInt, SynInt, SynInt),
    Or(SynInt, SynInt, SynInt),
    Not(SynInt, SynInt),
    ReadMem(SynInt, SynInt),
    WriteMem(SynInt, SynInt),
    Call(SynInt),
    Ret,
    Out(SynInt),
    In(SynInt),
    Noop,
    #[doc(hidden)]
    _Unknown,
}

impl Instruction {
    /// Reads the next instruction from the provided slice
    /// into RAM. Panics if the slice is not long enough to decode
    /// an instruction fully.
    pub fn next(ram: &[u16]) -> Instruction {
        use self::Instruction::*;
        match ram[0] {
            0 => Halt,
            1 => Set(ram[1].into(), ram[2].into()),
            2 => Push(ram[1].into()),
            3 => Pop(ram[1].into()),
            4 => Eq(ram[1].into(), ram[2].into(), ram[3].into()),
            5 => Gt(ram[1].into(), ram[2].into(), ram[3].into()),
            6 => Jmp(ram[1].into()),
            7 => Jt(ram[1].into(), ram[2].into()),
            8 => Jf(ram[1].into(), ram[2].into()),
            9 => Add(ram[1].into(), ram[2].into(), ram[3].into()),
            10 => Mult(ram[1].into(), ram[2].into(), ram[3].into()),
            11 => Mod(ram[1].into(), ram[2].into(), ram[3].into()),
            12 => And(ram[1].into(), ram[2].into(), ram[3].into()),
            13 => Or(ram[1].into(), ram[2].into(), ram[3].into()),
            14 => Not(ram[1].into(), ram[1].into()),
            15 => ReadMem(ram[1].into(), ram[2].into()),
            16 => WriteMem(ram[1].into(), ram[2].into()),
            17 => Call(ram[1].into()),
            18 => Ret,
            19 => Out(ram[1].into()),
            20 => In(ram[1].into()),
            21 => Noop,
            _ => _Unknown,
        }
    }

    /// Returns the amount to increment the program counter by after
    /// executing the instruction. Note that this returns 0 for all jump
    /// instructions as they modify the program counter directly.
    pub fn size(&self) -> u16 {
        use self::Instruction::*;

        match *self {
            // Jumps will increment the pc manually
            Halt | Jmp(_) | Jt(_, _) | Jf(_, _) | Call(_) | Ret => 0,
            Noop | _Unknown => 1,
            Push(_) | Pop(_) | Out(_) | In(_) => 2,
            Set(_, _) | Not(_, _) | ReadMem(_, _) | WriteMem(_, _) => 3,
            _ => 4,
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;
        match *self {
            Halt               => write!(f, "halt "),
            Set(dst, a)        => write!(f, "set  {} {}",
                                         dst, a),
            Push(src)          => write!(f, "push {}",
                                         src),
            Pop(dst)           => write!(f, "pop  {}",
                                         dst),
            Eq(dst, a, b)      => write!(f, "eq   {} {} {}",
                                         dst, a, b),
            Gt(dst, a, b)      => write!(f, "gt   {} {} {}",
                                         dst, a, b),
            Jmp(dst)           => write!(f, "jmp  {:x}",
                                         dst),
            Jt(src, dst)       => write!(f, "jmnz {} {:x}",
                                         src, dst),
            Jf(src, dst)       => write!(f, "jmpz {} {:x}",
                                         src, dst),
            Add(dst, a, b)     => write!(f, "add  {} {} {}",
                                         dst, a, b),
            Mult(dst, a, b)    => write!(f, "mult {} {} {}",
                                         dst, a, b),
            Mod(dst, a, b)     => write!(f, "mod  {} {} {}",
                                         dst, a, b),
            And(dst,a , b)     => write!(f, "and  {} {} {}",
                                         dst, a, b),
            Or(dst, a, b)      => write!(f, "or   {} {} {}",
                                         dst, a, b),
            Not(dst, a)        => write!(f, "not  {} {}",
                                         dst, a),
            ReadMem(dst, src)  => write!(f, "rmem {} {:x}",
                                         dst, src),
            WriteMem(dst, src) => write!(f, "wmem {} {:x}",
                                         dst, src),
            Call(dst)          => write!(f, "call {:x}",
                                         dst),
            Ret                => write!(f, "ret  "),
            Out(val)           => write!(f, "out  {}",
                                         val),
            In(dst)            => write!(f, "in   {}",
                                         dst),
            Noop               => write!(f, "noop "),
            _Unknown           => write!(f, "????"),
        }
    }
}

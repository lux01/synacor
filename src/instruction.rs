//! Architecture instructions

use std::fmt;
use cpu;

/// The single number type supported by the SynCpu.
pub type SynInt = u16;

/// The different opcode listings for the SynCpu.
#[derive(Debug, Clone, Copy)]
#[allow(missing_docs)]
pub enum Instruction {
    /// Stop execution and terminate the program
    Halt,
    /// Set register `dst` to the value of `a`
    Set { dst: SynInt, a: SynInt },
    /// Push `a` onto the stack
    Push { src: SynInt },
    /// Remove the top element from the stack and write it into `a`,
    /// an empty stack is an error.
    Pop { dst: SynInt },
    /// Set `dst` to `1` if `a` is equal to `b`, otherwise set it to `0`.
    Eq { dst: SynInt, a: SynInt, b: SynInt },
    /// Set `dst` to `1` if `a` is greater than `b`, otherwise set it to `0`.
    Gt { dst: SynInt, a: SynInt, b: SynInt },
    /// Jump unconditionally to `dst`
    Jmp { dst: SynInt },
    /// If `src` is non-zero, jump to `dst`
    Jt { src: SynInt, dst: SynInt },
    /// If `src` is zero, jump to `dst`.
    Jf { src: SynInt, dst: SynInt },
    /// Assign into `dst` the sum of `a` and `b` (modulo 32768).
    Add { dst: SynInt, a: SynInt, b: SynInt },
    /// Assign into `dst` the product of `a` and `b` (modulo 32768).
    Mult { dst: SynInt, a: SynInt, b: SynInt },
    /// Assign into `dst` the remainder of `a` divided by `b`.
    Mod { dst: SynInt, a: SynInt, b: SynInt },
    /// Store into `dst` the bitwise AND of `a` and `b`.
    And { dst: SynInt, a: SynInt, b: SynInt },
    /// Store into `dst` the bitwise OR of `a` and `b`.
    Or { dst: SynInt, a: SynInt, b: SynInt },
    /// Store the 15-bit inverse of `a` into `dst`.
    Not { dst: SynInt, a: SynInt },
    /// Read memory at address `src` and write it to `dst`.
    ReadMem { dst: SynInt, src: SynInt },
    /// Write the value from `src` into the memory at address `a`.
    WriteMem { dst: SynInt, src: SynInt },
    /// Write the address of the next instruction to the stack and jump to `dst`.
    Call { dst: SynInt },
    /// Remove the top element from the stack and jump to it,
    /// halt on an empty stack.
    Ret,
    /// Write the character represented by ASCII code `value` to the terminal.
    Out { value: SynInt },
    /// Read a character from the terminal and write its ascii code to `dst`.
    /// It can be assumed that once input starts, it will continue until a
    /// newline is encountered; this means that you can safely read whole lines
    /// from the keyboard and trust that they will be fully read.
    In { dst: SynInt },
    /// No operation
    Noop,
    #[doc(hidden)]
    _Unknown,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;

        match *self {
            Halt => {
                write!(f, "halt")
            },
            Set { dst, a } => {
                write!(f, "set    {}, {}", cpu::syn_int_str(dst), a)
            }
            Push { src } => {
                write!(f, "push   {}", cpu::syn_int_str(src))
            },
            Pop { dst } => {
                write!(f, "pop    {}", dst)
            }
            Eq { dst, a, b } => {
                write!(f, "eq     {}, {}, {}",
                       cpu::syn_int_str(dst),
                       cpu::syn_int_str(a),
                       cpu::syn_int_str(b))
            },
            Gt { dst, a, b } => {
                write!(f, "gt     {}, {}, {}",
                       cpu::syn_int_str(dst),
                       cpu::syn_int_str(a),
                       cpu::syn_int_str(b))                
            },
            Jmp { dst } => {
                write!(f, "jump   {}", cpu::syn_int_str_hex(dst))
            },
            Jt { src, dst } => {
                write!(f, "jmnz   {}, {}",
                       cpu::syn_int_str_hex(src),
                       cpu::syn_int_str_hex(dst))
            },
            Jf { src, dst } => {
                write!(f, "jmpz   {}, {}",
                       cpu::syn_int_str_hex(src),
                       cpu::syn_int_str_hex(dst))
            },
            Add { dst, a, b } => {
                write!(f, "add    {}, {}, {}",
                       cpu::syn_int_str(dst),
                       cpu::syn_int_str(a),
                       cpu::syn_int_str(b))
            },
            Mult { dst, a, b } => {
                write!(f, "mul    {}, {}, {}",
                       cpu::syn_int_str(dst),
                       cpu::syn_int_str(a),
                       cpu::syn_int_str(b))
            },
            Mod { dst, a, b } => {
                write!(f, "mod    {}, {}, {}",
                       cpu::syn_int_str(dst),
                       cpu::syn_int_str(a),
                       cpu::syn_int_str(b))
            },
            And { dst, a, b } => {
                write!(f, "and    {}, {}, {}",
                       cpu::syn_int_str(dst),
                       cpu::syn_int_str(a),
                       cpu::syn_int_str(b))
            },
            Or { dst, a, b } => {
                write!(f, "or     {}, {}, {}",
                       cpu::syn_int_str(dst),
                       cpu::syn_int_str(a),
                       cpu::syn_int_str(b))
            },
            Not { dst, a } => {
                write!(f, "not    {}, {}",
                       cpu::syn_int_str(dst),
                       cpu::syn_int_str(a))
            },
            ReadMem { dst, src } => {
                write!(f, "rmem   {}, {}",
                       cpu::syn_int_str(dst),
                       cpu::syn_int_str(src))
            },
            WriteMem { dst, src } => {
                write!(f, "wmem   {}, {}",
                       cpu::syn_int_str(dst),
                       cpu::syn_int_str(src))
            },
            Call { dst } => {
                write!(f, "call    {}",
                       cpu::syn_int_str(dst))
            },
            Ret => {
                write!(f, "ret     ")
            },
            Out { value } => {
                write!(f, "out    {}",
                       cpu::syn_int_str(value))
            },
            In { dst } => {
                write!(f, "in     {}",
                       cpu::syn_int_str(dst))
            },
            Noop => {
                write!(f, "noop   ")
            },
            _Unknown => {
                write!(f, "unknown")
            },
        }
    }
}

impl Instruction {
    /// Returns the number of memory addresses consumed by this instruction
    #[allow(unused_variables)]
    pub fn size(&self) -> u16 {
        use self::Instruction::*;

        match *self {
            Halt => 1,
            Set { dst, a } => 3,
            Push { src } => 2,
            Pop { dst } => 2,
            Eq { dst, a, b } => 4,
            Gt { dst, a, b } => 4,
            Jmp { dst } => 2,
            Jt { src, dst } => 3,
            Jf { src, dst } => 3,
            Add { dst, a, b } => 4,
            Mult { dst, a, b } => 4,
            Mod { dst, a, b } => 4,
            And { dst, a, b } => 4,
            Or { dst, a, b } => 4,
            Not { dst, a } => 3,
            ReadMem { dst, src } => 3,
            WriteMem { dst, src } => 3,
            Call { dst } => 2,
            Ret => 1,
            Out { value } => 2,
            In { dst } => 2,
            Noop => 1,
            _Unknown => 1,
        }
    }
    
    /// Read a single instruction from the Cursor.
    pub fn read_instr(binary: &[SynInt]) -> Instruction {
        use self::Instruction::*;
        let opcode = binary[0];

        match opcode {
            0 => Halt,
            1 => {
                Set {
                    dst: binary[1],
                    a: binary[2],
                }
            }
            2 => Push { src: binary[1] },
            3 => Pop { dst: binary[1] },
            4 => {
                Eq {
                    dst: binary[1],
                    a: binary[2],
                    b: binary[3],
                }
            }
            5 => {
                Gt {
                    dst: binary[1],
                    a: binary[2],
                    b: binary[3],
                }
            }
            6 => Jmp { dst: binary[1] },
            7 => {
                Jt {
                    src: binary[1],
                    dst: binary[2],
                }
            }
            8 => {
                Jf {
                    src: binary[1],
                    dst: binary[2],
                }
            }
            9 => {
                Add {
                    dst: binary[1],
                    a: binary[2],
                    b: binary[3],
                }
            }
            10 => {
                Mult {
                    dst: binary[1],
                    a: binary[2],
                    b: binary[3],
                }
            }
            11 => {
                Mod {
                    dst: binary[1],
                    a: binary[2],
                    b: binary[3],
                }
            }
            12 => {
                And {
                    dst: binary[1],
                    a: binary[2],
                    b: binary[3],
                }
            }
            13 => {
                Or {
                    dst: binary[1],
                    a: binary[2],
                    b: binary[3],
                }
            }
            14 => {
                Not {
                    dst: binary[1],
                    a: binary[2],
                }
            }
            15 => {
                ReadMem {
                    dst: binary[1],
                    src: binary[2],
                }
            }
            16 => {
                WriteMem {
                    dst: binary[1],
                    src: binary[2],
                }
            }
            17 => Call { dst: binary[1] },
            18 => Ret,
            19 => Out { value: binary[1] },
            20 => In { dst: binary[1] },
            21 => Noop,
            _ => _Unknown,
        }
    }
}

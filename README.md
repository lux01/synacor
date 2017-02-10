# Synacor Challenge

The [Synacor Challenge][Synacor] is a programming challenge where contestents are tasked to implement a virtual machine for a custom architecture specified in the [`arch-spec`](/arch-spec) file. A binary [`bin/challenge.bin`](/bin/challenge.bin) has been provided and contains a number of passcodes for input on the challenge website to track your progress. This repository contains a Rust library that implements the CPU, and some extra executables that implement the VM and solve various challenges that appear during execution.

[Synacor]: https://challenge.synacor.com

**NB**: This repository contains all the details you need to be able to find all the codes for the challenge without having to implement your own solutions. Please don't ruin the fun for everyone else by cheating!

## Executables

### Debugger

The debugger can be built using cargo, but it is recommended to run the debugger as a standalone executable due to a [bug in cargo][cargo-bug]. Build the debugger using cargo:
```
$ cargo build --release --bin debugger
```
and run using
```
$ target/release/debugger <binary> [replay] [injections]
```
where `binary` is the binary to execute, `replay` is a text file to feed into the CPU as a `stdin` input, and `injections` is a JSON array of code injections which takes the form:
```JSON
[
    {
        "addr": address,
        "payload": [...]
    },
    ...
]
```
where `address` is the memory address to start the injection at, and the `payload` field lists the 16-bit words to inject into the binary. This is used during execution of the challenge binary to modify some instructions to reach the final stages of the challenge.

The debugger starts at a prompt that provides various commands, the synopsis of which can be found using the `help` command:
```
(SVM: 0x0000) > help
The following commands are available. Short forms are listed in brackets after the long form. Options, if any are listed after the short forms
        help (h, ?)               - Print this message
        step (s) [n]              - Step through n instructions (default = 1)
        registers (r)             - Print the registers
        run (c)                   - Run execution until a breakpoint is hit or the CPU halts.
        breakpoint (bp)           - Set, unset, or list breakpoints.
        memory (m) [lines] [addr] - Print 20 lines of 8 16-bit entries from RAM, starting at addr. Default lines = 10, default addr = pc
        restart                   - Restart the program.
        list (l) [n] [addr]       - Disassemble the next n instructions, starting at addr. (default n = 10, addr = pc)
        dump [file]               - Dump the full contents of RAM to the specified file.
        set [n] [value]           - Set register n to the given (decimal) value.
        stack (ps)                - Print the contents of the stack.
        jump [addr]               - Set the programme counter to the given address (in hexadecimal).
```
Use `Ctrl+C` to interrupt the CPU during execution to drop back to the debug prompt.

### Virtual machine

The virtual machine can be built and run using cargo:
```
$ cargo run --release --bin synvm -- <binary> [replay] [injections]
```
where `binary`, `replay`, and `injections` are as specified above in the [Debugger](#debugger). To exit the virutal machine use `Ctrl+C`.


### Disassembler

The disassembler can also be built and run using cargo:
```
$ cargo run --release --bin disassembler -- <binary> <output>
```
which will disassemble `binary` and write the results to `output`.

## Other binaries

There are some other binaries provided that relate to solving challenges that appear during execution of the challenge binary.

### Brute forcer

To get past the 6th code, one needs to force the CPU to modify a register to a specific value during execution. The value of this register is then checked using a routine that will not finish executing in any reasonable amount of time. The checking routine is a modification of the [Ackermann function](https://en.wikipedia.org/wiki/Ackermann_function) and is done modulo 2^^15. The `brute_forcer` binary computes the correct value of this register using an optimization of the algorithm, and can be run using cargo:
```
$ cargo run --release --bin brute_forcer
```
the output of which can be seen [here](/the_eight_register.txt).

### Vault grid solver

To get the final code, you need to navigate through a grid with special rules for movement and unlocking the final door. A Haskell binary `synacor-grid` is provided in [`grid-src/Main.rs`](/grid-src/Main.rs) and can be built and run using `stack`:
```
$ stack build
$ stack exec synacor-grid
```
the output of which can be seen [here](/grid_solution.txt).

## License

The code in this repository is licensed under the [MIT license](/LICENSE) except the challenge binary, and architecture specification which are provided by Synacor.

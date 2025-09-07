# brainfuck interpreter

A simple Brainfuck interpreter written in Rust. It implements all of the Brainfuck commands and is able to run Brainfuck programs from files.

## Usage

To run program, use the following command:

```
$ cargo run code.bf
```

### Example program

Here is a program that implements hello world:

```
>++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.
```


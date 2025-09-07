enum Op {
    IncrementDataPtr,
    DecrementDataPtr,
    IncrementByte,
    DecrementByte,
    OutputByte,
    InputByte,
    LoopStart,
    LoopEnd,
}

fn parse_brainfuck(code: &str) -> Vec<Op> {
    code.chars()
        .filter_map(|c| match c {
            '>' => Some(Op::IncrementDataPtr),
            '<' => Some(Op::DecrementDataPtr),
            '+' => Some(Op::IncrementByte),
            '-' => Some(Op::DecrementByte),
            '.' => Some(Op::OutputByte),
            ',' => Some(Op::InputByte),
            '[' => Some(Op::LoopStart),
            ']' => Some(Op::LoopEnd),
            _ => None,
        })
        .collect()
}

struct Interpreter {
    program_ptr: usize,
    inst_ptr: usize,
    data_ptr: usize,
    memory: [u8; 30_000],
}

impl Interpreter {
    fn new() -> Self {
        Self {
            program_ptr: 0,
            inst_ptr: 0,
            data_ptr: 0,
            memory: [0; 30_000],
        }
    }

    fn run(&mut self, ops: &[Op]) {
        while self.inst_ptr < ops.len() {
            match ops[self.inst_ptr] {
                Op::IncrementDataPtr => self.data_ptr += 1,
                Op::DecrementDataPtr => self.data_ptr -= 1,
                Op::IncrementByte => {
                    self.memory[self.data_ptr] = self.memory[self.data_ptr].wrapping_add(1)
                }
                Op::DecrementByte => {
                    self.memory[self.data_ptr] = self.memory[self.data_ptr].wrapping_sub(1)
                }
                Op::OutputByte => print!("{}", self.memory[self.data_ptr] as char),
                Op::InputByte => {
                    use std::io::{self, Read};
                    let mut buffer = [0; 1];
                    io::stdin().read_exact(&mut buffer).unwrap();
                    self.memory[self.data_ptr] = buffer[0];
                }
                Op::LoopStart => {
                    if self.memory[self.data_ptr] == 0 {
                        let mut loop_counter = 1;
                        while loop_counter > 0 {
                            self.inst_ptr += 1;
                            if let Op::LoopStart = ops[self.inst_ptr] {
                                loop_counter += 1;
                            } else if let Op::LoopEnd = ops[self.inst_ptr] {
                                loop_counter -= 1;
                            }
                        }
                    }
                }
                Op::LoopEnd => {
                    if self.memory[self.data_ptr] != 0 {
                        let mut loop_counter = 1;
                        while loop_counter > 0 {
                            self.inst_ptr -= 1;
                            if let Op::LoopEnd = ops[self.inst_ptr] {
                                loop_counter += 1;
                            } else if let Op::LoopStart = ops[self.inst_ptr] {
                                loop_counter -= 1;
                            }
                        }
                    }
                }
            }
            self.inst_ptr += 1;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("code.bf")?;

    let ops = parse_brainfuck(&content);
    let mut interpreter = Interpreter::new();
    interpreter.run(&ops);

    Ok(())
}

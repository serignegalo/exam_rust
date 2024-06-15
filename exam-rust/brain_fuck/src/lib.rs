use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    brain_fuck(&args[1]);
}

pub fn brain_fuck(code: &str) {
    const MEMORY_SIZE: usize = 2048;
    let mut memory = vec![0u8; MEMORY_SIZE];
    let mut pointer = 0;
    let code_chars: Vec<char> = code.chars().collect();
    let mut pc = 0;
    let mut loop_stack = Vec::new();

    while pc < code_chars.len() {
        match code_chars[pc] {
            '>' => pointer = (pointer + 1) % MEMORY_SIZE,
            '<' => pointer = (pointer + MEMORY_SIZE - 1) % MEMORY_SIZE,
            '+' => memory[pointer] = memory[pointer].wrapping_add(1),
            '-' => memory[pointer] = memory[pointer].wrapping_sub(1),
            '.' => {
                print!("{}", memory[pointer] as char);
                io::stdout().flush().unwrap();
            }
            ',' => {
                // Input functionality can be implemented here if needed
            }
            '[' => {
                if memory[pointer] == 0 {
                    let mut loop_counter = 1;
                    while loop_counter > 0 {
                        pc += 1;
                        match code_chars[pc] {
                            '[' => loop_counter += 1,
                            ']' => loop_counter -= 1,
                            _ => {}
                        }
                    }
                } else {
                    loop_stack.push(pc);
                }
            }
            ']' => {
                if memory[pointer] != 0 {
                    pc = *loop_stack.last().unwrap();
                } else {
                    loop_stack.pop();
                }
            }
            _ => {} // Ignore any other characters as comments
        }
        pc += 1;
    }
}

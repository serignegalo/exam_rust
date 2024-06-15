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


#[cfg(test)]
mod tests {
    use std::process::Command;

    const MANIFEST_PATH: &str = "../../solutions/brain_fuck/Cargo.toml";

    fn run(s: &str) -> String {
        let output = Command::new("cargo")
            .arg("run")
            .arg("--manifest-path")
            .arg(MANIFEST_PATH)
            .arg(s)
            .output()
            .expect("Failed to execute command");

        String::from_utf8(output.stdout).unwrap()
    }

    #[test]
    fn nothing_passed() {
        assert_eq!("", run(""));
    }

    #[test]
    fn single_chars() {
        assert_eq!(
            "a",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>---.")
        );
        assert_eq!(
            "S",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+++++++++++++.")
        );
        assert_eq!(
            "7",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>---------------.")
        );
    }
    #[test]
    fn phrases() {
        assert_eq!(
            "Wow",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>>-------------.++++++++++++++++++++++++.++++++++.")
        );
        assert_eq!(
            "Good job!",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>+.>+++++++++++..-----------.<<++.>>++++++.+++++.-------------.<<+.")
        );
    }

    #[test]
    fn with_characters_in_middle() {
        assert_eq!("keep going", run("++++++++++[>+>ke+++>+++++++>++++++++++<<<<-]>>>>+++++++e.------p..+++++++++++.<<++.>g>---------.+o+++++++.------i.+++++.-n------.g"));
    }

    #[test]
    fn big_test() {
        assert_eq!(
            "3, 2, 1... Happy New Year",
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>-------------------.-------.<++.>++++++.------.<.>+++++.---...<.>++++++++++++++++++++++++++.>---.+++++++++++++++..+++++++++.<<.>++++++.>--------------------.++++++++++++++++++.<<.>+++++++++++.++++++++++++.----.>-----.")
        );
        assert_eq!(
            "To be or not be, that is the question!", 
            run("++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++++++++++++++.>+++++++++++.<<++.>>-------------.+++.<<.>>++++++++++.+++.<<.>>----.+.+++++.<<.>++++++++++++++.+++.<++++++++++++.------------.>>.<+++.-------.>.<<.>++++++++.>-.<<.>>+.<-.---.<.>>---.++++.<.>--.+.<++++.>-----.-.<<+.")
        );
    }
}

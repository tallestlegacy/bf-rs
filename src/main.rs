use std::{env, fs};

const STACK_SIZE: usize = 10;
enum Instruction {
    Add,
    Subtract,
    OpenLoop,
    CloseLoop,
    Input,
    Ouput,
    Forward,
    Back,
    Ignore,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let path = args[1].clone();
        let content = fs::read_to_string(path);

        match content {
            Ok(value) => {
                println!("{}", execute_to_string(value));
            }
            Err(e) => {
                println!("{e}");
            }
        }
    } else {
        println!("Requires one argument 'path'.");
    }
}

fn execute_to_string(input: String) -> String {
    let mut output = String::new();
    let mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut stack_pointer: usize = 0;
    let mut instruction_index: usize = 0;
    let mut loop_opening_index: i64 = -1;

    while instruction_index < input.len() {
        let character = input.chars().nth(instruction_index).unwrap();
        match get_instruction(character) {
            Instruction::Add => {
                stack[stack_pointer] += 1;
            }
            Instruction::Subtract => {
                if stack[stack_pointer] > 0 {
                    stack[stack_pointer] -= 1;
                }
            }

            Instruction::OpenLoop => {
                loop_opening_index = instruction_index as i64;
            }
            Instruction::CloseLoop => {
                if stack[0] > 0 {
                    instruction_index = loop_opening_index as usize;
                } else {
                    loop_opening_index = -1;
                }
            }
            Instruction::Input => {}

            Instruction::Ouput => {
                let decimal_value = stack[stack_pointer];
                output.push(decimal_value as char);
            }
            Instruction::Forward => {
                stack_pointer += 1;
                if stack_pointer == STACK_SIZE {
                    stack_pointer = 0
                }
            }
            Instruction::Back => {
                if stack_pointer == 0 {
                    stack_pointer = STACK_SIZE - 1;
                } else {
                    stack_pointer -= 1;
                }
            }
            Instruction::Ignore => {}
        }
        instruction_index += 1;
    }

    output
}

fn get_instruction(character: char) -> Instruction {
    match character {
        '+' => Instruction::Add,
        '-' => Instruction::Subtract,
        '[' => Instruction::OpenLoop,
        ']' => Instruction::CloseLoop,
        ',' => Instruction::Input,
        '.' => Instruction::Ouput,
        '>' => Instruction::Forward,
        '<' => Instruction::Back,
        _ => Instruction::Ignore,
    }
}
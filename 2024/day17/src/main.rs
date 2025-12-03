mod vector2;
use rayon::prelude::*;
use vector2::Vector2i;

#[derive(Debug, Clone)]
struct Computer {
    program: Vec<u8>,
    output: Vec<u8>,
    instruction_pointer: usize,
    A: i64,
    B: i64,
    C: i64,
}

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let computer = parse_input(input);
    let output = execute_program(computer.clone());
    let output_string = output
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    println!("Output: {:?}", output_string);

    let A = find_correct_A(computer);
    println!("Found A: {A}");
}

fn execute_program(mut computer: Computer) -> Vec<u8> {
    while computer.instruction_pointer < computer.program.len() {
        let instruction = computer.program[computer.instruction_pointer];
        match instruction {
            0 => {
                let operand = get_combo_operand(&computer);
                computer.A /= 2_i64.pow(operand as u32);
            }
            1 => {
                let operand = get_literal_operand(&computer);
                computer.B ^= operand as i64;
            }
            2 => {
                let operand = get_combo_operand(&computer);
                computer.B = operand % 8;
            }
            3 => {
                if computer.A != 0 {
                    computer.instruction_pointer = get_literal_operand(&computer) as usize;
                    continue;
                }
            }
            4 => computer.B ^= computer.C,
            5 => {
                let operand = get_combo_operand(&computer);
                computer.output.push((operand % 8) as u8);
            }
            6 => {
                let operand = get_combo_operand(&computer);
                computer.B = computer.A / 2_i64.pow(operand as u32);
            }
            7 => {
                let operand = get_combo_operand(&computer);
                computer.C = computer.A / 2_i64.pow(operand as u32);
            }
            _ => unreachable!(),
        }

        computer.instruction_pointer += 2;
    }

    computer.output
}

fn find_correct_A(computer: Computer) -> usize {
    let mut i: i64 = 190384100000000;
    let i_skip = 1000;
    println!("Program length: {}", computer.program.len());
    loop {
        let mut new_computer = computer.clone();
        new_computer.A = i;
        let output = execute_program(new_computer);
        println!("{}: {}, {:?}", i, output.len(), output);
        if output[15] == 0
            && output[14] == 3
            && output[13] == 5
            && output[12] == 5
            && output[11] == 7
            && output[10] == 1
            && output[9] == 7
            && output[8] == 4
            && output[7] == 3
            && output[6] == 0
            && output[5] == 5
        {
            i -= i_skip;
            println!("found i of {i}");
            break;
        }
        i += i_skip;
    }
    let group_size = 10000000;
    loop {
        println!("{i}");
        if let Some(A) = (i..i + group_size).into_par_iter().find_any(|A| {
            let mut new_computer = computer.clone();
            new_computer.A = *A;
            computer.program == execute_program(new_computer)
        }) {
            return A as usize;
        }
        i += group_size;
    }
}

fn get_literal_operand(computer: &Computer) -> u8 {
    computer.program[computer.instruction_pointer + 1]
}

fn get_combo_operand(computer: &Computer) -> i64 {
    let operand_index = computer.instruction_pointer + 1;
    let operand = computer.program[operand_index];

    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => computer.A,
        5 => computer.B,
        6 => computer.C,
        _ => {
            unreachable!()
        }
    }
}

fn parse_input(input: &str) -> Computer {
    let lines: Vec<&str> = input.lines().collect();

    let A = lines[0].split_once(": ").unwrap().1.parse::<i64>().unwrap();
    let B = lines[1].split_once(": ").unwrap().1.parse::<i64>().unwrap();
    let C = lines[2].split_once(": ").unwrap().1.parse::<i64>().unwrap();

    let program: Vec<u8> = lines[4]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .collect();

    Computer {
        program,
        instruction_pointer: 0,
        output: Vec::new(),
        A,
        B,
        C,
    }
}

const TEST_INPUT: &str = "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

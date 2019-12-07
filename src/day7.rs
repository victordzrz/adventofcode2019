use crate::common::{get_input, to_int_vec};
use permutator::Permutation;
use std::convert::TryInto;
use std::io::stdin;

pub struct IntcodeComputer {
    memory: Vec<i128>,
    pointer: usize,
}

#[derive(Debug)]
pub enum Parameter {
    Immediate(i128),
    Position(usize),
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum IntcodeReturn {
    WaitForInput,
    Output(i128),
    Continue,
    End,
}

#[derive(Debug)]
pub enum Instruction {
    Add(Parameter, Parameter, usize),
    Multiply(Parameter, Parameter, usize),
    Input(usize),
    Output(usize),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, usize),
    Equals(Parameter, Parameter, usize),
    Break,
    Unknown,
}

impl IntcodeComputer {
    pub fn get_next_instruction(&self) -> Instruction {
        let instruction_code = self.memory[self.pointer];
        match instruction_code % 100 {
            1 => Instruction::Add(
                self.get_parameter(instruction_code, 1),
                self.get_parameter(instruction_code, 2),
                self.memory[self.pointer + 3].try_into().unwrap(),
            ),
            2 => Instruction::Multiply(
                self.get_parameter(instruction_code, 1),
                self.get_parameter(instruction_code, 2),
                self.memory[self.pointer + 3].try_into().unwrap(),
            ),
            3 => Instruction::Input(self.memory[self.pointer + 1].try_into().unwrap()),
            4 => Instruction::Output(self.memory[self.pointer + 1].try_into().unwrap()),
            5 => Instruction::JumpIfTrue(
                self.get_parameter(instruction_code, 1),
                self.get_parameter(instruction_code, 2),
            ),
            6 => Instruction::JumpIfFalse(
                self.get_parameter(instruction_code, 1),
                self.get_parameter(instruction_code, 2),
            ),
            7 => Instruction::LessThan(
                self.get_parameter(instruction_code, 1),
                self.get_parameter(instruction_code, 2),
                self.memory[self.pointer + 3].try_into().unwrap(),
            ),
            8 => Instruction::Equals(
                self.get_parameter(instruction_code, 1),
                self.get_parameter(instruction_code, 2),
                self.memory[self.pointer + 3].try_into().unwrap(),
            ),
            99 => Instruction::Break,
            _ => Instruction::Unknown,
        }
    }

    pub fn get_parameter(&self, instruction_code: i128, position: u32) -> Parameter {
        let position_u: usize = position.try_into().unwrap();
        let pos_multiplier: i128 = (10 as i128).pow(position);
        let mode: i128 = (instruction_code % (100 * pos_multiplier)) / (10 * pos_multiplier);
        match mode {
            0 => Parameter::Position(self.memory[self.pointer + position_u].try_into().unwrap()),
            1 => Parameter::Immediate(self.memory[self.pointer + position_u]),
            _ => panic!(
                "Unknown mode {} for instruction code {} param number {}",
                mode, instruction_code, position
            ),
        }
    }

    pub fn fetch_data(&self, param: Parameter) -> i128 {
        match param {
            Parameter::Immediate(value) => value,
            Parameter::Position(pos) => self.memory[pos],
            Parameter::Unknown => panic!("Unknow data"),
        }
    }

    pub fn run(&mut self, input: Option<i128>) -> IntcodeReturn {
        let mut current_input = input;
        //println!("Run with {:?}", input);
        loop {
            let instruction = self.get_next_instruction();
            let result = match instruction {
                Instruction::Add(p1, p2, p3) => {
                    self.memory[p3] = self.fetch_data(p1) + self.fetch_data(p2);
                    self.pointer += 4;
                    IntcodeReturn::Continue
                }
                Instruction::Multiply(p1, p2, p3) => {
                    self.memory[p3] = self.fetch_data(p1) * self.fetch_data(p2);
                    self.pointer += 4;
                    IntcodeReturn::Continue
                }
                Instruction::Input(p1) => match current_input {
                    Some(value) => {
                        self.memory[p1] = value;
                        self.pointer += 2;
                        current_input = None;
                        IntcodeReturn::Continue
                    }
                    None => IntcodeReturn::WaitForInput,
                },
                Instruction::Output(p1) => {
                    let output = IntcodeReturn::Output(self.memory[p1]);
                    self.pointer += 2;
                    output
                }
                Instruction::JumpIfTrue(p1, p2) => {
                    if self.fetch_data(p1) != 0 {
                        self.pointer = self.fetch_data(p2).try_into().unwrap();
                    } else {
                        self.pointer += 3;
                    }
                    IntcodeReturn::Continue
                }
                Instruction::JumpIfFalse(p1, p2) => {
                    if self.fetch_data(p1) == 0 {
                        self.pointer = self.fetch_data(p2).try_into().unwrap();
                    } else {
                        self.pointer += 3;
                    }
                    IntcodeReturn::Continue
                }
                Instruction::LessThan(p1, p2, dest) => {
                    if self.fetch_data(p1) < self.fetch_data(p2) {
                        self.memory[dest] = 1;
                    } else {
                        self.memory[dest] = 0;
                    }
                    self.pointer += 4;
                    IntcodeReturn::Continue
                }
                Instruction::Equals(p1, p2, dest) => {
                    if self.fetch_data(p1) == self.fetch_data(p2) {
                        self.memory[dest] = 1;
                    } else {
                        self.memory[dest] = 0;
                    }
                    self.pointer += 4;
                    IntcodeReturn::Continue
                }
                Instruction::Break => IntcodeReturn::End,
                _ => panic!("Unknow opcode instruction"),
            };

            //println!("next is {:?}", result);

            if let IntcodeReturn::Continue = result {
                continue;
            } else {
                return result;
            }
        }
    }
}

pub fn run_thrusters(input_settings: Vec<i128>, software: Vec<i128>) -> i128 {
    let mut previous_output: i128 = 0;
    for setting in input_settings {
        let mut computer = IntcodeComputer {
            memory: software.clone(),
            pointer: 0,
        };

        let mut inputs: Vec<i128> = vec![previous_output, setting];
        let mut result = computer.run(inputs.pop());
        loop {
            //println!("Got {:?}", result);
            let mut next_input = None;
            match result {
                IntcodeReturn::Output(value) => {
                    previous_output = value;
                }
                IntcodeReturn::End => {
                    break;
                }
                IntcodeReturn::WaitForInput => {
                    next_input = inputs.pop();
                }
                _ => continue,
            }
            //println!("Next input is: {:?}", next_input);
            result = computer.run(next_input);
        }
    }
    return previous_output;
}

pub fn run_with_feedback(input_settings: Vec<i128>, software: Vec<i128>) -> i128 {
    let mut previous_output: i128 = 0;
    let mut computers: Vec<IntcodeComputer> = input_settings
        .iter()
        .map(|_| IntcodeComputer {
            memory: software.clone(),
            pointer: 0,
        })
        .collect();

    //Init with settings
    for i in 0..5 {
        computers[i].run(Some(input_settings[i]));
    }

    let mut previous_outputs: Vec<IntcodeReturn> = vec![
        IntcodeReturn::Output(0),
        IntcodeReturn::Output(0),
        IntcodeReturn::Output(0),
        IntcodeReturn::Output(0),
        IntcodeReturn::Output(0),
    ];

    let mut last_good_output = 0;
    loop {
        for current_computer_index in 0..5 {
            let computer = &mut computers[current_computer_index];
            let previous_computer_index = if current_computer_index == 0 {
                4
            } else {
                current_computer_index - 1
            };

            if let IntcodeReturn::Output(output) = previous_outputs[previous_computer_index] {
                previous_outputs[current_computer_index] = computer.run(Some(output));
            } else {
                previous_outputs[current_computer_index] = computer.run(None);
            }

            //println!("Got {:?}", previous_outputs[current_computer_index]);
        }
        if let IntcodeReturn::Output(value) = previous_outputs[4] {
            last_good_output = value;
        }
        if previous_outputs.iter().all(|o| IntcodeReturn::End == *o) {
            //println!("Final output {}",last_good_output );
            return last_good_output;
        }
    }
}

pub fn star1() {
    let input = to_int_vec(get_input("inputs/day7.txt", ","))
        .iter()
        .map(|x| *x as i128)
        .collect::<Vec<i128>>();
    let possible_settings: Vec<i128> = vec![4, 3, 2, 1, 0];
    let mut all_permutations: Vec<Vec<i128>> = possible_settings.clone().permutation().collect();
    all_permutations.push(possible_settings); //permutation() doesn't output the first permutation!!
    let result = all_permutations
        .iter()
        .map(|v| (v.clone(), run_thrusters(v.clone(), input.clone())))
        .max_by_key(|s| s.1)
        .unwrap();

    println!(
        "Max output {}. Settings {}",
        result.1,
        result
            .0
            .iter()
            .fold(String::new(), |s, v| s + &v.to_string())
    );
}

pub fn star2() {
    let input = to_int_vec(get_input("inputs/day7.txt", ","))
        .iter()
        .map(|x| *x as i128)
        .collect::<Vec<i128>>();
    let possible_settings: Vec<i128> = vec![9, 8, 7, 6, 5];
    let mut all_permutations: Vec<Vec<i128>> = possible_settings.clone().permutation().collect();
    all_permutations.push(possible_settings); //permutation() doesn't output the first permutation!!
    let result = all_permutations
        .iter()
        .map(|v| (v.clone(), run_with_feedback(v.clone(), input.clone())))
        .max_by_key(|s| s.1)
        .unwrap();

    println!(
        "Max output {}. Settings {}",
        result.1,
        result
            .0
            .iter()
            .fold(String::new(), |s, v| s + &v.to_string())
    );
}

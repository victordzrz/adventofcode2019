use crate::common::{get_input, to_int_vec};
use std::convert::TryInto;
use std::io::stdin;

pub struct IntcodeComputer {
    memory: Vec<i64>,
    pointer: usize,
}

#[derive(Debug)]
enum Parameter {
    Immediate(i64),
    Position(usize),
    Unknown,
}

#[derive(Debug)]
enum Instruction {
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

    pub fn get_parameter(&self, instruction_code: i64, position: u32) -> Parameter {
        let position_u: usize = position.try_into().unwrap();
        let pos_multiplier: i64 = (10 as i64).pow(position);
        let mode: i64 = (instruction_code % (100 * pos_multiplier)) / (10 * pos_multiplier);
        match mode {
            0 => Parameter::Position(self.memory[self.pointer + position_u].try_into().unwrap()),
            1 => Parameter::Immediate(self.memory[self.pointer + position_u]),
            _ => panic!(
                "Unknown mode {} for instruction code {} param number {}",
                mode, instruction_code, position
            ),
        }
    }

    pub fn fetch_data(&self, param: Parameter) -> i64 {
        match param {
            Parameter::Immediate(value) => value,
            Parameter::Position(pos) => self.memory[pos],
            Parameter::Unknown => panic!("Unknow data"),
        }
    }

    pub fn read_input() -> i64 {
        let mut buffer = String::new();
        println!("Read input: ");
        stdin().read_line(&mut buffer);
        buffer = buffer.trim().to_string();
        buffer.parse::<i64>().unwrap()
    }

    pub fn run(mut self) -> Result<i64, &'static str> {
        loop {
            let instruction = self.get_next_instruction();
            match instruction {
                Instruction::Add(p1, p2, p3) => {
                    self.memory[p3] = self.fetch_data(p1) + self.fetch_data(p2);
                    self.pointer += 4;
                    //println!("[{}] = {} + {}", val3, val1, val2)
                }
                Instruction::Multiply(p1, p2, p3) => {
                    self.memory[p3] = self.fetch_data(p1) * self.fetch_data(p2);
                    self.pointer += 4;
                    // println!("[{}] = {} * {}", val3, val1, val2)
                }
                Instruction::Input(p1) => {
                    self.memory[p1] = IntcodeComputer::read_input();
                    self.pointer += 2;
                }
                Instruction::Output(p1) => {
                    println!("Ouput: {}", self.memory[p1]);
                    self.pointer += 2;
                }
                Instruction::JumpIfTrue(p1, p2) => {
                    if self.fetch_data(p1) != 0 {
                        self.pointer = self.fetch_data(p2).try_into().unwrap();
                    } else {
                        self.pointer += 3;
                    }
                }
                Instruction::JumpIfFalse(p1, p2) => {
                    if self.fetch_data(p1) == 0 {
                        self.pointer = self.fetch_data(p2).try_into().unwrap();
                    } else {
                        self.pointer += 3;
                    }
                }
                Instruction::LessThan(p1, p2, dest) => {
                    if self.fetch_data(p1) < self.fetch_data(p2) {
                        self.memory[dest] = 1;
                    } else {
                        self.memory[dest] = 0;
                    }
                    self.pointer += 4;
                }
                Instruction::Equals(p1, p2, dest) => {
                    if self.fetch_data(p1) == self.fetch_data(p2) {
                        self.memory[dest] = 1;
                    } else {
                        self.memory[dest] = 0;
                    }
                    self.pointer += 4;
                }
                Instruction::Break => {
                    break;
                }
                _ => panic!("Unknow opcode instruction"),
            }
        }

        Ok(self.memory[0])
    }
}

pub fn star1() {
    let input = to_int_vec(get_input("inputs/day5.txt", ","));
    let mut computer = IntcodeComputer {
        memory: input,
        pointer: 0,
    };
    computer.run();
}

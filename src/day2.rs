use crate::common::{get_input, to_int_vec};
use std::convert::TryInto;

pub struct IntcodeComputer {
    memory: Vec<i64>,
}

impl IntcodeComputer {
    pub fn run(mut self, arg1: i64, arg2: i64) -> Result<i64, &'static str> {
        let mut pointer = 0;
        self.memory[1] = arg1;
        self.memory[2] = arg2;

        loop {
            let opcode = self.memory[pointer];
            if opcode == 99 {
                break;
            }
            let val1_position: usize = self.memory[pointer + 1].try_into().unwrap();
            let val2_position: usize = self.memory[pointer + 2].try_into().unwrap();
            let val1 = self.memory[val1_position];
            let val2 = self.memory[val2_position];
            let val3: usize = self.memory[pointer + 3].try_into().unwrap();
            match opcode {
                1 => {
                    self.memory[val3] = val1 + val2;
                    //println!("[{}] = {} + {}", val3, val1, val2)
                }
                2 => {
                    self.memory[val3] = val1 * val2;
                    // println!("[{}] = {} * {}", val3, val1, val2)
                }
                _ => println!("Unknow opcode {}", opcode),
            }
            pointer += 4;
        }

        Ok(self.memory[0])
    }
}

pub fn star1() {
    let input = to_int_vec(get_input("inputs/day2.txt", ","));
    let computer = IntcodeComputer { memory: input };
    let result = computer.run(12, 2);
    match result {
        Ok(value) => println!("Day 2 Star 1: {}", value),
        Err(error) => println!("Error"),
    }
}

pub fn star2() {
    let input = to_int_vec(get_input("inputs/day2.txt", ","));

    let expected_result = 19690720;
    let mut result_found = false;
    let mut args = (0, 0);
    for arg1 in 1..input.len() {
        for arg2 in 1..input.len() {
            args = (arg1.try_into().unwrap(), arg2.try_into().unwrap());
            print!("Args: {} {} ",arg1,arg2);
            let computer = IntcodeComputer {
                memory: input.clone(),
            };
            let result = computer.run(args.0, args.1);
            match result {
                Ok(value) => {
                    println!("Result: {}",value);
                    if value == expected_result {
                        result_found = true;
                        break;
                    }
                }
                Err(error) => println!(" {}",error),
            }
        }
        if result_found {
            break;
        }
    }
    println!(
        "Day 2 Star 2: 100 * {} + {} = {}",
        args.0,
        args.1,
        args.0 * 100 + args.1
    );
}

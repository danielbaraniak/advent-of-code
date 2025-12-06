use std::result;

use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();

    let registers: Vec<u32> = lines
        .iter()
        .take(3)
        .filter_map(|line| line.split_once(':'))
        .map(|(_, value)| value.trim().parse().unwrap())
        .collect();
    let (a_register, b_register, c_register) = (registers[0], registers[1], registers[2]);

    let mut computer = Computer {
        a_register,
        b_register,
        c_register,
        ..Default::default()
    };

    let program_str = lines.iter().last().unwrap();
    let program: Vec<u8> = program_str
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let output = computer.execute_program(&program);
    Some(output.iter().join(","))
}

pub fn part_two(input: &str) -> Option<u32> {
    let program: Vec<u8> = input
        .lines()
        .last()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    None
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Computer {
    a_register: u32,
    b_register: u32,
    c_register: u32,

    pointer: u32,

    std_out: Vec<u8>,
}

impl Computer {
    fn execute_program(&mut self, program: &[u8]) -> &Vec<u8> {
        while let (Some(optcode), Some(&operand)) = (
            program.get(self.pointer as usize),
            program.get((self.pointer + 1) as usize),
        ) {
            let instruction = match optcode {
                0 => Computer::adv,
                1 => Computer::bxl,
                2 => Computer::bst,
                3 => {
                    self.jnz(operand);
                    continue;
                }
                4 => Computer::bxc,
                5 => Computer::out,
                6 => Computer::bdv,
                7 => Computer::cdv,
                8.. => panic!("Unexpected optcode"),
            };

            instruction(self, operand);
            self.pointer += 2;
        }
        &self.std_out
    }

    fn combo_to_value(&self, operand: u8) -> u32 {
        match operand {
            0..=3 => operand as u32,
            4 => self.a_register,
            5 => self.b_register,
            6 => self.c_register,
            7.. => panic!("Reseved operand"),
        }
    }

    fn adv(&mut self, operand: u8) {
        self.a_register /= 2_u32.checked_pow(self.combo_to_value(operand)).unwrap();
        // self.a_register /= 1 << self.combo_to_value(operand);
    }

    fn bxl(&mut self, operand: u8) {
        self.b_register ^= operand as u32;
    }

    fn bst(&mut self, operand: u8) {
        self.b_register = self.combo_to_value(operand) & 0b111;
    }

    fn jnz(&mut self, operand: u8) {
        if self.a_register != 0 {
            self.pointer = operand as u32;
        } else {
            self.pointer += 2;
        }
    }

    fn bxc(&mut self, _: u8) {
        self.b_register ^= self.c_register;
    }

    fn out(&mut self, operand: u8) {
        let result = self.combo_to_value(operand) & 0b111;
        self.std_out.push(result as u8);
    }

    fn bdv(&mut self, operand: u8) {
        let numerator = self.a_register;
        let denominator = 2_u32.checked_pow(self.combo_to_value(operand)).unwrap();
        self.b_register = numerator / denominator;
    }

    fn cdv(&mut self, operand: u8) {
        let numerator = self.a_register;
        let denominator = 2_u32.checked_pow(self.combo_to_value(operand)).unwrap();
        self.c_register = numerator / denominator;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("Program: 0,3,5,4,3,0");
        assert_eq!(result, Some(117440));
    }
}

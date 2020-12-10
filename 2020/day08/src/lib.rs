use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, one_of},
    combinator::map,
    sequence::tuple,
    IResult,
};

#[derive(Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let line = tuple((alpha1, tag(" "), one_of("+-"), digit1));
    map(line, |(instr, _, sign, num): (&str, &str, char, &str)| {
        let num = num.parse::<i32>().unwrap();
        let num = if sign == '-' { -num } else { num };
        match instr {
            "nop" => Instruction::Nop(num),
            "acc" => Instruction::Acc(num),
            "jmp" => Instruction::Jmp(num),
            _ => panic!("Unknown instruction {}", instr),
        }
    })(input)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn stop_on_loop(file: &str) -> i32 {
    let mut visited = HashSet::new();
    let instructions: HashMap<usize, Instruction> = read_lines(file)
        .unwrap()
        .map(Result::unwrap)
        .map(|l| parse_instruction(&l).unwrap().1)
        .enumerate()
        .collect();
    let mut accumulator = 0;
    let mut index = 0;
    while !visited.contains(&index) {
        visited.insert(index);
        match instructions.get(&index).unwrap() {
            Instruction::Nop(_) => index += 1,
            Instruction::Acc(i) => {
                accumulator += i;
                index += 1
            }
            Instruction::Jmp(i) => {
                index = if i.is_negative() {
                    index.checked_sub((-i) as usize).unwrap()
                } else {
                    index.checked_add(*i as usize).unwrap()
                }
            }
        }
    }
    accumulator
}

fn halts(instructions: &HashMap<usize, Instruction>) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut accumulator = 0;
    let mut index = 0;
    while !visited.contains(&index) {
        visited.insert(index);
        match instructions.get(&index).unwrap() {
            Instruction::Nop(_) => index += 1,
            Instruction::Acc(i) => {
                accumulator += i;
                index += 1
            }
            Instruction::Jmp(i) => {
                index = if i.is_negative() {
                    index.checked_sub((-i) as usize).unwrap()
                } else {
                    index.checked_add(*i as usize).unwrap()
                }
            }
        }
        if index == instructions.len() {
            return Some(accumulator);
        }
    }
    None
}

pub fn try_until_halts(file: &str) -> i32 {
    let instructions: HashMap<usize, Instruction> = read_lines(file)
        .unwrap()
        .map(Result::unwrap)
        .map(|l| parse_instruction(&l).unwrap().1)
        .enumerate()
        .collect();

    for i in 0..instructions.len() {
        let mut changed_instructions = instructions.clone();
        changed_instructions
            .entry(i)
            .and_modify(|instr| match instr {
                Instruction::Nop(i) => *instr = Instruction::Jmp(*i),
                Instruction::Jmp(i) => *instr = Instruction::Nop(*i),
                _ => (),
            });
        if let Some(acc) = halts(&changed_instructions) {
            return acc;
        }
    }
    panic!("Never halts")
}

#[cfg(test)]
mod tests {
    use crate::{stop_on_loop, try_until_halts};

    #[test]
    fn part_2_example() {
        assert_eq!(8, try_until_halts("example.txt"));
    }
    #[test]
    fn part_2() {
        assert_eq!(8, try_until_halts("input.txt"));
    }

    #[test]
    fn part_1_example() {
        assert_eq!(5, stop_on_loop("example.txt"));
    }
    #[test]
    fn part_1() {
        assert_eq!(2025, stop_on_loop("input.txt"));
    }
}

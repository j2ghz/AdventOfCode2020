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

enum Instruction {
    Nop,
    Acc(i32),
    Jmp(i32),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let line = tuple((alpha1, tag(" "), one_of("+-"), digit1));
    map(line, |(instr, _, sign, num): (&str, &str, char, &str)| {
        let num = num.parse::<i32>().unwrap();
        let num = if sign == '-' { -num } else { num };
        match instr {
            "nop" => Instruction::Nop,
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

fn stop_on_loop(file: &str) -> i32 {
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
            Instruction::Nop => index += 1,
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

#[cfg(test)]
mod tests {
    use crate::stop_on_loop;

    #[test]
    fn part_1_example() {
        assert_eq!(5, stop_on_loop("example.txt"));
    }
    #[test]
    fn part_1() {
        assert_eq!(2025, stop_on_loop("input.txt"));
    }

}

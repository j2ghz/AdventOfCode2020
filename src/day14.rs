use bit_vec::BitVec;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, not_line_ending},
    combinator::{all_consuming, map, map_res},
    sequence::{preceded, tuple},
    IResult,
};
use std::collections::HashMap;

pub enum Instr {
    Mask(String),
    Mem(u16, BitVec),
}

#[derive(Default, Debug)]
struct Memory {
    mask_set_0: BitVec,
    mask_set_1: BitVec,
    mem: HashMap<u16, BitVec>,
}

impl Memory {
    fn process(&mut self, instr: &Instr) {
        match instr {
            Instr::Mask(s) => {
                self.mask_set_0 = s.chars().map(|c| c != '0').collect();
                self.mask_set_1 = s.chars().map(|c| c == '1').collect();
            }
            Instr::Mem(addr, val) => {
                let mut val = val.clone();
                apply_mask(&mut val, &self.mask_set_0, &self.mask_set_1);
                if val.none() {
                    self.mem.remove(addr);
                } else {
                    self.mem.insert(*addr, val);
                }
            }
        }
    }

    fn count_values(&self) -> u64 {
        self.mem
            .values()
            .map(|x| {
                let string = x
                    .iter()
                    .map(|x| if x { '1' } else { '0' })
                    .collect::<String>();
                u64::from_str_radix(&string, 2).unwrap()
            })
            .sum()
    }
}

fn apply_mask(curr: &mut BitVec, mask_set_0: &BitVec, mask_set_1: &BitVec) {
    curr.and(mask_set_0);
    curr.or(mask_set_1);
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Instr> {
    fn mask(input: &str) -> IResult<&str, Instr> {
        map(preceded(tag("mask = "), not_line_ending), |s: &str| {
            Instr::Mask(s.to_string())
        })(input)
    }

    fn parse_prim<T: std::str::FromStr>(input: &str) -> IResult<&str, T> {
        map_res(digit1, |i: &str| i.parse::<T>())(input)
    }

    fn mem(input: &str) -> IResult<&str, Instr> {
        map(
            tuple((tag("mem["), parse_prim, tag("] = "), parse_prim::<u64>)),
            |(_, addr, _, val)| {
                Instr::Mem(
                    addr,
                    format!("{:0>36}", format!("{:b}", val))
                        .chars()
                        .map(|c| match c {
                            '0' => false,
                            '1' => true,
                            _ => unreachable!(),
                        })
                        .collect(),
                )
            },
        )(input)
    }

    fn parse(input: &str) -> IResult<&str, Instr> {
        all_consuming(alt((mask, mem)))(input)
    }

    input.lines().map(|l| parse(l).unwrap().1).collect_vec()
}

#[aoc(day14, part1)]
pub fn part1(input: &[Instr]) -> u64 {
    let mut mem = Memory::default();
    for instr in input {
        mem.process(instr);
    }
    mem.count_values()
}

#[aoc(day14, part2)]
pub fn part2(input: &[Instr]) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    #[test]
    fn part1() {
        let input = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(165, super::part1(&super::input_generator(&input)));
    }
    #[test]
    fn part1_example() {
        let input = read_to_string("input/2020/day14.txt").expect("input file missing");
        assert_eq!(
            14553106347726,
            super::part1(&super::input_generator(&input))
        );
    }

    #[test]
    fn part2() {
        let input = read_to_string("input/2020/day14.txt").expect("input file missing");
        assert_eq!(0, super::part2(&super::input_generator(&input)).unwrap());
    }
}

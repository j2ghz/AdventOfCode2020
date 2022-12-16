use std::cmp::Reverse;

use color_eyre::Result;
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<u32>().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|g| g.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|g| g.iter().sum::<u32>())
        .sorted_by_key(|x| Reverse(*x))
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    #[test]
    fn part1() {
        let input = read_to_string("input/2020/day12.txt").expect("input file missing");
        assert_eq!(
            0,
            super::part1(&super::input_generator(&input).unwrap()).unwrap()
        );
    }

    #[test]
    fn part2() {
        let input = read_to_string("input/2020/day12.txt").expect("input file missing");
        assert_eq!(
            0,
            super::part2(&super::input_generator(&input).unwrap()).unwrap()
        );
    }
}

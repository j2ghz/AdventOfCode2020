use std::collections::HashMap;

use itertools::Itertools;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<u8> {}

#[aoc(day11, part1)]
pub fn part1(input: &[u8]) -> u64 {}

#[aoc(day11, part2)]
pub fn part2(input: &[u8]) -> u128 {
    let all_inputs: Vec<u8> = [0].iter().chain(input.iter()).copied().collect_vec();
    combinations(all_inputs[0], &all_inputs[1..], &mut HashMap::new())
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_example() {
        assert_eq!(35, solve_part1(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]));
    }
}

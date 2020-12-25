use anyhow::Context;
use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> anyhow::Result<Vec<u8>> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<u8>, std::num::ParseIntError>>()
        .context("reading input file")
}

#[aoc(day15, part1)]
pub fn part1(input: &[u8]) -> u32 {
    play_until(input, 2020)
}

pub fn play_until(input: &[u8], stop_at: u32) -> u32 {
    let mut nums: HashMap<u32, u32> = HashMap::new();
    let mut round = 0;
    let mut last_num: u32 = 0;
    for num in input.iter() {
        round += 1;
        nums.insert(*num as u32, round);
        last_num = *num as u32;
    }
    loop {
        round += 1;

        let num_to_say = match nums.get(&last_num) {
            Some(&last) => round - 1 - last,
            None => 0,
        };

        nums.insert(last_num, round - 1);

        // println!(
        //     "{} of {}: old: {} new: {} nums: {:?}",
        //     round, stop_at, last_num, num_to_say, nums
        // );

        last_num = num_to_say;
        if round >= stop_at {
            return last_num;
        }
    }
}

#[aoc(day15, part2, default)]
pub fn part2(input: &[u8]) -> u32 {
    play_until(input, 30000000)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    #[test]
    fn part1() {
        let input = read_to_string("input/2020/day15.txt").expect("input file missing");
        assert_eq!(1522, super::part1(&super::input_generator(&input).unwrap()));
    }

    #[test]
    fn part1_examples() {
        assert_eq!(0, super::play_until(&[0, 3, 6], 4));
        assert_eq!(3, super::play_until(&[0, 3, 6], 5));
        assert_eq!(3, super::play_until(&[0, 3, 6], 6));
        assert_eq!(1, super::play_until(&[0, 3, 6], 7));
        assert_eq!(0, super::play_until(&[0, 3, 6], 8));
        assert_eq!(4, super::play_until(&[0, 3, 6], 9));
        assert_eq!(0, super::play_until(&[0, 3, 6], 10));

        assert_eq!(1, super::part1(&[1, 3, 2]));
        assert_eq!(10, super::part1(&[2, 1, 3]));
        assert_eq!(27, super::part1(&[1, 2, 3]));
        assert_eq!(78, super::part1(&[2, 3, 1]));
        assert_eq!(438, super::part1(&[3, 2, 1]));
        assert_eq!(1836, super::part1(&[3, 1, 2]));
    }

    #[test]
    fn part2() {
        let input = read_to_string("input/2020/day15.txt").expect("input file missing");
        assert_eq!(
            18234,
            super::part2(&super::input_generator(&input).unwrap())
        );
    }
}

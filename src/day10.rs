use std::collections::HashMap;

use itertools::Itertools;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .sorted()
        .collect_vec()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[u8]) -> u64 {
    let (d1, d3) = input
        .iter()
        .sorted()
        .tuple_windows()
        .fold((1, 1), |(d1, d3), (p, n)| match n - p {
            1 => (d1 + 1, d3),
            3 => (d1, d3 + 1),
            _ => (d1, d3),
        });
    d1 * d3
}

#[aoc(day10, part2)]
pub fn part2(input: &[u8]) -> u128 {
    let all_inputs: Vec<u8> = [0].iter().chain(input.iter()).copied().collect_vec();
    combinations(all_inputs[0], &all_inputs[1..], &mut HashMap::new())
}

fn combinations(current: u8, input: &[u8], mut cache: &mut HashMap<Vec<u8>, u128>) -> u128 {
    if input.is_empty() {
        return 1;
    }

    let mut subcombinations: u128 = 0;
    for next in input.iter().take_while(|x| current + 3 >= **x) {
        let rest = &input
            .iter()
            .skip_while(|x| *next >= **x)
            .copied()
            .collect_vec();
        let cache_key = [*next].iter().chain(rest.iter()).copied().collect_vec();
        subcombinations += if let Some(val) = cache.get(&cache_key) {
            *val
        } else {
            let val = combinations(*next, rest, &mut cache);
            cache.insert(cache_key, val);
            val
        }
    }
    subcombinations
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use itertools::Itertools;

    use super::{input_generator, part2, solve_part1};

    #[test]
    fn part1_example() {
        assert_eq!(35, solve_part1(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]));
    }
    #[test]
    fn part2_example() {
        let input: Vec<_> = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
            .iter()
            .sorted()
            .copied()
            .collect();
        assert_eq!(8, part2(&input));
    }

    #[test]
    fn part2_basics() {
        assert_eq!(1, part2(&[1]));
        assert_eq!(2, part2(&[1, 2]));
    }

    #[test]
    fn part2_example2() {
        let input: Vec<_> = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]
        .iter()
        .sorted()
        .copied()
        .collect();
        assert_eq!(19208, part2(&input));
    }

    #[test]
    fn part1() {
        let input = read_to_string("input/2020/day10.txt").unwrap();
        assert_eq!(2590, solve_part1(&input_generator(&input)));
    }
    #[test]
    fn part2_input() {
        let input = read_to_string("input/2020/day10.txt").unwrap();
        assert_eq!(226775649501184, part2(&input_generator(&input)));
    }

    #[test]
    fn generator() {
        let input = read_to_string("input/2020/day10.txt").unwrap();
        for (p, n) in input_generator(&input).into_iter().tuple_windows() {
            assert!(p <= n, "Not sorted: {} > {}", p, n);
        }
    }

    #[test]
    fn part1_example2() {
        assert_eq!(
            220,
            solve_part1(&[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ])
        );
    }
}

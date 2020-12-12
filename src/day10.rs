use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u128> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .sorted()
        .collect_vec()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[u128]) -> u128 {
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
fn part2(input: &[u128]) -> u128 {
    let max = input.iter().max().unwrap() + 3;
    let all_inputs: Vec<u128> = [0]
        .iter()
        .chain(input.iter())
        .chain([max].iter())
        .copied()
        .collect_vec();
    combinations(/*&[],*/ &all_inputs, &mut HashMap::new())
}

fn combinations(
    /*prev: &[u128],*/ input: &[u128],
    mut cache: &mut HashMap<Vec<u128>, u128>,
) -> u128 {
    let current = input[0];
    let mut subcombinations: u128 = 0;
    if input.len() == 1 {
        /*println!("{:?} {:?}", prev, input);*/
        return 1;
    }
    for (idx, next) in input.iter().enumerate().skip(1) {
        if next - current > 3 {
            break;
        }
        subcombinations += if cache.contains_key(input) {
            cache[input]
        } else {
            let result = combinations(
                /*&prev.iter().chain([*next].iter()).copied().collect_vec(),*/
                &input[idx..],
                &mut cache,
            );
            cache.insert(input.to_vec(), result);
            result
        }
    }

    subcombinations
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use itertools::Itertools;

    use super::{combinations, input_generator, part2, solve_part1};

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

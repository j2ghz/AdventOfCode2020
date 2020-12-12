use itertools::Itertools;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<u32>>()
}
#[aoc(day10, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let (d1, d3) = input
        .iter()
        .sorted()
        .tuple_windows()
        .fold((0, 0), |(d1, d3), (p, n)| match n - p {
            1 => (d1 + 1, d3),
            3 => (d1, d3 + 1),
            _ => (d1, d3),
        });
    (d1 + 1) * (d3 + 1)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::{input_generator, solve_part1};

    #[test]
    fn part1_example() {
        assert_eq!(35, solve_part1(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]));
    }

    #[test]
    fn part1() {
        let input = read_to_string("input/2020/day10.txt").unwrap();
        assert_eq!(2590, solve_part1(&input_generator(&input)));
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

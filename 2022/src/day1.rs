use color_eyre::Result;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<u64>> {
    todo!()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u64]) -> Result<usize> {
    todo!()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u64]) -> Result<usize> {
    todo!()
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

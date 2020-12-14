#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> anyhow::Result<Vec<u64>> {
    todo!()
}

#[aoc(day12, part1)]
pub fn part1(input: &[u64]) -> anyhow::Result<usize> {
    todo!()
}

#[aoc(day12, part2)]
pub fn part2(input: &[u64]) -> anyhow::Result<usize> {
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

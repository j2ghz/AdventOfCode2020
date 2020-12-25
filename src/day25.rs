use std::num::ParseIntError;

use anyhow::Result;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> anyhow::Result<(u64, u64)> {
    let nums = input
        .lines()
        .map(|l| l.parse())
        .collect::<std::result::Result<Vec<u64>, ParseIntError>>()?;
    assert_eq!(2, nums.len());
    Ok((nums[0], nums[1]))
}

fn transform_until(subject_number: u64, expected: u64) -> u64 {
    let mut val = 1;
    let mut loop_size = 0;
    loop {
        val *= subject_number;
        val %= 20201227;
        loop_size += 1;
        if val == expected {
            return loop_size;
        }
    }
}

fn transform(subject_number: u64, loops: u64) -> u64 {
    let mut val = 1;
    for _ in 0..loops {
        val *= subject_number;
        val %= 20201227;
    }
    val
}

#[aoc(day25, part1)]
pub fn part1(&(a, b): &(u64, u64)) -> u64 {
    let a_loop_size = transform_until(7, a);
    let b_loop_size = transform_until(7, b);

    let a_enc = transform(a, b_loop_size);
    let b_enc = transform(b, a_loop_size);
    assert_eq!(a_enc, b_enc);
    a_enc
}

// #[aoc(day25, part2)]
pub fn part2(input: &(u64, u64)) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    #[test]
    fn part1() {
        let input = read_to_string("input/2020/day25.txt").expect("input file missing");
        assert_eq!(
            7032853,
            super::part1(&super::input_generator(&input).unwrap())
        );
    }

    #[test]
    #[ignore]
    fn part2() {
        let input = read_to_string("input/2020/day25.txt").expect("input file missing");
        assert_eq!(
            0,
            super::part2(&super::input_generator(&input).unwrap()).unwrap()
        );
    }
}

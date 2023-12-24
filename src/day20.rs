use std::collections::HashMap;

use bit_vec::BitVec;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    multi::{count, separated_list1},
    sequence::{separated_pair, tuple},
    Finish, IResult,
};
use nom::{
    character::complete::{char, newline},
    sequence::terminated,
};

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> anyhow::Result<HashMap<u16, BitVec>> {
    fn parse_num<T: std::str::FromStr>(input: &str) -> IResult<&str, T> {
        map_res(digit1, |r: &str| r.parse::<T>())(input)
    }
    fn parse_heading(input: &str) -> IResult<&str, u16> {
        map(
            tuple((tag("Tile "), parse_num::<u16>, tag(":"))),
            |(_, id, _)| id,
        )(input)
    }
    fn parse_char(input: &str) -> IResult<&str, bool> {
        alt((map(char('.'), |c| false), map(char('#'), |c| true)))(input)
    }
    fn parse_line(input: &str) -> IResult<&str, BitVec> {
        map(count(parse_char, 10), |r| r.into_iter().collect::<BitVec>())(input)
    }
    fn parse_lines(input: &str) -> IResult<&str, BitVec> {
        map(count(terminated(parse_line, newline), 10), |lines| {
            lines.into_iter().fold(BitVec::new(), |mut acc, x| {
                acc.extend(x);
                acc
            })
        })(input)
    }
    fn parse_piece(input: &str) -> IResult<&str, (u16, BitVec)> {
        map(
            separated_pair(parse_heading, newline, parse_lines),
            |(heading, lines)| (heading, lines),
        )(input)
    }
    fn parse_pieces(input: &str) -> IResult<&str, HashMap<u16, BitVec>> {
        map(separated_list1(newline, parse_piece), |r| {
            r.into_iter().collect()
        })(input)
    }
    parse_pieces(input)
        .finish()
        .map(|(_rest, result)| result)
        .map_err(|e| anyhow::anyhow!("Parsing failed: {}", e))
}

// #[aoc(day20, part1)]
pub fn part1(input: &HashMap<u16, BitVec>) -> anyhow::Result<usize> {
    dbg!(input);
    todo!()
}

// #[aoc(day20, part2)]
pub fn part2(input: &HashMap<u16, BitVec>) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    #[test]
    #[ignore]
    fn part1() {
        let input = read_to_string("input/2020/day20.txt").expect("input file missing");
        assert_eq!(
            0,
            super::part1(&super::input_generator(&input).unwrap()).unwrap()
        );
    }

    #[test]
    #[ignore]
    fn part2() {
        let input = read_to_string("input/2020/day20.txt").expect("input file missing");
        assert_eq!(
            0,
            super::part2(&super::input_generator(&input).unwrap()).unwrap()
        );
    }
}

use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, anychar, digit1, newline, none_of},
    combinator::{all_consuming, map, map_res},
    error::VerboseError,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    Finish,
};
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Field {
    name: String,
    rules: Vec<RangeInclusive<u32>>,
}
#[derive(Debug)]
struct Ticket(Vec<u32>);
#[derive(Debug)]
pub struct Scan {
    fields: Vec<Field>,
    my_ticket: Ticket,
    tickets: Vec<Ticket>,
}

type IResult<I, O> = nom::IResult<I, O, nom::error::VerboseError<I>>;

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> anyhow::Result<Scan> {
    fn parse_u32(input: &str) -> IResult<&str, u32> {
        map_res(digit1, |r: &str| r.parse::<u32>())(input)
    }
    fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
        map(separated_pair(parse_u32, tag("-"), parse_u32), |(a, b)| {
            a..=b
        })(input)
    }
    fn parse_rules(input: &str) -> IResult<&str, Vec<RangeInclusive<u32>>> {
        map(
            separated_pair(parse_range, tag(" or "), parse_range),
            |(a, b)| vec![a, b],
        )(input)
    }
    fn parse_field(input: &str) -> IResult<&str, Field> {
        map(
            separated_pair(is_not(":"), tag(": "), parse_rules),
            |(field, rules)| Field {
                name: field.to_owned(),
                rules,
            },
        )(input)
    }
    fn parse_field_list(input: &str) -> IResult<&str, Vec<Field>> {
        separated_list1(newline, parse_field)(input)
    }
    fn parse_ticket(input: &str) -> IResult<&str, Ticket> {
        map(separated_list1(tag(","), parse_u32), Ticket)(input)
    }
    fn parse_my_tickets(input: &str) -> IResult<&str, Ticket> {
        preceded(tuple((tag("your ticket:"), newline)), parse_ticket)(input) //add newlines
    }
    fn parse_tickets(input: &str) -> IResult<&str, Vec<Ticket>> {
        preceded(
            tuple((tag("nearby tickets:"), newline)),
            separated_list1(newline, parse_ticket),
        )(input)
    }

    map(
        all_consuming(tuple((
            parse_field_list,
            newline,
            newline,
            parse_my_tickets,
            newline,
            newline,
            parse_tickets,
            newline,
        ))),
        |(f, _, _, m, _, _, t, _)| Scan {
            fields: f,
            my_ticket: m,
            tickets: t,
        },
    )(input)
    .finish()
    .map(|(rest, parsed)| {
        assert_eq!("", rest);
        parsed
    })
    .map_err(|e: VerboseError<&str>| {
        dbg!(&e);
        anyhow!("{}", e)
    })
}

#[aoc(day16, part1)]
pub fn part1(input: &Scan) -> u32 {
    let rules = input
        .fields
        .iter()
        .flat_map(|f| f.rules.iter())
        .collect_vec();
    dbg!(&rules);
    dbg!(&input.tickets);
    let invalid = input
        .tickets
        .iter()
        .flat_map(|t| t.0.iter())
        .filter(|n| !rules.iter().any(|range| range.contains(n)))
        .collect_vec();
    dbg!(&invalid);
    invalid.iter().copied().sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &Scan) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    #[test]
    fn parse_example() {
        let input = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
        assert_eq!(71, super::part1(&super::input_generator(&input).unwrap()));
    }

    #[test]
    fn part1() {
        let input = read_to_string("input/2020/day16.txt").expect("input file missing");
        assert_eq!(0, super::part1(&super::input_generator(&input).unwrap()));
    }

    // #[test]
    fn part2() {
        let input = read_to_string("input/2020/day16.txt").expect("input file missing");
        assert_eq!(
            0,
            super::part2(&super::input_generator(&input).unwrap()).unwrap()
        );
    }
}

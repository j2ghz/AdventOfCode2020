use anyhow::Result;
use anyhow::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, newline},
    combinator::{all_consuming, eof, map, map_res},
    error::VerboseError,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    Finish,
};
use single::Single;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Field {
    name: String,
    rules: Vec<RangeInclusive<u32>>,
}
#[derive(Debug)]
struct Ticket(Vec<u32>);
#[derive(Debug)]
struct FieldValues {
    my_ticket: u32,
    tickets: Vec<u32>,
}
#[derive(Debug)]
pub struct Scan {
    fields: Vec<Field>,
    my_ticket: Ticket,
    tickets: Vec<Ticket>,
}
impl Scan {
    fn get_values_in_fields(&self) -> anyhow::Result<Vec<FieldValues>> {
        self.my_ticket
            .0
            .iter()
            .enumerate()
            .map(|(idx, my_value)| {
                let others_values = self
                    .tickets
                    .iter()
                    .map(|t| -> anyhow::Result<u32> {
                        t.0.get(idx)
                            .copied()
                            .ok_or_else(|| anyhow!("tickets differ in size"))
                    })
                    .collect::<Result<Vec<u32>>>()
                    .with_context(|| format!("gathering values for ticket field {}", idx))?;
                Ok(FieldValues {
                    my_ticket: *my_value,
                    tickets: others_values,
                })
            })
            .collect::<Result<Vec<FieldValues>>>()
            .with_context(|| "transposing tickets with ticket fields")
    }

    fn find_fields_options(&self) -> Result<Vec<(String, u32)>> {
        fn field_values_match_rules(fv: &FieldValues, rules: &[RangeInclusive<u32>]) -> bool {
            fv.tickets
                .iter()
                .all(|val| rules.iter().any(|rule| rule.contains(val)))
        }

        let values = self.get_values_in_fields()?;
        self.fields
            .iter()
            .map(|f| -> Result<(String, u32)> {
                Ok((
                    f.name.clone(),
                    values
                        .iter()
                        .filter(|fv| field_values_match_rules(fv, &f.rules))
                        .map(|fv| fv.my_ticket)
                        .single()
                        .map_err(|e| anyhow!("{}", e))
                        .with_context(|| {
                            format!("None of values matched the rules: {:?}", f.rules)
                        })?,
                ))
            })
            .collect()
    }
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
            alt((preceded(newline, eof), eof)),
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
        anyhow!("Parsing failed: {}", nom::error::convert_error(input, e))
    })
}

#[aoc(day16, part1)]
pub fn part1(input: &Scan) -> u32 {
    let rules = input
        .fields
        .iter()
        .flat_map(|f| f.rules.iter())
        .collect_vec();
    // dbg!(&rules);
    // dbg!(&input.tickets);
    let invalid = input
        .tickets
        .iter()
        .flat_map(|t| t.0.iter())
        .filter(|n| !rules.iter().any(|range| range.contains(n)))
        .collect_vec();
    // dbg!(&invalid);
    invalid.iter().copied().sum()
}

// #[aoc(day16, part2)]
pub fn part2(input: &Scan) -> anyhow::Result<u32> {
    Ok(input
        .find_fields_options()?
        .iter()
        .filter(|(n, _)| n.starts_with("departure"))
        .map(|(_, v)| *v)
        .product())
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
38,6,12";
        assert_eq!(71, super::part1(&super::input_generator(&input).unwrap()));
    }

    #[test]
    fn part1() -> anyhow::Result<()> {
        let input = read_to_string("input/2020/day16.txt")?;
        assert_eq!(21996, super::part1(&super::input_generator(&input)?));
        Ok(())
    }

    // TODO: WIP
    #[test]
    #[ignore]
    fn part2() {
        let input = read_to_string("input/2020/day16.txt").expect("input file missing");
        assert_eq!(
            0,
            super::part2(&super::input_generator(&input).unwrap()).unwrap()
        );
    }
}

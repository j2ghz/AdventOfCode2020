use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use nom::{branch::alt, multi::separated_list1};
use nom::{
    bytes::complete::tag, character::complete::one_of, character::is_digit, combinator::map_res,
    multi::many1,
};
use nom::{bytes::complete::take_while, combinator::map};
use nom::{sequence::*, IResult};
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Bag {
    modifier: String,
    color: String,
}

impl std::fmt::Display for Bag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("'{} {}' bag", &self.modifier, &self.color))
    }
}

fn parse_bag(input: &str) -> IResult<&str, Bag> {
    let space = tag(" ");
    let word = take_while(|c: char| c.is_ascii_alphabetic());
    map(
        tuple((
            word,
            space,
            take_while(|c: char| c.is_ascii_alphabetic()),
            alt((tag(" bags"), tag(" bag"))),
        )),
        |(modifier, _, color, _): (&str, &str, &str, &str)| Bag {
            modifier: modifier.to_string(),
            color: color.to_string(),
        },
    )(input)
}

fn parse_bag_count(input: &str) -> IResult<&str, (u32, Bag)> {
    let space = tag(" ");
    let count = map(
        terminated(nom::character::complete::digit1, space),
        |r: &str| r.parse::<u32>().unwrap(),
    );
    tuple((count, parse_bag))(input)
}

fn parse_antecedent(input: &str) -> IResult<&str, Bag> {
    terminated(parse_bag, tag(" contain "))(input)
}

fn parse_consequent(input: &str) -> IResult<&str, Vec<(u32, Bag)>> {
    let list = separated_list1(tag(", "), parse_bag_count);
    let none = map(tag("no other bags"), |_| vec![]);
    terminated(alt((none, list)), tag("."))(input)
}

fn parse_rule(input: &str) -> IResult<&str, (Bag, Vec<(u32, Bag)>)> {
    tuple((parse_antecedent, parse_consequent))(input)
}

fn get_rules(filename: &str) -> std::vec::Vec<(Bag, std::vec::Vec<(u32, Bag)>)> {
    read_lines(filename)
        .unwrap()
        .into_iter()
        .map(|l| {
            let l = &l.unwrap();
            parse_rule(l).unwrap().1
        })
        .collect::<Vec<_>>()
}

pub fn get_bags_with_gold_in(filename: &str) -> Vec<Bag> {
    let rules = get_rules(filename);
    let rules_lookup = get_rules(filename)
        .into_iter()
        .collect::<HashMap<Bag, Vec<(u32, Bag)>>>();
    rules
        .into_iter()
        .map(|(b, _)| b)
        .filter(|b| can_contain_gold(b, &rules_lookup))
        .collect()
}

fn golden() -> Bag {
    Bag {
        modifier: "shiny".to_string(),
        color: "gold".to_string(),
    }
}

fn can_contain_gold(bag: &Bag, rules: &HashMap<Bag, Vec<(u32, Bag)>>) -> bool {
    if bag == &golden() {
        true
    } else {
        let rule_contains = rules.get(&bag).unwrap();
        for (_, contained_bag) in rule_contains {
            if can_contain_gold(contained_bag, rules) {
                return true;
            }
        }
        false
    }
}

pub fn get_bags_in_gold(filename: &str) -> u32 {
    let rules = get_rules(filename)
        .into_iter()
        .collect::<HashMap<Bag, Vec<(u32, Bag)>>>();
    get_bags_in(&golden(), &rules)
}

fn get_bags_in(bag: &Bag, rules: &HashMap<Bag, Vec<(u32, Bag)>>) -> u32 {
    let inside: u32 = rules
        .get(&bag)
        .unwrap()
        .iter()
        .map(|(count, bag_in)| get_bags_in(bag_in, rules) * count)
        .sum();
    1 + inside
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::{
        get_bags_in_gold, get_bags_with_gold_in, parse_antecedent, parse_bag, parse_consequent,
        parse_rule, read_lines, Bag,
    };

    #[test]
    fn parse_bag_example_short() {
        assert_eq!(
            Bag {
                modifier: "bright".to_string(),
                color: "white".to_string()
            },
            parse_bag("bright white bag").unwrap().1
        );
    }
    #[test]
    fn parse_bag_example_line() {
        let result =
            parse_bag("light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap();
        assert_eq!(
            Bag {
                modifier: "light".to_string(),
                color: "red".to_string()
            },
            result.1
        );
        assert_eq!(
            " contain 1 bright white bag, 2 muted yellow bags.",
            result.0
        );
    }
    #[test]
    fn parse_antecedent_example_line() {
        let result =
            parse_antecedent("light red bags contain 1 bright white bag, 2 muted yellow bags.")
                .unwrap();
        assert_eq!(
            Bag {
                modifier: "light".to_string(),
                color: "red".to_string()
            },
            result.1
        );
        assert_eq!("1 bright white bag, 2 muted yellow bags.", result.0);
    }

    #[test]
    fn parse_consequent_example_line() {
        let result = parse_consequent("1 bright white bag, 2 muted yellow bags.").unwrap();

        assert_eq!("", result.0);
        assert_eq!(
            vec![
                (
                    1,
                    Bag {
                        modifier: "bright".to_string(),
                        color: "white".to_string()
                    }
                ),
                (
                    2,
                    Bag {
                        modifier: "muted".to_string(),
                        color: "yellow".to_string()
                    }
                )
            ],
            result.1
        );
    }

    #[test]
    fn parse_rule_example() {
        for line in read_lines("example.txt").unwrap() {
            let line = &line.unwrap();
            let result = parse_rule(line).unwrap();
            assert_eq!(result.0, "");
        }
    }
    #[test]
    fn get_bags_with_gold_in_example() {
        let count = get_bags_with_gold_in("example.txt").into_iter().count();
        assert_eq!(4, count - 1);
    }
    #[test]
    fn get_bags_with_gold_in_input() {
        let count = get_bags_with_gold_in("input.txt").into_iter().count();
        assert_eq!(252, count - 1);
    }

    #[test]
    fn get_bags_in_gold_example() {
        let count = get_bags_in_gold("example.txt");
        assert_eq!(32, count - 1);
    }
    #[test]
    fn get_bags_in_gold_example2() {
        let count = get_bags_in_gold("example2.txt");
        assert_eq!(126, count - 1);
    }
    #[test]
    fn get_bags_in_gold_input() {
        let count = get_bags_in_gold("input.txt");
        assert_eq!(35487, count - 1);
    }
}

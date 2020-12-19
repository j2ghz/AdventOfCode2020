use core::panic;

use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, error::convert_error,
    multi::*, sequence::*, Finish,
};

enum Op {
    Add,
    Multiply,
}
pub struct Expression {
    initial: Box<SubExp>,
    steps: Vec<(Op, SubExp)>,
}
enum SubExp {
    Number(u64),
    Parentheses(Expression),
}

pub trait Computable {
    fn get_result(&self) -> u64;
}
impl Computable for Expression {
    fn get_result(&self) -> u64 {
        self.steps
            .iter()
            .fold(self.initial.get_result(), |subsum, (op, subexp)| match op {
                Op::Add => subsum + subexp.get_result(),
                Op::Multiply => subsum * subexp.get_result(),
            })
    }
}
impl Computable for SubExp {
    fn get_result(&self) -> u64 {
        match self {
            SubExp::Number(i) => *i,
            SubExp::Parentheses(e) => e.get_result(),
        }
    }
}

type IResult<I, O> = nom::IResult<I, O, nom::error::VerboseError<I>>;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> anyhow::Result<Vec<Expression>> {
    /// `0`
    fn parse_u64(input: &str) -> IResult<&str, u64> {
        map_res(digit1, |r: &str| r.parse::<u64>())(input)
    }
    /// `+`
    fn parse_op(input: &str) -> IResult<&str, Op> {
        map(terminated(one_of("+*"), char(' ')), |r| match r {
            '+' => Op::Add,
            '*' => Op::Multiply,
            _ => panic!("Unknown operation {}", r),
        })(input)
    }
    /// `0`
    /// `(0 + 1)`
    fn parse_subexp(input: &str) -> IResult<&str, SubExp> {
        alt((
            map(parse_u64, SubExp::Number),
            map(
                preceded(char('('), terminated(parse_expression, char(')'))),
                SubExp::Parentheses,
            ),
        ))(input)
    }
    /// `+ 2`
    /// `+ 2 * (3 + 5)`
    fn parse_rest_of_exp(input: &str) -> IResult<&str, Vec<(Op, SubExp)>> {
        separated_list1(char(' '), tuple((parse_op, parse_subexp)))(input)
    }
    fn parse_expression(input: &str) -> IResult<&str, Expression> {
        map(
            tuple((terminated(parse_subexp, char(' ')), parse_rest_of_exp)),
            |(init, rest)| Expression {
                initial: Box::new(init),
                steps: rest,
            },
        )(input)
    }
    fn parse_line(input: &str) -> anyhow::Result<Expression> {
        all_consuming(parse_expression)(input)
            .finish()
            .map(|(_rest, res)| res)
            .map_err(|e| anyhow::anyhow!("Parser error:\n{}\n", convert_error(input, e)))
    }
    input
        .lines()
        .map(parse_line)
        .collect::<anyhow::Result<Vec<Expression>>>()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Expression]) -> u64 {
    input.iter().map(|e| e.get_result()).sum()
}

#[aoc(day18, part2)]
pub fn par21(input: &[Expression]) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use anyhow::Result;

    use super::{input_generator, Computable};

    #[test]
    fn parse_example() -> Result<()> {
        input_generator("(2 * 3) + 1")?;
        input_generator("2 * 3 + 20")?;
        input_generator("2 * 3 + (4 * 5)")?;
        input_generator("5 + (8 * 3 + 9 + 3 * 4 * 3)")?;
        input_generator("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")?;
        input_generator("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")?;
        Ok(())
    }
    #[test]
    fn part1_example() -> Result<()> {
        assert_eq!(6, input_generator("2 * 3")?[0].get_result());
        assert_eq!(26, input_generator("2 * 3 + (4 * 5)")?[0].get_result());
        assert_eq!(
            437,
            input_generator("5 + (8 * 3 + 9 + 3 * 4 * 3)")?[0].get_result()
        );
        assert_eq!(
            12240,
            input_generator("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")?[0].get_result()
        );
        assert_eq!(
            13632,
            input_generator("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")?[0].get_result()
        );
        Ok(())
    }

    #[test]
    fn part1() {
        let input = read_to_string("input/2020/day18.txt").expect("input file missing");
        assert_eq!(
            12918250417632,
            super::part1(&super::input_generator(&input).unwrap())
        );
    }
}

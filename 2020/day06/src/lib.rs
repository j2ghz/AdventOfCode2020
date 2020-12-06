use std::{collections::HashSet, fs::read_to_string};

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub fn sum_yes_answers(filename: &str) -> usize {
    read_to_string(filename)
        .unwrap()
        .split(LINE_ENDING.repeat(2).as_str())
        .map(unique_answer_count)
        .sum::<usize>()
}

fn unique_answer_count(answers: &str) -> usize {
    answers
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>()
        .len()
}

fn all_yes_answer_count(answers: &str) -> usize {
    answers
        .lines()
        .fold(None, |acc, c| -> Option<HashSet<char>> {
            let answers = c.chars().collect::<HashSet<_>>();
            Some(match acc {
                None => answers,
                Some(h) => answers.intersection(&h).cloned().collect::<HashSet<_>>(),
            })
        })
        .unwrap()
        .len()
}

pub fn sum_all_yes_answers(filename: &str) -> usize {
    read_to_string(filename)
        .unwrap()
        .split(LINE_ENDING.repeat(2).as_str())
        .map(all_yes_answer_count)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::{sum_all_yes_answers, sum_yes_answers};

    #[test]
    fn part1() {
        assert_eq!(6291, sum_yes_answers("input.txt"));
    }

    #[test]
    fn part2_example() {
        assert_eq!(6, sum_all_yes_answers("example.txt"));
    }

    #[test]
    fn part2() {
        assert_eq!(3052, sum_all_yes_answers("input.txt"));
    }
}

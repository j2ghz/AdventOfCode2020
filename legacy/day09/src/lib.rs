use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn get_unique_pair_sums(list: &[u64]) -> HashSet<u64> {
    let mut sums = HashSet::new();
    for x in list {
        for y in list {
            if x != y {
                sums.insert(x + y);
            }
        }
    }
    //println!("Sums: {:?}", sums);
    sums
}

fn check_number(previous: &[u64], next: u64) -> Option<u64> {
    //println!("Checking {} with {:?}", next, previous);
    if !get_unique_pair_sums(previous).contains(&next) {
        Some(next)
    } else {
        None
    }
}

pub fn get_first_invalid(preamble_size: usize, file: &str) -> u64 {
    let lines = read_lines(file)
        .unwrap()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    for slice in lines.windows(preamble_size + 1) {
        if let Some(n) = check_number(&slice[..preamble_size], slice[preamble_size]) {
            return n;
        }
    }
    panic!("All numbers good")
}

pub fn part2(preamble_size: usize, file: &str) -> u64 {
    let lines = read_lines(file)
        .unwrap()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let invalid = lines
        .windows(preamble_size + 1)
        .find_map(|slice| check_number(&slice[..preamble_size], slice[preamble_size]))
        .unwrap();
    println!("Looking for sums equal to {}", invalid);

    'index: for index in 0..lines.len() {
        'end: for end in index + 1..lines.len() {
            let numbers = lines[index..end].iter();
            let sum: u64 = numbers.clone().copied().sum();
            match sum.cmp(&invalid) {
                std::cmp::Ordering::Less => {
                    continue 'end;
                }
                std::cmp::Ordering::Greater => {
                    continue 'index;
                }
                std::cmp::Ordering::Equal => {
                    return numbers.clone().min().unwrap() + numbers.clone().max().unwrap()
                }
            }
        }
    }

    panic!("Not found")
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
    use crate::{get_first_invalid, part2};

    #[test]
    fn part_1_example() {
        assert_eq!(127, get_first_invalid(5, "example.txt"));
    }
    #[test]
    fn part_1() {
        assert_eq!(756008079, get_first_invalid(25, "input.txt"));
    }
    #[test]
    fn part_2_example() {
        assert_eq!(62, part2(5, "example.txt"));
    }
    #[test]
    fn part_2_input() {
        assert_eq!(93727241, part2(25, "input.txt"));
    }
}

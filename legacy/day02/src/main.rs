use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use regex::Regex;

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let lines = read_lines("input.txt").unwrap();
    let mut all_count = 0;
    let mut all_count2 = 0;
    for line in lines {
        let line = line.unwrap();
        let c = re.captures(&line).unwrap();
        let min: usize = c.get(1).unwrap().as_str().parse().unwrap();
        let max: usize = c.get(2).unwrap().as_str().parse().unwrap();
        let p = c.get(3).unwrap().as_str().chars().next().unwrap();
        let passw = c.get(4).unwrap().as_str();
        let count = passw.chars().into_iter().filter(|c| *c == p).count();
        if count >= min && count <= max {
            all_count += 1;
        }
        if (passw.chars().nth(min - 1).unwrap() == p) ^ (passw.chars().nth(max - 1).unwrap() == p) {
            all_count2 += 1;
        }
    }
    println!("Valid min-max: {}", all_count);
    println!("Valid pos-xor: {}", all_count2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

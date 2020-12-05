use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn main2() {
    let lines = read_lines("input.txt").unwrap();
    let mut old_numbers = Vec::new();
    for line in lines {
        let number = line.unwrap().parse::<i32>().unwrap();
        for old in old_numbers.iter() {
            if number + old == 2020 {
                println!("{}x{}={}", number, old, number * old);
            }
        }
        old_numbers.insert(0, number);
    }
}

pub fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut old_numbers = Vec::new();
    for line in lines {
        let number = line.unwrap().parse::<i32>().unwrap();
        for old2 in old_numbers.iter() {
            for old in old_numbers.iter() {
                if number + old + old2 == 2020 {
                    println!("{}x{}z{}={}", number, old, old2, number * old * old2);
                }
            }
        }
        old_numbers.insert(0, number);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

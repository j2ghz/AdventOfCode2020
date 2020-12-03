use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines = read_lines("input.txt").unwrap();
    let mut trees = 0;
    let mut pos = 0;
    for line in lines {
        let line = line.unwrap();
        println!("{}", line);
        println!("{}^", " ".repeat(pos));
        if line.chars().nth(pos).unwrap() == '#' {
            trees += 1;
        }
        pos += 3;
        if pos > 30 {
            pos -= 31;
        }
    }
    println!("Trees: {}", trees);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

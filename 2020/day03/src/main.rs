use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {}

fn trees_on_slope(right: usize, down: usize, file: &str) -> usize {
    let lines = read_lines(file).unwrap().step_by(down);
    let mut trees = 0;
    let mut pos = 0;
    for line in lines {
        let line = line.unwrap();
        println!("{}", line);
        println!("{}^", " ".repeat(pos));
        if line.chars().nth(pos).unwrap() == '#' {
            trees += 1;
        }
        pos += right;

        let max_index = line.len() - 1;
        if pos > max_index {
            pos -= line.len();
        }
    }
    trees
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
    use crate::trees_on_slope;

    #[test]
    fn part1_example() {
        assert_eq!(trees_on_slope(3, 1, "example.txt"), 7);
    }
    #[test]
    fn part1() {
        assert_eq!(trees_on_slope(3, 1, "input.txt"), 223);
    }
    #[test]
    fn part2_examples() {
        assert_eq!(trees_on_slope(1, 1, "example.txt"), 2);
        assert_eq!(trees_on_slope(3, 1, "example.txt"), 7);
        assert_eq!(trees_on_slope(5, 1, "example.txt"), 3);
        assert_eq!(trees_on_slope(7, 1, "example.txt"), 4);
        assert_eq!(trees_on_slope(1, 2, "example.txt"), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(trees_on_slope(1, 1, "input.txt"), 58);
        assert_eq!(trees_on_slope(3, 1, "input.txt"), 223);
        assert_eq!(trees_on_slope(5, 1, "input.txt"), 105);
        assert_eq!(trees_on_slope(7, 1, "input.txt"), 74);
        assert_eq!(trees_on_slope(1, 2, "input.txt"), 35);
    }
}

use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn split_interval((lower, upper): (i32, i32), split: char) -> (i32, i32) {
    let middle = (lower + upper + 1) / 2;
    println!(
        "Splitting {} {}..{}  {}..{}",
        split,
        lower,
        middle - 1,
        middle,
        upper
    );
    if lower == upper {
        panic!("Can't split interval of 1");
    }
    match split {
        'F' | 'L' => (lower, middle - 1),
        'B' | 'R' => (middle, upper),
        _ => panic!("Unknown char: {}", split),
    }
}

fn get(pass: &str, max: i32) -> i32 {
    println!("Starting split 0..{}\t{}", max, pass);
    let (lower, upper) = pass.chars().into_iter().fold((0, max), split_interval);
    assert_eq!(lower, upper);
    lower
}

fn get_row_column_id(pass: &str) -> (i32, i32, i32) {
    let row = get(&pass[..7], 127);
    let column = get(&pass[7..], 7);
    let id = row * 8 + column;
    (row, column, id)
}

pub fn highest_seat_id(passfile: &str) -> i32 {
    read_lines(passfile)
        .unwrap()
        .map(|s| get_row_column_id(s.unwrap().as_str()).2)
        .max()
        .unwrap()
}

pub fn get_missing_seat(passfile: &str) -> i32 {
    let seat_ids = read_lines(passfile)
        .unwrap()
        .map(|s| get_row_column_id(s.unwrap().as_str()).2)
        .collect::<HashSet<i32>>();
    println!("Seats: {:?}", seat_ids);
    (0..823)
        .filter(|id| {
            !seat_ids.contains(id) && seat_ids.contains(&(id - 1)) && seat_ids.contains(&(id + 1))
        })
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .to_owned()
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
    use std::assert_eq;

    use crate::{get_missing_seat, get_row_column_id, highest_seat_id};

    #[test]
    fn example() {
        assert_eq!((44, 5, 357), get_row_column_id("FBFBBFFRLR"));
    }

    #[test]
    fn part1() {
        assert_eq!(822, highest_seat_id("input.txt"));
    }
    #[test]
    fn part2() {
        assert_eq!(705, get_missing_seat("input.txt"));
    }
}
